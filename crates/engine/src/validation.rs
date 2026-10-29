use crate::{
    ValidationError,
    expressions::{validate_condition, validate_effect},
};
use grimm_semantic::{
    NarrativeContent,
    story::{STORY_SCHEMA_VERSION, Story},
};
use std::collections::{BTreeMap, BTreeSet};

fn nonempty(value: &str, context: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError(format!("{context} must not be blank")));
    }
    Ok(())
}

pub(crate) fn validate(story: &Story) -> Result<(), ValidationError> {
    if story.schema_version != STORY_SCHEMA_VERSION {
        return Err(ValidationError(format!(
            "unsupported story schema version {}; expected {STORY_SCHEMA_VERSION}",
            story.schema_version
        )));
    }
    nonempty(&story.id, "story id")?;
    nonempty(&story.title, "story title")?;
    if story.version == 0 {
        return Err(ValidationError("story version must be positive".into()));
    }
    for name in story.variables.keys() {
        nonempty(name, "variable name")?;
    }
    let mut nodes = BTreeMap::new();
    let mut endings = BTreeSet::new();
    for node in &story.nodes {
        nonempty(&node.id, "node id")?;
        if nodes.insert(node.id.as_str(), node).is_some() {
            return Err(ValidationError(format!("duplicate node '{}'", node.id)));
        }
        if node.content.is_empty() {
            return Err(ValidationError(format!(
                "node '{}': content must not be empty",
                node.id
            )));
        }
        for content in &node.content {
            match content {
                NarrativeContent::Narration { text } => {
                    nonempty(text, &format!("node '{}' narration", node.id))?
                }
                NarrativeContent::Dialogue { speaker, text } => {
                    nonempty(speaker, &format!("node '{}' dialogue speaker", node.id))?;
                    nonempty(text, &format!("node '{}' dialogue text", node.id))?;
                }
            }
        }
        if let Some(ending) = &node.ending {
            nonempty(&ending.id, &format!("node '{}' ending id", node.id))?;
            nonempty(&ending.title, &format!("node '{}' ending title", node.id))?;
            if !endings.insert(&ending.id) {
                return Err(ValidationError(format!("duplicate ending '{}'", ending.id)));
            }
            if !node.choices.is_empty() {
                return Err(ValidationError(format!(
                    "ending node '{}' must not have choices",
                    node.id
                )));
            }
        } else if node.choices.is_empty() {
            return Err(ValidationError(format!(
                "node '{}' has no choices and is not an ending",
                node.id
            )));
        }
        let mut choices = BTreeSet::new();
        for choice in &node.choices {
            let context = format!("node '{}', choice '{}'", node.id, choice.id);
            nonempty(&choice.id, &format!("{context} id"))?;
            nonempty(&choice.label, &format!("{context} label"))?;
            if !choices.insert(&choice.id) {
                return Err(ValidationError(format!("{context}: duplicate choice id")));
            }
            let validate_expressions = || {
                if let Some(condition) = &choice.condition {
                    validate_condition(condition, &story.variables, 0)?;
                }
                for effect in &choice.effects {
                    validate_effect(effect, &story.variables)?;
                }
                Ok::<_, ValidationError>(())
            };
            validate_expressions()
                .map_err(|error| ValidationError(format!("{context}: {error}")))?;
        }
    }
    if !nodes.contains_key(story.start.as_str()) {
        return Err(ValidationError(format!(
            "start node '{}' does not exist",
            story.start
        )));
    }
    for node in &story.nodes {
        for choice in &node.choices {
            if !nodes.contains_key(choice.target.as_str()) {
                return Err(ValidationError(format!(
                    "node '{}', choice '{}': target '{}' does not exist",
                    node.id, choice.id, choice.target
                )));
            }
        }
    }
    // Structural reachability ignores conditions; state-dependent deadlocks are
    // rejected at initialization, restore and before each transition commit.
    let mut reachable = BTreeSet::new();
    let mut pending = vec![story.start.as_str()];
    while let Some(id) = pending.pop() {
        if reachable.insert(id) {
            pending.extend(
                nodes[id]
                    .choices
                    .iter()
                    .map(|choice| choice.target.as_str()),
            );
        }
    }
    for id in nodes.keys() {
        if !reachable.contains(id) {
            return Err(ValidationError(format!(
                "node '{id}' is unreachable from start '{}'",
                story.start
            )));
        }
    }
    let mut can_end: BTreeSet<_> = nodes
        .iter()
        .filter(|(_, node)| node.ending.is_some())
        .map(|(id, _)| *id)
        .collect();
    loop {
        let before = can_end.len();
        for (id, node) in &nodes {
            if node
                .choices
                .iter()
                .any(|choice| can_end.contains(choice.target.as_str()))
            {
                can_end.insert(*id);
            }
        }
        if before == can_end.len() {
            break;
        }
    }
    for id in nodes.keys() {
        if !can_end.contains(id) {
            return Err(ValidationError(format!(
                "node '{id}' has no structural path to an ending"
            )));
        }
    }
    Ok(())
}
