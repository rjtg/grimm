//! WHITEOUT rules over its actual Bevy resource. No general inventory/rules engine.
use bevy::prelude::*;
use grimm_graph::{ConditionCall, Edge, Parameters};
use grimm_semantic::story::Value;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneratorStatus {
    #[default]
    Running,
    Unstable,
    Isolated,
    Stopped,
    Repaired,
    Powered,
}
impl GeneratorStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Unstable => "Warning light / unstable",
            Self::Isolated => "Safely isolated",
            Self::Stopped => "Stopped / station dark",
            Self::Repaired => "Repaired / station powered",
            Self::Powered => "Power stable",
        }
    }
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "running" => Ok(Self::Running),
            "unstable" => Ok(Self::Unstable),
            "isolated" => Ok(Self::Isolated),
            "stopped" => Ok(Self::Stopped),
            "repaired" => Ok(Self::Repaired),
            "powered" => Ok(Self::Powered),
            _ => Err(format!("unknown generator status '{value}'")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemId {
    SparePart,
}
impl ItemId {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "spare_part" => Ok(Self::SparePart),
            _ => Err(format!("unknown WHITEOUT item '{value}'")),
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct Generator {
    pub status: GeneratorStatus,
    pub has_spare_part: bool,
    pub deicer_isolated: bool,
    pub engineering_lost: bool,
    pub applications: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Condition {
    Repair { required_item: ItemId },
    TakeSparePart,
    StartGenerator,
}
impl Condition {
    pub fn parse(call: &ConditionCall) -> Result<Self, String> {
        match call.condition_id.as_str() {
            "can_repair_generator" => Ok(Self::Repair {
                required_item: ItemId::parse(one_text(&call.args, "required_item")?)?,
            }),
            "can_start_generator" => {
                no_args(&call.args)?;
                Ok(Self::StartGenerator)
            }
            "can_take_spare_part" => {
                no_args(&call.args)?;
                Ok(Self::TakeSparePart)
            }
            id => Err(format!("unknown WHITEOUT condition '{id}'")),
        }
    }

    fn blocked_reason(self, state: &Generator) -> Option<&'static str> {
        match self {
            Self::Repair {
                required_item: ItemId::SparePart,
            } if !state.has_spare_part => Some("requires a spare part"),
            Self::Repair { .. } if state.status != GeneratorStatus::Unstable => {
                Some("requires an unstable generator")
            }
            Self::TakeSparePart if state.has_spare_part => Some("spare part already carried"),
            Self::TakeSparePart if state.status != GeneratorStatus::Running => {
                Some("supplies require a safe generator room")
            }
            Self::StartGenerator if state.engineering_lost => Some("annex access is sealed"),
            Self::StartGenerator if !state.deicer_isolated => {
                Some("isolate the overloaded de-icer first")
            }
            Self::StartGenerator
                if matches!(
                    state.status,
                    GeneratorStatus::Repaired | GeneratorStatus::Powered
                ) =>
            {
                Some("generator is already supplying power")
            }
            _ => None,
        }
    }
}

/// None means available. Errors remain errors, never a permissive default.
pub(crate) fn blocked_reason(
    edge: &Edge,
    state: &Generator,
) -> Result<Option<&'static str>, String> {
    match &edge.condition {
        Some(call) => Ok(Condition::parse(call)?.blocked_reason(state)),
        None => Ok(None),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Effect {
    SetGeneratorStatus(GeneratorStatus),
    TakeItem(ItemId),
    RepairGenerator { required_item: ItemId },
    ResetDemo,
    PrepareAnnex,
    IsolateDeicer,
    StartGenerator,
    SealAnnex,
}
impl Effect {
    pub fn parse(id: &str, args: &Parameters) -> Result<Self, String> {
        match id {
            "set_generator_status" => Ok(Self::SetGeneratorStatus(GeneratorStatus::parse(
                one_text(args, "status")?,
            )?)),
            "take_item" => Ok(Self::TakeItem(ItemId::parse(one_text(args, "item")?)?)),
            "repair_generator" => Ok(Self::RepairGenerator {
                required_item: ItemId::parse(one_text(args, "required_item")?)?,
            }),
            "prepare_annex" => {
                no_args(args)?;
                Ok(Self::PrepareAnnex)
            }
            "isolate_deicer" => {
                no_args(args)?;
                Ok(Self::IsolateDeicer)
            }
            "start_generator" => {
                no_args(args)?;
                Ok(Self::StartGenerator)
            }
            "seal_annex" => {
                no_args(args)?;
                Ok(Self::SealAnnex)
            }
            "reset_demo" => {
                no_args(args)?;
                Ok(Self::ResetDemo)
            }
            _ => Err(format!("unknown WHITEOUT effect '{id}'")),
        }
    }
}

impl Generator {
    pub fn apply(&mut self, id: &str, args: &Parameters) -> Result<(), String> {
        let effect = Effect::parse(id, args)?;
        // Compute and validate every value before touching actual ECS fields.
        let mut isolated = self.deicer_isolated;
        let mut lost = self.engineering_lost;
        let (status, has_spare_part) = match effect {
            Effect::SetGeneratorStatus(status) => (status, self.has_spare_part),
            Effect::TakeItem(ItemId::SparePart) => {
                if self.has_spare_part {
                    return Err("spare part already carried".into());
                }
                (self.status, true)
            }
            Effect::RepairGenerator { required_item } => {
                if let Some(reason) = (Condition::Repair { required_item }).blocked_reason(self) {
                    return Err(format!("cannot repair generator: {reason}"));
                }
                (GeneratorStatus::Repaired, false)
            }
            Effect::ResetDemo => {
                isolated = false;
                lost = false;
                (GeneratorStatus::Running, false)
            }
            Effect::PrepareAnnex => {
                isolated = false;
                lost = false;
                (GeneratorStatus::Stopped, false)
            }
            Effect::IsolateDeicer => {
                isolated = true;
                (self.status, self.has_spare_part)
            }
            Effect::StartGenerator => {
                if let Some(reason) = Condition::StartGenerator.blocked_reason(self) {
                    return Err(format!("cannot start generator: {reason}"));
                }
                (GeneratorStatus::Powered, self.has_spare_part)
            }
            Effect::SealAnnex => {
                lost = true;
                (self.status, self.has_spare_part)
            }
        };
        let applications = self
            .applications
            .checked_add(1)
            .ok_or("generator effect counter overflow")?;
        self.status = status;
        self.has_spare_part = has_spare_part;
        self.deicer_isolated = isolated;
        self.engineering_lost = lost;
        self.applications = applications;
        Ok(())
    }
}

fn one_text<'a>(args: &'a Parameters, name: &str) -> Result<&'a str, String> {
    match (args.len(), args.get(name)) {
        (1, Some(Value::Text(value))) => Ok(value),
        _ => Err(format!("expected exactly one text argument '{name}'")),
    }
}
fn no_args(args: &Parameters) -> Result<(), String> {
    if args.is_empty() {
        Ok(())
    } else {
        Err("expected no arguments".into())
    }
}

#[cfg(test)]
mod tests;
