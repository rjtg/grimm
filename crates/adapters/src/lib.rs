//! Adapter contracts and authored-text presentation.
//! Adapters have protocol dependencies only, never access to the engine.

use grimm_presentation::{PresentationCommand, RawInput};
use grimm_semantic::{GameAction, NarrativeContent, SemanticOutput};

pub trait OutputAdapter {
    fn present(&mut self, output: &SemanticOutput) -> Vec<PresentationCommand>;
}

pub trait InputAdapter {
    fn resolve(&self, input: &RawInput) -> Option<GameAction>;
}

/// Holds the current screen's number-to-offer mapping, not canonical game state.
#[derive(Debug, Default)]
pub struct TextAdapter {
    actions: Vec<GameAction>,
}

impl OutputAdapter for TextAdapter {
    fn present(&mut self, output: &SemanticOutput) -> Vec<PresentationCommand> {
        self.actions = output
            .offers
            .iter()
            .map(|offer| GameAction {
                revision: output.revision,
                choice_id: offer.id.clone(),
            })
            .collect();
        let mut commands: Vec<_> = output
            .content
            .iter()
            .map(|content| {
                PresentationCommand::Text(match content {
                    NarrativeContent::Narration { text } => text.clone(),
                    NarrativeContent::Dialogue { speaker, text } => format!("{speaker}: {text}"),
                })
            })
            .collect();
        if let Some(ending) = &output.ending {
            commands.push(PresentationCommand::Text(format!(
                "Ending: {}",
                ending.title
            )));
        }
        if !output.offers.is_empty() {
            commands.push(PresentationCommand::Choices(
                output
                    .offers
                    .iter()
                    .map(|offer| offer.label.clone())
                    .collect(),
            ));
        }
        commands
    }
}

impl InputAdapter for TextAdapter {
    fn resolve(&self, input: &RawInput) -> Option<GameAction> {
        let index = input.line.trim().parse::<usize>().ok()?.checked_sub(1)?;
        self.actions.get(index).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grimm_semantic::InteractionOffer;

    #[test]
    fn preserves_authored_blocks_and_formats_dialogue_and_ending() {
        let output = SemanticOutput {
            revision: 3,
            content: vec![
                NarrativeContent::Narration {
                    text: "Exact text.\n  Indentation stays.".into(),
                },
                NarrativeContent::Dialogue {
                    speaker: "Mara".into(),
                    text: "I saw it.\nEvery word.".into(),
                },
            ],
            offers: vec![],
            ending: Some(grimm_semantic::Ending {
                id: "truth".into(),
                title: "The Truth".into(),
            }),
        };
        assert_eq!(
            TextAdapter::default().present(&output),
            vec![
                PresentationCommand::Text("Exact text.\n  Indentation stays.".into()),
                PresentationCommand::Text("Mara: I saw it.\nEvery word.".into()),
                PresentationCommand::Text("Ending: The Truth".into()),
            ]
        );
    }

    #[test]
    fn maps_screen_numbers_to_semantic_ids_and_replaces_old_offers() {
        let mut adapter = TextAdapter::default();
        assert_eq!(adapter.resolve(&RawInput { line: "1".into() }), None);
        let output = SemanticOutput {
            revision: 42,
            content: vec![NarrativeContent::Narration {
                text: "Exact text.".into(),
            }],
            ending: None,
            offers: vec![InteractionOffer {
                id: "semantic-id".into(),
                label: "Visible label".into(),
            }],
        };
        assert_eq!(
            adapter.present(&output),
            vec![
                PresentationCommand::Text("Exact text.".into()),
                PresentationCommand::Choices(vec!["Visible label".into()])
            ]
        );
        assert_eq!(
            adapter.resolve(&RawInput {
                line: " 1\n".into()
            }),
            Some(GameAction {
                revision: 42,
                choice_id: "semantic-id".into()
            })
        );
        for line in [
            "0",
            "2",
            "-1",
            "",
            "hello",
            "99999999999999999999999999999999999999",
        ] {
            assert_eq!(adapter.resolve(&RawInput { line: line.into() }), None);
        }
        adapter.present(&SemanticOutput {
            offers: vec![],
            ..output
        });
        assert_eq!(adapter.resolve(&RawInput { line: "1".into() }), None);
    }
}
