//! Host-only authoring reports. Never part of a player-visible Cue Sheet.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub file: String,
    /// JSON Pointer into the authored YAML document, not a runtime occurrence.
    pub path: String,
    /// One-based positions when the source codec can locate this declaration.
    pub line: Option<usize>,
    pub column: Option<usize>,
}
impl Source {
    pub fn at(path: impl Into<String>) -> Self {
        Self {
            file: String::new(),
            path: path.into(),
            line: None,
            column: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub source: Source,
    pub related: Vec<Source>,
    pub suggestion: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Declaration {
    pub id: String,
    pub source: Source,
    pub uses: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CueIndex {
    pub cue_id: String,
    pub generated: bool,
    pub narrative_id: Option<String>,
    pub source: Source,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIndex {
    pub narrative_id: String,
    pub uses: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition {
    pub target: String,
    pub action_id: Option<String>,
    /// action, interrupted_action, or timeout.
    pub kind: String,
    pub source: Source,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextIndex {
    pub id: String,
    pub namespace: String,
    pub terminal: bool,
    pub source: Source,
    pub actions: Vec<Declaration>,
    pub cues: Vec<CueIndex>,
    pub narratives: Vec<ContentIndex>,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryIndex {
    pub schema_version: u32,
    pub story_id: String,
    pub story_version: u32,
    pub contexts: Vec<ContextIndex>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub schema_version: u32,
    pub diagnostics: Vec<Diagnostic>,
    /// Present after successful DSL compilation, even if structural lint fails.
    pub index: Option<StoryIndex>,
}
impl Report {
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }
}
