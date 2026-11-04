//! Concrete WHITEOUT text/color and interaction-presentation bindings.
pub(crate) struct Panel {
    pub title: &'static str,
    pub body: &'static str,
    pub accent: [f32; 3],
    pub actions_open: bool,
    pub countdown: bool,
}

pub(crate) fn presentation(id: &str) -> Option<Panel> {
    Some(match id {
        "generator_room" => Panel {
            title: "The generator room",
            body: "Snow scours the station walls. Beneath your boots, the generator hums unevenly.\n\nA spare relay lies in the supply cabinet. Take it before inspecting the panel if you want to attempt a repair.",
            accent: [0.25, 0.65, 0.75],
            actions_open: true,
            countdown: false,
        },
        "generator_warning" => Panel {
            title: "Something is wrong",
            body: "The warning lamp flickers. Read the panel: the controls open in five seconds.\n\nThe radio crackles: 'If the generator stops, we lose the heaters.'",
            accent: [0.95, 0.28, 0.20],
            actions_open: false,
            countdown: false,
        },
        "generator_ready" => Panel {
            title: "Act before the belt snaps",
            body: "The warning lamp turns red. You can repair the drive with a spare relay, or isolate the generator.\n\nChoose before the countdown reaches zero. Waiting leads to a blackout.",
            accent: [0.95, 0.28, 0.20],
            actions_open: true,
            countdown: true,
        },
        "generator_transition" => Panel {
            title: "Pulling the breaker",
            body: "You reach for the breaker. The lights dip as you isolate the damaged drive.\n\nThis short presentation will lead to the next scene automatically.",
            accent: [0.7, 0.55, 0.25],
            actions_open: false,
            countdown: false,
        },
        "generator_decision" => Panel {
            title: "A moment of safety",
            body: "The breaker is open. The damaged generator is isolated.\n\nEnd of this scheduler demo. The heaters are safe for now, but power is not restored. A spare relay could have repaired the drive.",
            accent: [0.45, 0.8, 0.55],
            actions_open: true,
            countdown: false,
        },
        "generator_failed" => Panel {
            title: "WHITEOUT / Blackout",
            body: "The belt snaps before you reach the breaker. The station goes dark.\n\nYou waited too long. Restart the demo to try an earlier intervention.",
            accent: [0.6, 0.15, 0.15],
            actions_open: true,
            countdown: false,
        },
        "generator_supplies" => Panel {
            title: "The spare relay",
            body: "You reach into the supply cabinet for the replacement relay.\n\nOnce it is in your pocket, you will return to the control panel.",
            accent: [0.4, 0.65, 0.8],
            actions_open: false,
            countdown: false,
        },
        "generator_repair" => Panel {
            title: "Replacing the relay",
            body: "You open the relay housing and fit the spare part.\n\nThe repair takes a moment. Watch the generator status change when it completes.",
            accent: [0.8, 0.6, 0.25],
            actions_open: false,
            countdown: false,
        },
        "generator_repaired" => Panel {
            title: "Power restored",
            body: "The replacement relay clicks into place. The generator settles into a steady hum.\n\nThe heaters and radio stay online. The spare part has been used.",
            accent: [0.35, 0.9, 0.5],
            actions_open: true,
            countdown: false,
        },
        "generator_reset" => Panel {
            title: "A fresh attempt",
            body: "Returning to the station before the alarm. The demo resets the generator and the spare relay.",
            accent: [0.25, 0.65, 0.75],
            actions_open: false,
            countdown: false,
        },
        _ => return None,
    })
}

pub(crate) fn edge_label(id: &str) -> Option<&'static str> {
    Some(match id {
        "inspect_panel" => "Inspect the control panel",
        "take_spare_part" => "Take the spare relay",
        "repair_generator" => "Repair the generator with the spare relay",
        "isolate_generator" => "Isolate the damaged generator",
        "restart_demo" => "Return to the beginning (demo)",
        _ => return None,
    })
}
