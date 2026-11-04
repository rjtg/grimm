//! Deterministic host demonstration; no clock thread, rules library or cue ACKs.
use grimm_graph::{GraphEngine, Instruction, StoryGraph};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let story: StoryGraph = serde_yaml_ng::from_str(include_str!(
        "../../../examples/graph-foundation/locked-in.yaml"
    ))?;
    let mut graph = GraphEngine::new(story)?;
    println!(
        "{} epoch={}",
        graph.current().node_id,
        graph.current().epoch
    );
    // The game host has chosen this edge. The graph does not ask a condition
    // handler or need access to the host's gameplay state.
    let middle = graph
        .traverse_edge("break_door", graph.current().epoch)?
        .cue_sheet;
    println!("{} epoch={}", middle.node_id, middle.epoch);
    let mut door_broken = false;
    for at_ms in [0, 1200, 1500] {
        for cue in middle.cues.iter().filter(|cue| cue.at_ms == at_ms) {
            match &cue.instruction {
                Instruction::Presentation {
                    presentation_id, ..
                } => {
                    println!("host T={at_ms}: present {presentation_id}");
                }
                Instruction::Effect { effect_id, .. } => {
                    // This example's game-native handler, outside the core.
                    assert_eq!(effect_id, "set_door_state");
                    door_broken = true;
                    println!("host T={at_ms}: door becomes broken");
                }
                Instruction::TraverseEdge { edge_id } => {
                    assert!(door_broken);
                    let next = graph.traverse_edge(edge_id, middle.epoch)?;
                    println!(
                        "host T={at_ms}: {} epoch={}",
                        next.cue_sheet.node_id, next.cue_sheet.epoch
                    );
                }
            }
        }
    }
    assert_eq!(graph.current().node_id, "CORRIDOR_SCENE");
    Ok(())
}
