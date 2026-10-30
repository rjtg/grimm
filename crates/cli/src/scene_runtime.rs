//! Concrete text game composition for M2. Rules remain in SceneEngine; this
//! host owns a renderer clock, bindings, input, media acknowledgements and files.
use crate::{FileError, runtime::terminal};
use grimm_adapters::cues as adapter;
use grimm_cli_renderer::{CliRenderer, cues::CuePlayer};
use grimm_engine::scene::SceneEngine;
use grimm_presentation::{PresentationCommand, RawInput, cues::Instruction};
use grimm_semantic::scene::{SceneSave, SceneStory};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs,
    io::{self, IsTerminal, Write},
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, RecvTimeoutError, Sender},
    thread,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TextBindings {
    narratives: BTreeMap<String, String>,
    actions: BTreeMap<String, String>,
}

pub fn parse(source: &str, yaml: &str) -> std::result::Result<SceneEngine, FileError> {
    let story: SceneStory = serde_yaml_ng::from_str(yaml)
        .map_err(|error| FileError(format!("{source}: invalid scene DSL: {error}")))?;
    SceneEngine::new(story).map_err(|error| FileError(format!("{source}: invalid scene: {error}")))
}

pub fn check(path: &Path) -> Result<()> {
    let engine = parse(&path.display().to_string(), &fs::read_to_string(path)?)?;
    println!(
        "{}: scene DSL, typed rules, identifiers, branches and explicit graph references valid. State-dependent solvability is not proven.",
        engine.title()
    );
    Ok(())
}

struct DriverClock {
    base: u64,
    origin: Instant,
    running: bool,
    paused: bool,
    manual: bool,
}
impl DriverClock {
    fn now(&self) -> u64 {
        if self.manual || !self.running || self.paused {
            self.base
        } else {
            self.base.saturating_add(
                u64::try_from(self.origin.elapsed().as_millis()).unwrap_or(u64::MAX),
            )
        }
    }
    fn reset(&mut self, player: &CuePlayer) {
        let clock = player.clock().expect("applied sheet");
        self.base = clock.elapsed_ms;
        self.running = clock.running;
        self.origin = Instant::now();
    }
}

struct Line {
    revision: u64,
    input: Option<RawInput>,
}
fn lines() -> (Sender<u64>, Receiver<io::Result<Line>>) {
    let (request, requests) = mpsc::channel();
    let (send, receive) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let mut reader = CliRenderer::new(io::stdin().lock(), io::sink());
        for revision in requests {
            let result = reader.read_input().map(|input| Line { revision, input });
            let done = result.as_ref().is_ok_and(|line| line.input.is_none());
            if send.send(result).is_err() || done {
                return;
            }
        }
    });
    (request, receive)
}

fn replace(engine: &SceneEngine, player: &mut CuePlayer, clock: &mut DriverClock) {
    player.replace(adapter::present(&engine.output()));
    clock.reset(player);
}

fn observe(
    engine: &mut SceneEngine,
    player: &mut CuePlayer,
    clock: &mut DriverClock,
) -> Result<()> {
    let at = clock.now();
    let sheet = engine.observe(player.revision(), at)?;
    if sheet.revision != player.revision() {
        replace(engine, player, clock);
    } else {
        player.set_time(at).map_err(io::Error::other)?;
    }
    Ok(())
}

/// Execute only presentation. Controls finish immediately; prose is started but
/// waits for a reader acknowledgement. Scheduled events never wait on prose.
fn pump<W: Write>(
    engine: &mut SceneEngine,
    player: &mut CuePlayer,
    clock: &mut DriverClock,
    text: &TextBindings,
    renderer: &mut CliRenderer<io::Empty, W>,
) -> Result<()> {
    for _ in 0..1024 {
        let Some(cue) = player.due().into_iter().next() else {
            return Ok(());
        };
        let was_running = player.clock().expect("clock").running;
        let mut candidate = player.clone();
        let feedback = candidate.started(&cue.cue_id).map_err(io::Error::other)?;
        let sheet = engine.cue_input(&adapter::feedback(feedback))?;
        if sheet.revision != player.revision() {
            replace(engine, player, clock);
            continue;
        }
        *player = candidate;
        if !was_running && player.clock().expect("clock").running {
            clock.reset(player);
        }
        match &cue.instruction {
            Instruction::Narrative { narrative_id } => {
                let prose = text.narratives.get(narrative_id).ok_or_else(|| {
                    io::Error::other(format!(
                        "missing text binding for narrative '{narrative_id}'"
                    ))
                })?;
                renderer.render(&[
                    PresentationCommand::Text(prose.clone()),
                    PresentationCommand::Text(
                        "Press Enter after reading. Available numbered actions may still be used."
                            .into(),
                    ),
                ])?;
            }
            _ => {
                let feedback = player.completed(&cue.cue_id).map_err(io::Error::other)?;
                engine.cue_input(&adapter::feedback(feedback))?;
            }
        }
    }
    Err("presentation execution budget exceeded".into())
}

fn show_menu<W: Write>(renderer: &mut CliRenderer<io::Empty, W>, path: &Path) -> Result<()> {
    renderer.render(&[PresentationCommand::Text(format!(
        "\nScene session menu — paused\n[1] Continue  [2] Save  [3] Load  [4] Quit\nCheckpoint: {}",
        path.display()
    ))])?;
    Ok(())
}

pub fn run(args: &[OsString]) -> Result<()> {
    if args.len() < 2 {
        return Err(
            "Usage: grimm-cli play-scene STORY.yaml TEXT.yaml [--manual-clock] [--load SAVE.json]"
                .into(),
        );
    }
    let story_path = Path::new(&args[0]);
    let mut engine = parse(
        &story_path.display().to_string(),
        &fs::read_to_string(story_path)?,
    )?;
    let text: TextBindings = serde_yaml_ng::from_str(&fs::read_to_string(&args[1])?)?;
    let mut manual = false;
    let mut load = None;
    let mut index = 2;
    while index < args.len() {
        if args[index] == "--manual-clock" {
            manual = true;
            index += 1;
        } else if args[index] == "--load" && index + 1 < args.len() {
            load = Some(PathBuf::from(&args[index + 1]));
            index += 2;
        } else {
            return Err("unknown/missing play-scene option".into());
        }
    }
    if let Some(path) = load {
        engine.restore(serde_json::from_str::<SceneSave>(&fs::read_to_string(
            path,
        )?)?)?;
    }
    let mut terminal = if io::stdin().is_terminal() && io::stdout().is_terminal() {
        Some(terminal::Terminal::enter()?)
    } else {
        None
    };
    let line_input = terminal.is_none().then(lines);
    let mut pending = false;
    let mut eof = false;
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    let mut clock = DriverClock {
        base: 0,
        origin: Instant::now(),
        running: false,
        paused: false,
        manual,
    };
    clock.reset(&player);
    let mut renderer = CliRenderer::new(
        io::empty(),
        terminal::Output {
            inner: io::stdout().lock(),
            raw: terminal.is_some(),
        },
    );
    renderer.render(&[PresentationCommand::Text(engine.title().into()), PresentationCommand::Text("Number keys choose; Enter completes text; Esc opens the session menu. Piped commands: :save PATH, :load PATH, :quit. Manual clock: :time MS or :advance MS; @qualified.action selects an exact offered ID.".into())])?;
    let save_path =
        PathBuf::from(".grimm-saves").join(format!("scene-{}.json", engine.save().story_id));
    let mut menu = false;
    let mut shown_actions = vec![];
    let mut shown_seconds = None;
    let mut shown_revision = None;
    loop {
        if !menu {
            observe(&mut engine, &mut player, &mut clock)?;
            if shown_revision != Some(player.revision()) {
                if let Some(terminal) = &mut terminal {
                    terminal.begin_prompt()?;
                }
                shown_revision = Some(player.revision());
                shown_actions.clear();
            }
            pump(&mut engine, &mut player, &mut clock, &text, &mut renderer)?;
            if player.actions() != shown_actions {
                let mut labels = vec![];
                for id in player.actions() {
                    labels.push(
                        text.actions
                            .get(id)
                            .ok_or_else(|| {
                                io::Error::other(format!("missing text binding for action '{id}'"))
                            })?
                            .clone(),
                    );
                }
                if !labels.is_empty() {
                    renderer.render(&[PresentationCommand::Choices(labels)])?;
                }
                shown_actions = player.actions().to_vec();
            }
            let view = player.clock().expect("clock");
            if view.visible
                && let Some(deadline) = view.deadline_ms
            {
                let remaining = deadline.saturating_sub(view.elapsed_ms);
                if shown_seconds != Some(remaining.div_ceil(1000)) {
                    renderer.render(&[PresentationCommand::Countdown {
                        remaining_ms: remaining,
                        label: "Time remaining".into(),
                    }])?;
                    shown_seconds = Some(remaining.div_ceil(1000));
                }
            }
            if engine.is_terminal() && !player.pending() {
                return Ok(());
            }
        }
        if let Some((request, _)) = &line_input
            && !eof
            && !pending
        {
            request.send(player.revision())?;
            pending = true;
        }
        let incoming = if let Some(terminal) = &mut terminal {
            let mut echo = terminal::Output {
                inner: io::stdout(),
                raw: true,
            };
            terminal
                .poll(
                    player.revision(),
                    Duration::from_millis(50),
                    player.actions().len() <= 9,
                    &mut echo,
                )?
                .map(|(revision, input)| Line {
                    revision,
                    input: Some(input),
                })
        } else if eof {
            if menu
                || !player.running_narratives().is_empty()
                || manual
                || player
                    .clock()
                    .is_none_or(|clock| clock.deadline_ms.is_none())
            {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(50));
            None
        } else {
            match line_input
                .as_ref()
                .expect("line transport")
                .1
                .recv_timeout(Duration::from_millis(50))
            {
                Ok(result) => {
                    pending = false;
                    Some(result?)
                }
                Err(RecvTimeoutError::Timeout) => None,
                Err(RecvTimeoutError::Disconnected) => {
                    pending = false;
                    Some(Line {
                        revision: player.revision(),
                        input: None,
                    })
                }
            }
        };
        let Some(line) = incoming else {
            continue;
        };
        let Some(raw) = line.input else {
            eof = true;
            continue;
        };
        if !menu {
            observe(&mut engine, &mut player, &mut clock)?;
        }
        if line.revision != player.revision() {
            renderer.render(&[PresentationCommand::Text(
                "Stale input ignored; follow the current prompt.".into(),
            )])?;
            continue;
        }
        let command = raw.line.trim();
        if command == ":quit" {
            return Ok(());
        }
        if command == ":menu" || (menu && command == ":menu-continue") {
            if !menu {
                clock.base = player.clock().expect("clock").elapsed_ms;
                clock.paused = true;
                menu = true;
                if let Some(terminal) = &mut terminal {
                    terminal.set_menu(true)?;
                }
                show_menu(&mut renderer, &save_path)?;
            } else {
                menu = false;
                clock.paused = false;
                clock.reset(&player);
                if let Some(terminal) = &mut terminal {
                    terminal.set_menu(false)?;
                }
                shown_actions.clear();
            }
            continue;
        }
        let path_command = if menu && command == ":menu-save" {
            Some((true, save_path.clone()))
        } else if menu && command == ":menu-load" {
            Some((false, save_path.clone()))
        } else if let Some(path) = command.strip_prefix(":save ") {
            Some((true, PathBuf::from(path.trim())))
        } else {
            command
                .strip_prefix(":load ")
                .map(|path| (false, PathBuf::from(path.trim())))
        };
        if let Some((save, path)) = path_command {
            let result: Result<()> = (|| {
                if save {
                    if let Some(parent) = path.parent()
                        && !parent.as_os_str().is_empty()
                    {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&path, serde_json::to_string_pretty(&engine.save())?)?;
                } else {
                    engine.restore(serde_json::from_str(&fs::read_to_string(&path)?)?)?;
                    player = CuePlayer::default();
                    replace(&engine, &mut player, &mut clock);
                    shown_revision = None;
                    shown_actions.clear();
                }
                Ok(())
            })();
            renderer.render(&[PresentationCommand::Text(match result {
                Ok(()) => format!(
                    "{} {}",
                    if save { "Saved" } else { "Loaded" },
                    path.display()
                ),
                Err(error) => format!("Session file error: {error}"),
            })])?;
            if menu {
                show_menu(&mut renderer, &save_path)?;
            }
            continue;
        }
        if menu {
            continue;
        }
        if manual {
            if let Some(value) = command
                .strip_prefix(":time ")
                .and_then(|value| value.parse::<u64>().ok())
            {
                clock.base = value;
                continue;
            }
            if let Some(value) = command
                .strip_prefix(":advance ")
                .and_then(|value| value.parse::<u64>().ok())
            {
                clock.base = clock
                    .base
                    .checked_add(value)
                    .ok_or("manual clock overflow")?;
                continue;
            }
        }
        if raw.line.ends_with('\n') && command.is_empty() {
            if let Some(id) = player.running_narratives().first() {
                let mut candidate = player.clone();
                let feedback = candidate.completed(id).map_err(io::Error::other)?;
                let sheet = engine.cue_input(&adapter::feedback(feedback))?;
                if sheet.revision == player.revision() {
                    player = candidate;
                } else {
                    replace(&engine, &mut player, &mut clock);
                }
            }
            continue;
        }
        let choice = if let Some(id) = command.strip_prefix('@') {
            player
                .actions()
                .iter()
                .position(|action| action == id.trim())
        } else {
            command
                .parse::<usize>()
                .ok()
                .and_then(|number| number.checked_sub(1))
        };
        let Some(action) = choice.and_then(|index| player.select(index)) else {
            renderer.render(&[PresentationCommand::Text(
                "Choose an available number, or Enter to finish text.".into(),
            )])?;
            continue;
        };
        match engine.apply(&adapter::action(action)) {
            Ok(_) => {
                replace(&engine, &mut player, &mut clock);
            }
            Err(error) => renderer.render(&[PresentationCommand::Text(format!(
                "Action rejected: {error}"
            ))])?,
        }
    }
}
