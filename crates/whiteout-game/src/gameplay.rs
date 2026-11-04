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
}
impl GeneratorStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Unstable => "Warning light / unstable",
            Self::Isolated => "Safely isolated",
            Self::Stopped => "Stopped / station dark",
            Self::Repaired => "Repaired / station powered",
        }
    }
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "running" => Ok(Self::Running),
            "unstable" => Ok(Self::Unstable),
            "isolated" => Ok(Self::Isolated),
            "stopped" => Ok(Self::Stopped),
            "repaired" => Ok(Self::Repaired),
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
    pub applications: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Condition {
    CanRepair { required_item: ItemId },
    CanTakeSparePart,
}
impl Condition {
    pub fn parse(call: &ConditionCall) -> Result<Self, String> {
        match call.condition_id.as_str() {
            "can_repair_generator" => Ok(Self::CanRepair {
                required_item: ItemId::parse(one_text(&call.args, "required_item")?)?,
            }),
            "can_take_spare_part" => {
                no_args(&call.args)?;
                Ok(Self::CanTakeSparePart)
            }
            id => Err(format!("unknown WHITEOUT condition '{id}'")),
        }
    }

    fn blocked_reason(self, state: &Generator) -> Option<&'static str> {
        match self {
            Self::CanRepair {
                required_item: ItemId::SparePart,
            } if !state.has_spare_part => Some("requires a spare part"),
            Self::CanRepair { .. } if state.status != GeneratorStatus::Unstable => {
                Some("requires an unstable generator")
            }
            Self::CanTakeSparePart if state.has_spare_part => Some("spare part already carried"),
            Self::CanTakeSparePart if state.status != GeneratorStatus::Running => {
                Some("supplies require a safe generator room")
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
        let (status, has_spare_part) = match effect {
            Effect::SetGeneratorStatus(status) => (status, self.has_spare_part),
            Effect::TakeItem(ItemId::SparePart) => {
                if self.has_spare_part {
                    return Err("spare part already carried".into());
                }
                (self.status, true)
            }
            Effect::RepairGenerator { required_item } => {
                if let Some(reason) = (Condition::CanRepair { required_item }).blocked_reason(self)
                {
                    return Err(format!("cannot repair generator: {reason}"));
                }
                (GeneratorStatus::Repaired, false)
            }
            Effect::ResetDemo => (GeneratorStatus::Running, false),
        };
        let applications = self
            .applications
            .checked_add(1)
            .ok_or("generator effect counter overflow")?;
        self.status = status;
        self.has_spare_part = has_spare_part;
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
