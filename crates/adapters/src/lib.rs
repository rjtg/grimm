//! Adapter contracts and authored-text presentation.
//! Adapters have protocol dependencies only, never access to the engine.

use grimm_presentation::{PresentationCommand, RawInput};
use grimm_presentation::{PresentationEvent, PresentationFeedback};
use grimm_semantic::flow::{FlowEvent, FlowInput, FlowPhase};
use grimm_semantic::time::DecisionPhase;
use grimm_semantic::{GameAction, NarrativeContent, SemanticOutput};

/// Host-side translation, deliberately independent of any engine implementation.
pub fn lifecycle_input(feedback: PresentationFeedback) -> FlowInput {
    FlowInput {
        revision: feedback.revision,
        event: match feedback.event {
            PresentationEvent::Complete => FlowEvent::PresentationComplete,
            PresentationEvent::InteractionReady => FlowEvent::InteractionReady,
        },
    }
}

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

impl TextAdapter {
    /// Refresh timing presentation without repeating prose or remapping actions.
    pub fn present_deadline(&self, output: &SemanticOutput) -> Vec<PresentationCommand> {
        if output
            .flow
            .as_ref()
            .is_some_and(|flow| flow.phase == FlowPhase::Presenting)
        {
            return vec![];
        }
        output
            .decision
            .iter()
            .map(|decision| PresentationCommand::Countdown {
                remaining_ms: decision.remaining_ms,
                label: match decision.phase {
                    DecisionPhase::AwaitingReady => "Decision time (starts after prompt)",
                    DecisionPhase::Active => "Time remaining",
                    DecisionPhase::Paused => "Paused; time remaining",
                }
                .into(),
            })
            .collect()
    }
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
            .filter(|_| {
                output
                    .flow
                    .as_ref()
                    .is_none_or(|flow| flow.phase == FlowPhase::Presenting)
            })
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
        if let Some(flow) = &output.flow {
            if flow.paused {
                commands.push(PresentationCommand::Text(
                    "Paused. Use :resume or :save PATH.".into(),
                ));
                return commands;
            }
            if flow.phase == FlowPhase::Presenting {
                commands.push(PresentationCommand::AwaitCompletion {
                    revision: output.revision,
                    prompt: "Press Enter when finished reading...".into(),
                });
                return commands;
            }
        }
        commands.extend(self.present_deadline(output));
        if !output.offers.is_empty() {
            commands.push(PresentationCommand::Choices(
                output
                    .offers
                    .iter()
                    .map(|offer| offer.label.clone())
                    .collect(),
            ));
        }
        if output
            .flow
            .as_ref()
            .is_some_and(|flow| flow.phase == FlowPhase::AwaitingInteraction)
        {
            commands.push(PresentationCommand::InteractionAvailable {
                revision: output.revision,
            });
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
    fn required_reading_has_no_choices_or_countdown_and_feedback_is_typed() {
        let mut output = SemanticOutput {
            revision: 9,
            content: vec![NarrativeContent::Narration {
                text: "Required dialogue.".into(),
            }],
            offers: vec![],
            ending: None,
            decision: Some(grimm_semantic::time::DecisionView {
                remaining_ms: 5000,
                phase: DecisionPhase::AwaitingReady,
            }),
            flow: Some(grimm_semantic::flow::FlowView {
                phase: FlowPhase::Presenting,
                paused: false,
            }),
        };
        let mut adapter = TextAdapter::default();
        let commands = adapter.present(&output);
        assert!(matches!(
            &commands[..],
            [
                PresentationCommand::Text(_),
                PresentationCommand::AwaitCompletion { revision: 9, .. }
            ]
        ));
        assert!(adapter.resolve(&RawInput { line: "1".into() }).is_none());
        let feedback = PresentationFeedback {
            revision: 9,
            event: PresentationEvent::Complete,
        };
        assert_eq!(
            lifecycle_input(feedback),
            FlowInput {
                revision: 9,
                event: FlowEvent::PresentationComplete
            }
        );
        output.revision = 10;
        output.flow.as_mut().unwrap().phase = FlowPhase::AwaitingInteraction;
        output.offers.push(InteractionOffer {
            id: "act".into(),
            label: "Act".into(),
        });
        let commands = adapter.present(&output);
        assert!(
            !commands
                .iter()
                .any(|command| matches!(command, PresentationCommand::Text(_)))
        );
        assert!(matches!(
            commands.last(),
            Some(PresentationCommand::InteractionAvailable { revision: 10 })
        ));
        assert_eq!(
            adapter
                .resolve(&RawInput { line: "1".into() })
                .unwrap()
                .revision,
            10
        );
    }

    #[test]
    fn deadline_updates_do_not_remap_or_invent_actions() {
        let mut adapter = TextAdapter::default();
        let mut output = SemanticOutput {
            flow: None,
            revision: 7,
            content: vec![],
            offers: vec![InteractionOffer {
                id: "answer".into(),
                label: "Answer".into(),
            }],
            ending: None,
            decision: Some(grimm_semantic::time::DecisionView {
                remaining_ms: 900,
                phase: DecisionPhase::AwaitingReady,
            }),
        };
        adapter.present(&output);
        for phase in [
            DecisionPhase::AwaitingReady,
            DecisionPhase::Active,
            DecisionPhase::Paused,
        ] {
            output.decision.as_mut().unwrap().phase = phase;
            let command = adapter.present_deadline(&output);
            assert!(matches!(
                &command[..],
                [PresentationCommand::Countdown {
                    remaining_ms: 900,
                    ..
                }]
            ));
            assert_eq!(
                adapter
                    .resolve(&RawInput { line: "1".into() })
                    .unwrap()
                    .choice_id,
                "answer"
            );
        }
    }

    #[test]
    fn preserves_authored_blocks_and_formats_dialogue_and_ending() {
        let output = SemanticOutput {
            flow: None,
            decision: None,
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
            flow: None,
            decision: None,
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
            flow: None,
            decision: None,
            offers: vec![],
            ..output
        });
        assert_eq!(adapter.resolve(&RawInput { line: "1".into() }), None);
    }
}
