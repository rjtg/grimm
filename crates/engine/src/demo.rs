//! The original M0 demo expressed as an ordinary authored definition.
use grimm_semantic::{
    Ending, NarrativeContent,
    story::{Choice, Node, STORY_SCHEMA_VERSION, Story},
};

pub(crate) fn story() -> Story {
    Story {
        time: None,
        schema_version: STORY_SCHEMA_VERSION,
        id: "m0-demo".into(),
        version: 1,
        title: "GRIMM architecture demo".into(),
        start: "start".into(),
        variables: Default::default(),
        nodes: vec![
            Node {
                continue_to: None,
                presentation_required: false,
                decision: None,
                id: "start".into(),
                content: vec![NarrativeContent::Narration {
                    text: "Welcome to GRIMM's architecture demo.".into(),
                }],
                choices: vec![Choice {
                    advance_minutes: 0,
                    id: "continue".into(),
                    label: "Continue".into(),
                    target: "end".into(),
                    condition: None,
                    effects: vec![],
                }],
                ending: None,
            },
            Node {
                continue_to: None,
                presentation_required: false,
                decision: None,
                id: "end".into(),
                content: vec![NarrativeContent::Narration {
                    text: "The engine accepted your choice. Demo complete.".into(),
                }],
                choices: vec![],
                ending: Some(Ending {
                    id: "complete".into(),
                    title: "Demo complete".into(),
                }),
            },
        ],
    }
}
