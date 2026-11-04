//! Host-driven story traversal. No world access, rule interpreter, physical
//! clock or cue feedback. Each accepted edge immediately emits a full REPLACE.
pub use grimm_semantic::graph::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoryError {
    InvalidDefinition(String),
    StaleEpoch { expected: u64, actual: u64 },
    UnknownEdge { node_id: String, edge_id: String },
    InvalidContinuation(String),
    EpochExhausted,
}
impl fmt::Display for StoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDefinition(message) => write!(f, "invalid graph: {message}"),
            Self::StaleEpoch { expected, actual } => write!(
                f,
                "stale traversal epoch {expected}; current epoch is {actual}"
            ),
            Self::UnknownEdge { node_id, edge_id } => {
                write!(f, "node '{node_id}' has no edge '{edge_id}'")
            }
            Self::InvalidContinuation(message) => {
                write!(f, "invalid graph continuation: {message}")
            }
            Self::EpochExhausted => write!(f, "graph epoch exhausted"),
        }
    }
}
impl std::error::Error for StoryError {}

#[derive(Debug)]
pub struct GraphEngine {
    graph: StoryGraph,
    nodes: BTreeMap<String, usize>,
    current: usize,
    epoch: u64,
}
impl GraphEngine {
    pub fn new(graph: StoryGraph) -> Result<Self, StoryError> {
        validate(&graph)?;
        let nodes: BTreeMap<_, _> = graph
            .nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (node.id.clone(), i))
            .collect();
        let current = nodes[&graph.start];
        Ok(Self {
            graph,
            nodes,
            current,
            epoch: 0,
        })
    }

    /// Repeated reads return the complete authored declaration, unchanged by
    /// host playback. Every sheet replaces the previous node-entry plan.
    pub fn current(&self) -> CueSheet {
        let node = &self.graph.nodes[self.current];
        CueSheet {
            protocol_version: CUE_SHEET_VERSION,
            node_id: node.id.clone(),
            epoch: self.epoch,
            cues: node.cues.clone(),
            outgoing_edges: self
                .exposed_edges()
                .map(|(id, edge)| (id.to_owned(), edge.clone()))
                .collect(),
        }
    }

    /// Discovery only, never an authorization or condition verdict.
    pub fn exposed_edges(&self) -> impl Iterator<Item = (&str, &Edge)> {
        self.graph.nodes[self.current]
            .edges
            .iter()
            .filter(|(_, edge)| edge.expose)
            .map(|(id, edge)| (id.as_str(), edge))
    }

    /// The host decides whether/when to traverse. Conditions are deliberately
    /// not evaluated. Rejections leave node, epoch and cue sheet unchanged.
    pub fn traverse_edge(
        &mut self,
        edge_id: &str,
        expected_epoch: u64,
    ) -> Result<TraversalResult, StoryError> {
        if expected_epoch != self.epoch {
            return Err(StoryError::StaleEpoch {
                expected: expected_epoch,
                actual: self.epoch,
            });
        }
        let node = &self.graph.nodes[self.current];
        let edge = node
            .edges
            .get(edge_id)
            .ok_or_else(|| StoryError::UnknownEdge {
                node_id: node.id.clone(),
                edge_id: edge_id.into(),
            })?;
        let epoch = self
            .epoch
            .checked_add(1)
            .ok_or(StoryError::EpochExhausted)?;
        let next = self.nodes[&edge.target];
        let path_event = PathEvent {
            sequence: epoch,
            from_node: node.id.clone(),
            from_epoch: self.epoch,
            edge_id: edge_id.into(),
            to_node: edge.target.clone(),
            to_epoch: epoch,
        };
        self.current = next;
        self.epoch = epoch;
        Ok(TraversalResult {
            cue_sheet: self.current(),
            path_event,
        })
    }

    pub fn snapshot(&self) -> StoryContinuation {
        StoryContinuation {
            schema_version: CONTINUATION_VERSION,
            story_id: self.graph.id.clone(),
            story_version: self.graph.version,
            node_id: self.graph.nodes[self.current].id.clone(),
            epoch: self.epoch,
        }
    }

    /// Restore graph position only, never world/media state or prior effects.
    /// Reserve a fresh correlation epoch to reject pre-restore callbacks in this
    /// runtime. The host must also isolate callback queues between instances.
    pub fn restore(&mut self, saved: StoryContinuation) -> Result<CueSheet, StoryError> {
        if saved.schema_version != CONTINUATION_VERSION
            || saved.story_id != self.graph.id
            || saved.story_version != self.graph.version
        {
            return Err(StoryError::InvalidContinuation(
                "schema/story/version mismatch".into(),
            ));
        }
        let current = *self.nodes.get(&saved.node_id).ok_or_else(|| {
            StoryError::InvalidContinuation(format!("unknown node '{}'", saved.node_id))
        })?;
        let epoch = self
            .epoch
            .max(saved.epoch)
            .checked_add(1)
            .ok_or(StoryError::EpochExhausted)?;
        self.current = current;
        self.epoch = epoch;
        Ok(self.current())
    }
}

fn identifier(id: &str) -> bool {
    !id.is_empty()
        && id.is_ascii()
        && id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-' | b'.' | b':'))
}
fn require(ok: bool, message: impl Into<String>) -> Result<(), StoryError> {
    if ok {
        Ok(())
    } else {
        Err(StoryError::InvalidDefinition(message.into()))
    }
}
fn validate(graph: &StoryGraph) -> Result<(), StoryError> {
    require(
        graph.schema_version == GRAPH_SCHEMA_VERSION && graph.version > 0,
        "unsupported schema or zero story version",
    )?;
    require(identifier(&graph.id), "invalid story identifier")?;
    let mut nodes = BTreeSet::new();
    for node in &graph.nodes {
        require(identifier(&node.id), format!("invalid node '{}'", node.id))?;
        require(
            nodes.insert(&node.id),
            format!("duplicate node '{}'", node.id),
        )?;
    }
    require(
        nodes.contains(&graph.start),
        format!("unknown start '{}'", graph.start),
    )?;
    for node in &graph.nodes {
        for (id, edge) in &node.edges {
            require(identifier(id), format!("{}: invalid edge '{id}'", node.id))?;
            require(
                nodes.contains(&edge.target),
                format!("{}.{id}: unknown target '{}'", node.id, edge.target),
            )?;
            if let Some(condition) = &edge.condition {
                require(
                    identifier(&condition.condition_id),
                    format!("{}.{id}: invalid condition ID", node.id),
                )?;
                validate_args(&condition.args)?;
            }
        }
        let mut cues = BTreeSet::new();
        for cue in &node.cues {
            require(
                identifier(&cue.cue_id),
                format!("{}: invalid cue '{}'", node.id, cue.cue_id),
            )?;
            require(
                cues.insert(&cue.cue_id),
                format!("{}: duplicate cue '{}'", node.id, cue.cue_id),
            )?;
            match &cue.instruction {
                Instruction::Presentation {
                    presentation_id,
                    args,
                    ..
                } => {
                    require(
                        identifier(presentation_id),
                        format!("{}: invalid presentation ID", cue.cue_id),
                    )?;
                    validate_args(args)?;
                }
                Instruction::Effect { effect_id, args } => {
                    require(
                        identifier(effect_id),
                        format!("{}: invalid effect ID", cue.cue_id),
                    )?;
                    validate_args(args)?;
                }
                Instruction::TraverseEdge { edge_id } => require(
                    node.edges.contains_key(edge_id),
                    format!(
                        "{}.{}: unknown traversal edge '{edge_id}'",
                        node.id, cue.cue_id
                    ),
                )?,
            }
        }
    }
    Ok(())
}
fn validate_args(args: &Parameters) -> Result<(), StoryError> {
    for name in args.keys() {
        require(identifier(name), format!("invalid parameter name '{name}'"))?;
    }
    Ok(())
}
