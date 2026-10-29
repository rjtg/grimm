//! Text presentation and raw interaction, with no engine or semantic dependency.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresentationCommand {
    Text(String),
    Choices(Vec<String>),
    Countdown { remaining_ms: u64, label: String },
    AwaitCompletion { revision: u64, prompt: String },
    InteractionAvailable { revision: u64 },
}

/// A renderer observation for the host to correlate and translate, never a
/// canonical transition or a renderer-to-engine call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationFeedback {
    pub revision: u64,
    pub event: PresentationEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresentationEvent {
    Complete,
    InteractionReady,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawInput {
    pub line: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_round_trips() {
        for command in [
            PresentationCommand::AwaitCompletion {
                revision: 3,
                prompt: "Read".into(),
            },
            PresentationCommand::InteractionAvailable { revision: 4 },
            PresentationCommand::Text("Hello\nworld".into()),
            PresentationCommand::Choices(vec!["Continue".into()]),
            PresentationCommand::Countdown {
                remaining_ms: 1234,
                label: "Time remaining".into(),
            },
        ] {
            assert_eq!(
                command,
                serde_json::from_str(&serde_json::to_string(&command).unwrap()).unwrap()
            );
        }
        let input = RawInput { line: "1\n".into() };
        for event in [
            PresentationEvent::Complete,
            PresentationEvent::InteractionReady,
        ] {
            let feedback = PresentationFeedback { revision: 6, event };
            assert_eq!(
                feedback,
                serde_json::from_str(&serde_json::to_string(&feedback).unwrap()).unwrap()
            );
        }
        assert_eq!(
            input,
            serde_json::from_str(&serde_json::to_string(&input).unwrap()).unwrap()
        );
    }
}
