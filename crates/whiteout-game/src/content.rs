//! Concrete WHITEOUT text/color bindings, outside the story graph.
pub(crate) struct Panel {
    pub title: &'static str,
    pub body: &'static str,
    pub accent: [f32; 3],
}

pub(crate) fn presentation(id: &str) -> Option<Panel> {
    Some(match id {
        "generator_room" => Panel {
            title: "The generator room",
            body: "Snow scours the station walls. Beneath your boots, the generator hums unevenly.\n\nA small inspection lamp flickers beside the control panel.",
            accent: [0.25, 0.65, 0.75],
        },
        "generator_warning" => Panel {
            title: "Something is wrong",
            body: "The lamp turns red. A belt shudders behind its guard.\n\nThe radio crackles: 'If the generator stops, we lose the heaters.'",
            accent: [0.95, 0.28, 0.20],
        },
        "generator_decision" => Panel {
            title: "At the breaker",
            body: "Your hand rests beside the breaker. The station is counting on you.\n\nEnd of this bootstrap: repair rules and deadlines arrive in later work.",
            accent: [0.95, 0.67, 0.25],
        },
        _ => return None,
    })
}

pub(crate) fn edge_label(id: &str) -> Option<&'static str> {
    Some(match id {
        "inspect_panel" => "Inspect the control panel",
        "approach_switch" => "Move to the breaker",
        "restart_demo" => "Return to the beginning (demo)",
        _ => return None,
    })
}
