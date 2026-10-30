//! The shipped WHITEOUT rules, executed through real engine/adapter/renderer boundaries.
//! Time is supplied explicitly; no sleep or wall-clock dependency.
use grimm_adapters::cues as adapter;
use grimm_cli_renderer::cues::CuePlayer;
use grimm_engine::scene::SceneEngine;
use grimm_presentation::cues::Instruction;
use grimm_semantic::{scene::SceneStory, story::Value};

const STORY: &str = include_str!("../../../examples/whiteout/story.yaml");
const TEXT: &str = include_str!("../../../examples/whiteout/text.yaml");

struct Game {
    engine: SceneEngine,
    player: CuePlayer,
    narratives: Vec<String>,
}
impl Game {
    fn new() -> Self {
        let engine = SceneEngine::new(serde_yaml_ng::from_str(STORY).unwrap()).unwrap();
        let mut game = Self {
            engine,
            player: CuePlayer::default(),
            narratives: vec![],
        };
        game.replace();
        game.settle();
        game
    }
    fn replace(&mut self) {
        self.player.replace(adapter::present(&self.engine.output()));
    }
    fn now(&self) -> u64 {
        self.player.clock().unwrap().elapsed_ms
    }
    fn at(&mut self, time: u64) {
        let sheet = self.engine.observe(self.player.revision(), time).unwrap();
        if sheet.revision != self.player.revision() {
            self.replace();
        } else {
            self.player.set_time(time).unwrap();
        }
    }
    fn pump(&mut self, finish_text: bool) {
        for _ in 0..256 {
            let Some(cue) = self.player.due().into_iter().next() else {
                return;
            };
            let feedback = self.player.started(&cue.cue_id).unwrap();
            self.engine.cue_input(&adapter::feedback(feedback)).unwrap();
            if let Instruction::Narrative { narrative_id } = &cue.instruction {
                self.narratives.push(narrative_id.clone());
            }
            if finish_text || !matches!(cue.instruction, Instruction::Narrative { .. }) {
                self.complete(&cue.cue_id);
            }
        }
        panic!("unbounded presentation");
    }
    fn complete(&mut self, id: &str) {
        let feedback = self.player.completed(id).unwrap();
        self.engine.cue_input(&adapter::feedback(feedback)).unwrap();
    }
    fn settle(&mut self) {
        self.pump(true);
    }
    fn select(&mut self, id: &str) {
        let i = self
            .player
            .actions()
            .iter()
            .position(|a| a.ends_with(&format!(".{id}")))
            .unwrap_or_else(|| panic!("{id} absent from {:?}", self.player.actions()));
        let action = adapter::action(self.player.select(i).unwrap());
        self.engine
            .apply(&action)
            .unwrap_or_else(|e| panic!("{id}: {e}"));
        self.replace();
    }
    fn step(&mut self, id: &str) {
        let epoch = self.player.clock().unwrap().epoch;
        self.select(id);
        self.settle();
        if epoch == self.player.clock().unwrap().epoch {
            let end = self.engine.action_history().last().unwrap().logical_end_ms;
            self.at(end);
            self.settle();
        }
    }
    fn int(&self, name: &str) -> i64 {
        match self.engine.globals()[name] {
            Value::Int(v) => v,
            _ => panic!("not integer"),
        }
    }
    fn flag(&self, name: &str) -> bool {
        self.engine.globals()[name] == Value::Bool(true)
    }
    fn ending(&self, id: &str) {
        assert!(self.engine.is_terminal());
        assert_eq!(self.engine.globals()["ending"], Value::Text(id.into()));
    }
    fn prepared(thermal: bool, line: bool, treated: bool) -> Self {
        let mut g = Self::new();
        g.step("begin");
        if thermal {
            g.step("take_thermal_liner");
        }
        if treated {
            g.step("jonas_treated");
        }
        for id in [
            "take_copper_braid",
            "take_ceramic_mug",
            "take_spanner",
            "take_heat_pack",
            "log_read",
        ] {
            g.step(id);
        }
        if line {
            g.step("take_safety_line");
        }
        g.step("leave");
        g
    }
    fn generator(&mut self, lever: bool) {
        self.step("engineering");
        self.step("deploy_heat_pack");
        if lever {
            self.step("make_lever");
            self.step("use_lever");
        } else {
            while self.int("latch_damage") < 6 {
                self.step("open_generator_latch");
            }
        }
        assert!(
            !self
                .player
                .actions()
                .contains(&"generator.make_lever".into())
        );
        self.step("isolate_deicer");
        self.step("start_generator");
        self.step("return");
    }
    fn radio(&mut self, adapter: bool) {
        self.step("communications");
        self.step(if adapter {
            "make_adapter"
        } else {
            "hand_align"
        });
        self.step("send_signal");
        self.step("return");
    }
}

#[test]
fn full_crew_rescue_uses_force_crafting_and_preparation() {
    let mut g = Game::prepared(true, true, true);
    g.generator(false);
    g.radio(true);
    g.step("ridge");
    g.step("coordinate");
    g.ending("full_rescue");
    for name in [
        "alex_alive",
        "mara_alive",
        "jonas_alive",
        "lea_alive",
        "generator_done",
        "radio_done",
        "radio_auto",
    ] {
        assert!(g.flag(name), "{name}");
    }
    assert!(!g.flag("copper_braid") && !g.flag("ceramic_mug"));
    assert_eq!(g.int("first_task"), 1);
    assert!(g.narratives.contains(&"ending_full.truth".into()));
}

#[test]
fn lever_is_a_real_alternate_solution_with_a_later_radio_cost() {
    let mut g = Game::prepared(true, true, true);
    g.generator(true);
    assert!(g.flag("lever"));
    assert!(!g.flag("spanner") && !g.flag("copper_braid"));
    g.radio(false);
    assert!(!g.flag("radio_auto"));
    g.step("ridge");
    g.step("coordinate");
    g.ending("full_rescue");
}

#[test]
fn choosing_jonas_over_the_manual_radio_operator_has_a_named_cost() {
    let mut g = Game::prepared(true, false, true);
    g.generator(true);
    g.radio(false);
    g.step("ridge");
    g.step("help_jonas");
    g.ending("partial_survival");
    assert!(g.flag("jonas_alive") && g.flag("alex_alive") && g.flag("mara_alive"));
    assert!(!g.flag("lea_alive"));
    assert!(g.narratives.contains(&"ending_partial.lea".into()));
}

#[test]
fn radio_first_delay_changes_the_later_same_rescue_decision() {
    for radio_first in [false, true] {
        let mut g = Game::prepared(true, false, true);
        if radio_first {
            g.radio(false);
            g.generator(false);
        } else {
            g.generator(false);
            g.radio(false);
        }
        g.step("ridge");
        g.step("help_lea");
        g.ending(if radio_first {
            "partial_survival"
        } else {
            "full_rescue"
        });
        assert_eq!(g.flag("jonas_alive"), !radio_first);
        assert!(g.flag("lea_alive"));
    }
}

#[test]
fn storm_timeout_has_a_catastrophic_canonical_ending() {
    let mut g = Game::prepared(true, true, true);
    g.generator(false);
    g.radio(true);
    g.step("ridge");
    g.at(45000);
    g.settle();
    g.ending("station_lost");
    for name in ["alex_alive", "mara_alive", "jonas_alive", "lea_alive"] {
        assert!(!g.flag(name));
    }
}

#[test]
fn m3_index_and_every_text_binding_are_complete_and_renderer_independent() {
    let report = grimm_cli::authoring::inspect("whiteout.yaml", STORY);
    assert!(!report.has_errors(), "{:?}", report.diagnostics);
    let index = report.index.unwrap();
    assert_eq!(index.contexts.len(), 9);
    assert_eq!(index.contexts.iter().filter(|c| c.terminal).count(), 3);
    let text: serde_yaml_ng::Value = serde_yaml_ng::from_str(TEXT).unwrap();
    for c in &index.contexts {
        for a in &c.actions {
            assert!(text["actions"][&a.id].as_str().is_some(), "{}", a.id);
        }
        for n in &c.narratives {
            assert!(
                text["narratives"][&n.narrative_id].as_str().is_some(),
                "{}",
                n.narrative_id
            );
        }
    }
    // Normal travel is instantaneous. Interrupted-work timeout transitions
    // separately reserve effective_ms credit; see the cross-epoch regression.
    let story: SceneStory = serde_yaml_ng::from_str(STORY).unwrap();
    for c in &story.contexts {
        for a in &c.actions {
            if a.outcomes
                .iter()
                .chain(&a.interrupted_outcomes)
                .any(|b| b.outcome.goto.is_some())
            {
                assert_eq!(a.duration_ms, 0);
            }
        }
    }
    let opening = text["narratives"]["blackout.arrival"].as_str().unwrap();
    assert!(opening.split("\n\n").count() >= 3);
}

#[test]
fn elapsed_cold_is_booked_once_including_costs_rate_changes_and_departure() {
    let mut g = Game::new();
    g.step("begin");
    g.at(5000);
    g.step("take_thermal_liner");
    assert_eq!(g.int("cold_exposure"), 6000); // 5s thinking + 1s clothing at old rate.
    assert_eq!(g.int("accounted_ms"), 6000);
    assert_eq!(g.int("exposure_rate"), 0);
    g.step("take_spanner");
    assert_eq!(g.int("cold_exposure"), 6000);
    g.step("leave");
    assert_eq!(g.int("accounted_ms"), 0);
    g.at(5000);
    g.step("engineering");
    assert_eq!(g.int("cold_exposure"), 6000);
    g.at(8000);
    g.step("open_generator_latch");
    assert_eq!(g.now(), 11000);
    assert_eq!(g.int("accounted_ms"), 11000);
    assert_eq!(g.int("cold_exposure"), 17000);
    g.at(15000);
    g.step("abandon");
    assert_eq!(g.int("cold_exposure"), 21000); // Only the unbooked 11s..15s interval.
    assert_eq!(g.int("accounted_ms"), 0);
    assert_eq!(g.now(), 0);
    g.at(5000);
    g.step("communications");
    assert_eq!(g.int("cold_exposure"), 21000); // New warm-hall epoch, no old timestamp subtraction.
    assert_eq!(g.int("accounted_ms"), 0);
}

#[test]
fn slower_deliberation_changes_latch_progress_and_next_authored_duration() {
    let mut cold = vec![];
    for time in [8000, 40000] {
        let mut g = Game::prepared(true, true, true);
        g.step("engineering");
        g.step("deploy_heat_pack");
        g.at(time);
        g.step("open_generator_latch");
        cold.push(g.int("cold_exposure"));
        assert_eq!(g.int("latch_damage"), if time == 8000 { 3 } else { 1 });
        if time == 40000 {
            g.step("work_latch_with_numb_hands");
            let record = g.engine.action_history().last().unwrap();
            assert_eq!(record.effective_ms, 5000);
            assert_eq!(g.int("latch_damage"), 2);
        }
    }
    assert_eq!(cold, [12000, 44000]);
}

#[test]
fn an_offered_transmission_can_fail_from_deliberation_without_protocol_rejection() {
    for (time, success) in [(3000, true), (40000, false)] {
        let mut g = Game::prepared(false, false, true);
        g.step("communications");
        g.step("make_adapter");
        let before = g.int("cold_exposure");
        assert_eq!(before, 15000);
        assert!(g.player.actions().contains(&"radio.send_signal".into()));
        g.at(time);
        assert_eq!(g.int("cold_exposure"), before); // No background stat tick.
        g.step("send_signal");
        assert_eq!(g.flag("radio_done"), success);
        assert_eq!(g.int("cold_exposure"), if success { 21000 } else { 95000 });
        if !success {
            assert!(g.narratives.contains(&"radio.cold_transmit_failure".into()));
        }
    }
}

#[test]
fn restored_heating_changes_future_rate_and_can_recover_a_fine_repair() {
    let mut g = Game::prepared(true, true, true);
    g.generator(false);
    assert_eq!(g.int("cold_exposure"), 14000);
    assert_eq!(g.int("indoor_rate"), 0);
    assert_eq!(g.int("exposure_rate"), 0);
    g.at(90000);
    g.step("warm_hands");
    assert_eq!(g.int("cold_exposure"), 0); // Warming, not a per-second simulation.
    g.radio(true);
    g.step("ridge");
    assert_eq!(g.int("exposure_rate"), 1);
    g.at(5000);
    g.step("coordinate");
    g.ending("full_rescue");
    assert_eq!(g.int("cold_exposure"), 5000);
}

#[test]
fn real_generator_warning_overlaps_latch_media_and_never_restarts_the_timer() {
    let mut g = Game::prepared(true, true, true);
    g.step("engineering");
    assert!(!g.player.clock().unwrap().visible);
    assert_eq!(g.player.clock().unwrap().deadline_ms, Some(30000));
    g.at(8000);
    g.select("open_generator_latch");
    g.pump(false);
    assert_eq!(g.now(), 8000); // No jump by the 3s authored action cost.
    let latch = g.player.running_narratives().first().unwrap().clone();
    g.at(10000);
    g.pump(false);
    assert!(g.player.running_narratives().contains(&latch));
    assert!(g.narratives.contains(&"generator.radio_warning".into()));
    assert!(g.player.clock().unwrap().visible);
    assert_eq!(
        g.player.clock().unwrap().deadline_ms.unwrap() - g.now(),
        20000
    );
    assert!(g.player.actions().is_empty());
    let warning = g
        .player
        .running_narratives()
        .into_iter()
        .find(|id| id.contains("radio_warning"))
        .unwrap();
    g.complete(&warning);
    g.at(11000);
    g.pump(false);
    assert!(!g.player.actions().is_empty());
    g.at(12000);
    g.select("isolate_deicer");
    g.pump(false);
    assert_eq!(g.player.clock().unwrap().deadline_ms, Some(30000));
    assert!(
        !g.engine
            .output()
            .cues
            .iter()
            .any(|c| c.cue_id == "generator.radio_warning")
    );
    assert_eq!(
        g.narratives
            .iter()
            .filter(|id| id.as_str() == "generator.radio_warning")
            .count(),
        1
    );
    let isolated = g
        .player
        .running_narratives()
        .into_iter()
        .find(|id| id.contains(".isolated"))
        .unwrap();
    g.complete(&isolated);
    g.at(14000);
    g.pump(false);
    assert!(g.player.running_narratives().contains(&latch));
    g.select("abandon");
    assert!(
        !g.engine
            .output()
            .cues
            .iter()
            .any(|c| c.cue_id.starts_with("generator."))
    );
}

#[test]
fn startup_fine_cues_lock_input_until_all_three_beats_and_the_cost_finish() {
    let mut g = Game::prepared(true, true, true);
    g.step("engineering");
    g.step("deploy_heat_pack");
    g.step("open_generator_latch");
    g.step("open_generator_latch");
    g.step("isolate_deicer");
    let start = g.now();
    g.select("start_generator");
    g.pump(false);
    assert!(g.flag("generator_done"));
    assert!(g.player.actions().is_empty());
    let snapshot = g.engine.save();
    assert!(
        g.engine
            .apply(&grimm_semantic::scene::SceneAction {
                revision: g.player.revision(),
                action_id: "generator.return".into(),
                scene_time_ms: start
            })
            .is_err()
    );
    assert_eq!(g.engine.save(), snapshot);
    g.at(start + 4000);
    for (i, beat) in ["startup_prime", "startup_ignition", "startup_heat"]
        .iter()
        .enumerate()
    {
        let id = g
            .player
            .running_narratives()
            .into_iter()
            .find(|id| id.contains(beat))
            .unwrap();
        assert!(g.player.actions().is_empty());
        g.complete(&id);
        g.pump(false);
        if i < 2 {
            assert!(g.player.actions().is_empty());
        }
    }
    assert!(g.player.actions().contains(&"generator.return".into()));
}

#[test]
fn deadline_cutoff_accounts_only_effective_time_and_does_not_invent_power() {
    for (selected, success) in [(26000, true), (27000, false)] {
        let mut g = Game::prepared(true, true, true);
        g.step("engineering");
        g.step("open_generator_latch");
        g.step("open_generator_latch");
        g.step("isolate_deicer");
        g.at(selected);
        g.step("start_generator");
        assert_eq!(g.flag("generator_done"), success);
        assert!(g.flag("engineering_lost"));
        assert_eq!(g.int("cold_exposure"), 31000);
        assert_eq!(g.int("accounted_ms"), if success { 0 } else { 3000 });
        if !success {
            g.at(3000);
            g.settle();
        }
        assert!(!g.player.actions().is_empty());
        let record = g
            .engine
            .action_history()
            .iter()
            .find(|a| a.action_id == "generator.start_generator")
            .unwrap();
        assert_eq!(record.logical_end_ms, 30000);
        assert_eq!(record.effective_ms, if success { 4000 } else { 3000 });
        assert_eq!(record.interrupted, !success);
        assert!(
            g.player
                .actions()
                .iter()
                .all(|id| id.starts_with("dispatch."))
        );
    }
}

#[test]
fn save_at_dispatch_then_restore_has_identical_world_and_ending_continuation() {
    let mut original = Game::prepared(true, true, true);
    original.generator(false);
    let json = serde_json::to_string(&original.engine.save()).unwrap();
    let mut restored = Game::new();
    restored
        .engine
        .restore(serde_json::from_str(&json).unwrap())
        .unwrap();
    restored.player = CuePlayer::default();
    restored.replace();
    restored.settle();
    assert_eq!(original.engine.globals(), restored.engine.globals());
    assert!(!restored.narratives.iter().any(|id| id.contains("startup_")));
    for g in [&mut original, &mut restored] {
        g.radio(true);
        g.step("ridge");
        g.step("coordinate");
        g.ending("full_rescue");
    }
    assert_eq!(original.engine.globals(), restored.engine.globals());
    assert_eq!(
        original.engine.action_history(),
        restored.engine.action_history()
    );
}

fn run_cli(script: &str, load: Option<&std::path::Path>) -> std::process::Output {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let mut command = Command::new(env!("CARGO_BIN_EXE_grimm-cli"));
    command.args([
        "play-scene",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../examples/whiteout/story.yaml"
        ),
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../examples/whiteout/text.yaml"
        ),
        "--manual-clock",
    ]);
    if let Some(path) = load {
        command.arg("--load").arg(path);
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn actual_cli_reaches_all_three_endings_and_the_lever_rescue() {
    for (script, ending) in [
        (
            include_str!("../../../examples/whiteout/playthroughs/full-rescue.input"),
            "ENDING / FOUR NAMES ON THE MANIFEST",
        ),
        (
            include_str!("../../../examples/whiteout/playthroughs/partial-survival.input"),
            "ENDING / THE EMPTY PLACE",
        ),
        (
            include_str!("../../../examples/whiteout/playthroughs/station-lost.input"),
            "ENDING / WHITEOUT",
        ),
        (
            include_str!("../../../examples/whiteout/playthroughs/lever-rescue.input"),
            "ENDING / FOUR NAMES ON THE MANIFEST",
        ),
    ] {
        let output = run_cli(script, None);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains(ending), "{text}");
        for unwanted in [
            "Action rejected",
            "Choose an available",
            "cold_exposure",
            "accounted_ms",
            "global.",
        ] {
            assert!(!text.contains(unwanted), "{unwanted}: {text}");
        }
    }
}

#[test]
fn actual_cli_checkpoint_load_resumes_the_remaining_adventure() {
    let script = include_str!("../../../examples/whiteout/playthroughs/full-rescue.input");
    let (prefix, suffix) = script.split_once("@dispatch.communications").unwrap();
    let path = std::env::temp_dir().join(format!("grimm-whiteout-{}.json", std::process::id()));
    let saved = run_cli(&format!("{prefix}:save {}\n:quit\n", path.display()), None);
    assert!(saved.status.success());
    assert!(String::from_utf8_lossy(&saved.stdout).contains("Saved"));
    let output = run_cli(&format!("@dispatch.communications{suffix}"), Some(&path));
    std::fs::remove_file(path).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        text.contains("ENDING / FOUR NAMES ON THE MANIFEST"),
        "{text}"
    );
    assert!(!text.contains("You hold the primer"));
}

#[test]
fn preparation_feedback_and_pending_radio_cues_follow_the_changed_world() {
    for automatic in [false, true] {
        let mut g = Game::prepared(true, true, true);
        g.generator(false);
        g.step("communications");
        g.step(if automatic {
            "make_adapter"
        } else {
            "hand_align"
        });
        g.step("send_signal");
        g.at(15000);
        g.settle();
        assert_eq!(
            g.narratives.contains(&"radio.gust_warning".into()),
            !automatic
        );
    }
}

#[test]
fn treating_jonas_changes_cumulative_progress_and_attempt_count() {
    for treated in [false, true] {
        let mut g = Game::prepared(true, true, treated);
        g.step("engineering");
        let mut progress = vec![];
        while g.int("latch_damage") < 6 {
            g.step("open_generator_latch");
            progress.push(g.int("latch_damage"));
        }
        assert_eq!(progress, if treated { vec![3, 6] } else { vec![2, 4, 6] });
        assert!(g.narratives.contains(&"whiteout.latch_crack".into()));
        assert!(g.narratives.contains(&"whiteout.latch_open".into()));
        if !treated {
            assert!(g.narratives.contains(&"whiteout.latch_nearly".into()));
        }
    }
}

#[test]
fn safety_line_preparation_can_also_save_everyone_after_radio_first() {
    let mut g = Game::prepared(true, true, true);
    g.radio(false);
    g.generator(false);
    assert_eq!(g.int("first_task"), 2);
    assert!(!g.flag("radio_auto"));
    g.step("ridge");
    g.step("coordinate");
    g.ending("full_rescue");
    assert!(g.flag("jonas_alive") && g.flag("lea_alive"));
}

#[test]
fn cold_from_slow_engineering_changes_a_later_radio_outcome_and_can_be_recovered() {
    for slow in [false, true] {
        let mut g = Game::prepared(true, true, true);
        g.step("engineering");
        g.step("deploy_heat_pack");
        g.step("open_generator_latch");
        g.step("open_generator_latch");
        g.step("isolate_deicer");
        g.at(if slow { 85000 } else { 9000 });
        g.step("start_generator");
        g.step("return");
        assert_eq!(g.int("accounted_ms"), 0);
        assert_eq!(g.int("cold_exposure"), if slow { 90000 } else { 14000 });
        g.step("communications");
        g.step("make_adapter");
        g.step("send_signal");
        assert_eq!(g.flag("radio_done"), !slow);
        if slow {
            assert!(g.narratives.contains(&"radio.cold_transmit_failure".into()));
            g.step("warm_hands");
            g.step("send_signal");
            assert!(g.flag("radio_done"));
        }
        g.step("return");
        g.step("ridge");
        g.step("coordinate");
        g.ending("full_rescue");
    }
}

#[test]
fn interrupted_work_credit_is_not_charged_again_in_the_new_scene() {
    for new_time in [1000, 5000] {
        let mut g = Game::prepared(false, true, true);
        assert_eq!(g.int("cold_exposure"), 10000);
        g.step("engineering");
        g.at(29000);
        g.step("open_generator_latch");
        assert!(g.flag("engineering_lost"));
        assert_eq!(g.int("latch_damage"), 0);
        assert_eq!(g.int("cold_exposure"), 70000); // Old rate 2 through the 30s deadline.
        assert_eq!(g.int("accounted_ms"), 1000); // Credit only the effective 1s, not old T=30s.
        assert_eq!(g.now(), 0);
        assert!(g.player.actions().is_empty());
        g.at(new_time);
        g.settle();
        g.step("communications");
        assert_eq!(g.int("cold_exposure"), 70000 + (new_time as i64 - 1000));
        assert_eq!(g.int("accounted_ms"), 0); // Normal departure starts another fresh epoch.
    }
    // An idle expiry carries no work credit: all time after its reset is new.
    let mut idle = Game::prepared(false, true, true);
    idle.step("engineering");
    idle.at(30000);
    idle.settle();
    assert_eq!(idle.int("accounted_ms"), 0);
    idle.at(1000);
    idle.step("communications");
    assert_eq!(idle.int("cold_exposure"), 71000);
}
