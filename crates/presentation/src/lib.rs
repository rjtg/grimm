//! Text presentation and raw interaction, with no engine or semantic dependency.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresentationCommand {
    Text(String),
    Choices(Vec<String>),
    Countdown { remaining_ms: u64, label: String },
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
        assert_eq!(
            input,
            serde_json::from_str(&serde_json::to_string(&input).unwrap()).unwrap()
        );
    }
}
