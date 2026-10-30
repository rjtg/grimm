//! M2 authored scene DSL and ID-only full replacement protocol.
//! Rules/checkpoints are host-only; only CueSheet crosses the presentation boundary.
use crate::story::{Value, Variables};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneStory {
    pub schema_version: u32,
    pub id: String,
    pub version: u32,
    pub title: String,
    pub start: String,
    #[serde(default, deserialize_with = "crate::story::unique_variables")]
    pub globals: Variables,
    pub contexts: Vec<Context>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub id: String,
    #[serde(default)]
    pub namespace: Option<String>,
    #[serde(default, deserialize_with = "crate::story::unique_variables")]
    pub locals: Variables,
    #[serde(default)]
    pub terminal: bool,
    pub cues: Vec<AuthoredCue>,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub timeout: Option<Outcome>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub id: String,
    #[serde(default)]
    pub when: Option<Test>,
    #[serde(default)]
    pub duration_ms: u64,
    #[serde(default)]
    pub effects: Vec<Update>,
    /// Ordered IF/ELSE IF/ELSE. Exactly one final unconditional branch required.
    pub outcomes: Vec<Branch>,
    /// Deadline cutoff cases; absence uses the context's authored timeout.
    #[serde(default)]
    pub interrupted_outcomes: Vec<Branch>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Branch {
    #[serde(default)]
    pub when: Option<Test>,
    #[serde(flatten)]
    pub outcome: Outcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    #[serde(default)]
    pub effects: Vec<Update>,
    #[serde(default)]
    pub cues: Vec<AuthoredCue>,
    #[serde(default)]
    pub goto: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum Expr {
    Variable {
        var: String,
    },
    Builtin {
        clock: ClockValue,
    },
    Arithmetic {
        calc: Arithmetic,
        args: [Box<Expr>; 2],
    },
    Literal(Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClockValue {
    SelectionMs,
    TimePassed,
    DurationMs,
    EffectiveMs,
    DeadlineMs,
    Interrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Arithmetic {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Test {
    Eq { left: Expr, right: Expr },
    Ne { left: Expr, right: Expr },
    Lt { left: Expr, right: Expr },
    Le { left: Expr, right: Expr },
    Gt { left: Expr, right: Expr },
    Ge { left: Expr, right: Expr },
    All { tests: Vec<Test> },
    Any { tests: Vec<Test> },
    Not { test: Box<Test> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Update {
    Set { var: String, value: Expr },
    Add { var: String, value: Expr },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionSet {
    Eligible(String),
    Only(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoredCue {
    #[serde(default)]
    pub cue_id: Option<String>,
    #[serde(default)]
    pub at_ms: Option<u64>,
    #[serde(default)]
    pub when: Option<Test>,
    #[serde(flatten)]
    pub instruction: AuthoredInstruction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AuthoredInstruction {
    Narrative {
        narrative_id: String,
    },
    Actions {
        available_actions: ActionSet,
    },
    Timer {
        #[serde(default)]
        start_ms: Option<u64>,
        #[serde(default)]
        extend_ms: Option<u64>,
        #[serde(default)]
        visible: Option<bool>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CueSheet {
    pub revision: u64,
    pub clock: SceneClock,
    /// Complete remaining plan. Always replaces; never append/merge instructions.
    pub cues: Vec<Cue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneClock {
    /// Opaque visit token, never a graph context ID.
    pub epoch: u64,
    pub elapsed_ms: u64,
    pub running: bool,
    pub deadline_ms: Option<u64>,
    pub visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Cue {
    pub cue_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at_ms: Option<u64>,
    /// A logical-time release can also wait for a preceding sequential cue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_cue: Option<String>,
    #[serde(flatten)]
    pub instruction: Instruction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Instruction {
    Narrative {
        narrative_id: String,
    },
    Actions {
        available_actions: Vec<String>,
    },
    Timer {
        start_ms: Option<u64>,
        extend_ms: Option<u64>,
        visible: Option<bool>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneAction {
    pub revision: u64,
    pub action_id: String,
    pub scene_time_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CueEvent {
    Started,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CueInput {
    pub revision: u64,
    pub cue_id: String,
    pub scene_time_ms: u64,
    pub event: CueEvent,
}

/// Canonical replay log only, never renderer playback frames/assets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SceneInput {
    Action(SceneAction),
    Cue(CueInput),
    Observe {
        revision: u64,
        scene_time_ms: u64,
    },
    /// Engine-generated restore boundary; not a player operation.
    RestoreEpoch {
        revision: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneSave {
    pub schema_version: u32,
    pub story_id: String,
    pub story_version: u32,
    pub revision: u64,
    pub history: Vec<SceneInput>,
}

/// Private host diagnostic data. Not part of CueSheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRecord {
    pub action_id: String,
    pub selected_ms: u64,
    pub effective_ms: u64,
    pub logical_end_ms: u64,
    pub interrupted: bool,
}

// Explicit flat decoding keeps unknown-field diagnostics while avoiding serde's
// unsupported deny_unknown_fields + flatten combination.
impl<'de> Deserialize<'de> for Branch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Fields {
            #[serde(default)]
            when: Option<Test>,
            #[serde(default)]
            effects: Vec<Update>,
            #[serde(default)]
            cues: Vec<AuthoredCue>,
            #[serde(default)]
            goto: Option<String>,
        }
        let f = Fields::deserialize(deserializer)?;
        Ok(Self {
            when: f.when,
            outcome: Outcome {
                effects: f.effects,
                cues: f.cues,
                goto: f.goto,
            },
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CueFields<A> {
    #[serde(default)]
    cue_id: Option<String>,
    #[serde(default)]
    at_ms: Option<u64>,
    #[serde(default)]
    after_cue: Option<String>,
    #[serde(default)]
    when: Option<Test>,
    kind: String,
    #[serde(default)]
    narrative_id: Option<String>,
    available_actions: Option<A>,
    #[serde(default)]
    start_ms: Option<u64>,
    #[serde(default)]
    extend_ms: Option<u64>,
    #[serde(default)]
    visible: Option<bool>,
}

impl<A> CueFields<A> {
    fn valid(&self) -> bool {
        let timer = self.start_ms.is_some() || self.extend_ms.is_some() || self.visible.is_some();
        match self.kind.as_str() {
            "narrative" => {
                self.narrative_id.is_some() && self.available_actions.is_none() && !timer
            }
            "actions" => self.available_actions.is_some() && self.narrative_id.is_none() && !timer,
            "timer" => timer && self.narrative_id.is_none() && self.available_actions.is_none(),
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for AuthoredCue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let f = CueFields::<ActionSet>::deserialize(deserializer)?;
        if !f.valid() || f.after_cue.is_some() {
            return Err(serde::de::Error::custom(
                "cue kind/payload mismatch or authored after_cue is unsupported",
            ));
        }
        let instruction = match f.kind.as_str() {
            "narrative" => AuthoredInstruction::Narrative {
                narrative_id: f.narrative_id.unwrap(),
            },
            "actions" => AuthoredInstruction::Actions {
                available_actions: f.available_actions.unwrap(),
            },
            _ => AuthoredInstruction::Timer {
                start_ms: f.start_ms,
                extend_ms: f.extend_ms,
                visible: f.visible,
            },
        };
        Ok(Self {
            cue_id: f.cue_id,
            at_ms: f.at_ms,
            when: f.when,
            instruction,
        })
    }
}

impl<'de> Deserialize<'de> for Cue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let f = CueFields::<Vec<String>>::deserialize(deserializer)?;
        if !f.valid() || f.cue_id.is_none() || f.when.is_some() {
            return Err(serde::de::Error::custom("invalid Cue Sheet instruction"));
        }
        let instruction = match f.kind.as_str() {
            "narrative" => Instruction::Narrative {
                narrative_id: f.narrative_id.unwrap(),
            },
            "actions" => Instruction::Actions {
                available_actions: f.available_actions.unwrap(),
            },
            _ => Instruction::Timer {
                start_ms: f.start_ms,
                extend_ms: f.extend_ms,
                visible: f.visible,
            },
        };
        Ok(Self {
            cue_id: f.cue_id.unwrap(),
            at_ms: f.at_ms,
            after_cue: f.after_cue,
            instruction,
        })
    }
}
