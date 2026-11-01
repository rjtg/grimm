//! M4.6 lifecycle translation. No rule evaluation or resource loading.
use grimm_presentation::lifecycle as p;
use grimm_semantic::lifecycle as s;
pub fn present(sheet: &s::SceneCueSheet) -> p::SceneCueSheet {
    p::SceneCueSheet {
        protocol_version: sheet.protocol_version,
        scene: p::ScenePresentation {
            phase: match sheet.scene.phase {
                s::ScenePhase::Prepare => p::ScenePhase::Prepare,
                s::ScenePhase::Active => p::ScenePhase::Active,
                s::ScenePhase::Ending => p::ScenePhase::Ending,
                s::ScenePhase::Stable => p::ScenePhase::Stable,
            },
            scene_id: sheet.scene.scene_id.clone(),
            epoch: sheet.scene.epoch,
            next_scene_id: sheet.scene.next_scene_id.clone(),
            ready: sheet.scene.ready,
        },
        plan: crate::cues::present(&sheet.plan),
    }
}
pub fn feedback(input: p::SceneFeedback) -> s::SceneFeedback {
    s::SceneFeedback {
        revision: input.revision,
        scene_id: input.scene_id,
        epoch: input.epoch,
        event: match input.event {
            p::SceneEvent::Ready => s::SceneEvent::Ready,
            p::SceneEvent::Begin => s::SceneEvent::Begin,
            p::SceneEvent::End => s::SceneEvent::End,
            p::SceneEvent::Prepare => s::SceneEvent::Prepare,
            p::SceneEvent::PreparationFailed => s::SceneEvent::PreparationFailed,
        },
    }
}
