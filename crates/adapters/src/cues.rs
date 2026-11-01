//! Mechanical semantic/presentation conversion, never game-rule evaluation.
use grimm_presentation::cues as p;
use grimm_semantic::scene as s;

pub fn present(sheet: &s::CueSheet) -> p::CueSheet {
    p::CueSheet {
        protocol_version: sheet.protocol_version,
        revision: sheet.revision,
        clock: p::Clock {
            epoch: sheet.clock.epoch,
            elapsed_ms: sheet.clock.elapsed_ms,
            running: sheet.clock.running,
            deadline_ms: sheet.clock.deadline_ms,
            visible: sheet.clock.visible,
        },
        cues: sheet
            .cues
            .iter()
            .map(|cue| p::Cue {
                cue_id: cue.cue_id.clone(),
                at_ms: cue.at_ms,
                started_at_ms: cue.started_at_ms,
                after_cue: cue.after_cue.clone(),
                instruction: match &cue.instruction {
                    s::Instruction::Narrative {
                        narrative_id,
                        duration_ms,
                    } => p::Instruction::Narrative {
                        narrative_id: narrative_id.clone(),
                        duration_ms: *duration_ms,
                    },
                    s::Instruction::Actions { available_actions } => p::Instruction::Actions {
                        available_actions: available_actions.clone(),
                    },
                    s::Instruction::Timer {
                        start_ms,
                        deadline_at_ms,
                        unset_deadline,
                        extend_ms,
                        visible,
                    } => p::Instruction::Timer {
                        start_ms: *start_ms,
                        deadline_at_ms: *deadline_at_ms,
                        unset_deadline: *unset_deadline,
                        extend_ms: *extend_ms,
                        visible: *visible,
                    },
                },
            })
            .collect(),
    }
}

pub fn feedback(input: p::CueFeedback) -> s::CueInput {
    s::CueInput {
        revision: input.revision,
        cue_id: input.cue_id,
        scene_time_ms: input.scene_time_ms,
        event: match input.event {
            p::CueEvent::Started => s::CueEvent::Started,
            p::CueEvent::Completed => s::CueEvent::Completed,
        },
    }
}

pub fn action(input: p::ActionInput) -> s::SceneAction {
    s::SceneAction {
        revision: input.revision,
        action_id: input.action_id,
        scene_time_ms: input.scene_time_ms,
    }
}
