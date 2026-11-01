//! Host composition: legacy engine or explicitly opted-in lifecycle engine.
use grimm_engine::scene::{SceneEngine, SceneError, SceneResult, lifecycle::LifecycleEngine};
use grimm_presentation::lifecycle::{SceneEvent, SceneFeedback, ScenePresentation};
use grimm_semantic::scene::{CueInput, CueSheet, SceneAction, SceneSave};

pub(super) enum Backend {
    Legacy(Box<SceneEngine>),
    Lifecycle(Box<LifecycleEngine>),
}
impl Backend {
    pub fn core(&self) -> &SceneEngine {
        match self {
            Self::Legacy(engine) => engine,
            Self::Lifecycle(engine) => engine.engine(),
        }
    }
    pub fn output(&self) -> CueSheet {
        match self {
            Self::Legacy(engine) => engine.output(),
            Self::Lifecycle(engine) => engine.output().plan,
        }
    }
    pub fn scene(&self) -> Option<ScenePresentation> {
        match self {
            Self::Legacy(_) => None,
            Self::Lifecycle(engine) => {
                Some(grimm_adapters::lifecycle::present(&engine.output()).scene)
            }
        }
    }
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Legacy(engine) => engine.is_terminal(),
            Self::Lifecycle(engine) => engine.is_complete(),
        }
    }
    pub fn provider_revision(&self) -> Option<u64> {
        match self {
            Self::Legacy(_) => None,
            Self::Lifecycle(engine) => Some(engine.world_snapshot().revision),
        }
    }
    pub fn apply(&mut self, input: &SceneAction) -> SceneResult<CueSheet> {
        match self {
            Self::Legacy(engine) => engine.apply(input),
            Self::Lifecycle(engine) => engine.apply(input).map(|sheet| sheet.plan),
        }
    }
    pub fn cue_input(&mut self, input: &CueInput) -> SceneResult<CueSheet> {
        match self {
            Self::Legacy(engine) => engine.cue_input(input),
            Self::Lifecycle(engine) => engine.cue_input(input).map(|sheet| sheet.plan),
        }
    }
    pub fn observe(&mut self, revision: u64, time: u64) -> SceneResult<CueSheet> {
        match self {
            Self::Legacy(engine) => engine.observe(revision, time),
            Self::Lifecycle(engine) => engine.observe(revision, time).map(|sheet| sheet.plan),
        }
    }
    pub fn event(&mut self, event: SceneEvent) -> SceneResult<()> {
        let Self::Lifecycle(engine) = self else {
            return Err(SceneError(
                "legacy game has no explicit scene lifecycle".into(),
            ));
        };
        let sheet = grimm_adapters::lifecycle::present(&engine.output());
        let input = SceneFeedback {
            revision: sheet.plan.revision,
            scene_id: sheet.scene.scene_id,
            epoch: sheet.scene.epoch,
            event,
        };
        engine.scene_input(&grimm_adapters::lifecycle::feedback(input))?;
        Ok(())
    }
    pub fn restore(&mut self, save: SceneSave) -> SceneResult<()> {
        let Self::Legacy(engine) = self else {
            return Err(SceneError("use a boundary checkpoint for M4.6".into()));
        };
        engine.restore(save)?;
        Ok(())
    }
    pub fn save_json(&self) -> SceneResult<String> {
        match self {
            Self::Legacy(engine) => serde_json::to_string_pretty(&engine.save()),
            Self::Lifecycle(engine) => serde_json::to_string_pretty(&engine.checkpoint()?),
        }
        .map_err(|e| SceneError(e.to_string()))
    }
    pub fn restore_json(&mut self, json: &str) -> SceneResult<()> {
        match self {
            Self::Legacy(engine) => {
                engine
                    .restore(serde_json::from_str(json).map_err(|e| SceneError(e.to_string()))?)?;
            }
            Self::Lifecycle(engine) => {
                engine
                    .restore(serde_json::from_str(json).map_err(|e| SceneError(e.to_string()))?)?;
            }
        }
        Ok(())
    }
}
