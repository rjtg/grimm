//! Concrete text game composition. The synchronous coordinator is shared by the
//! interactive TUI and deterministic/noninteractive line transport.
use crate::{FileError, scene_session::SceneSession};
use grimm_cli_renderer::CliRenderer;
use grimm_engine::scene::SceneEngine;
use grimm_presentation::{PresentationCommand, RawInput, cues::Instruction};
use grimm_semantic::scene::{SceneSave, SceneStory};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs,
    io::{self, IsTerminal},
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, RecvTimeoutError, Sender},
    thread,
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TextBindings {
    pub(crate) narratives: BTreeMap<String, String>,
    pub(crate) actions: BTreeMap<String, String>,
}
pub fn parse(source: &str, yaml: &str) -> std::result::Result<SceneEngine, FileError> {
    let story: SceneStory = serde_yaml_ng::from_str(yaml)
        .map_err(|error| FileError(format!("{source}: invalid scene DSL: {error}")))?;
    SceneEngine::new(story).map_err(|error| FileError(format!("{source}: invalid scene: {error}")))
}
pub fn check(path: &Path) -> Result<()> {
    crate::authoring::run(false, &[path.as_os_str().to_owned()])
}
struct DriverClock {
    base: u64,
    origin: Instant,
    epoch: u64,
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
                self.origin
                    .elapsed()
                    .as_millis()
                    .try_into()
                    .unwrap_or(u64::MAX),
            )
        }
    }
    fn reset(&mut self, session: &SceneSession) {
        let clock = session.player().clock().expect("sheet");
        self.base = clock.elapsed_ms;
        self.epoch = clock.epoch;
        self.running = clock.running;
        self.origin = Instant::now();
    }
    fn observe(&mut self, session: &mut SceneSession) -> Result<()> {
        session.advance_to(self.now())?;
        let clock = session.player().clock().expect("sheet");
        if clock.epoch != self.epoch || clock.running != self.running {
            self.reset(session);
        }
        Ok(())
    }
}
#[derive(Clone)]
struct Prompt {
    revision: u64,
    actions: Vec<String>,
    reading: Option<String>,
}
struct Line {
    prompt: Prompt,
    input: Option<RawInput>,
}
fn lines() -> (Sender<Prompt>, Receiver<io::Result<Line>>) {
    let (request, requests) = mpsc::channel::<Prompt>();
    let (send, receive) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let mut reader = CliRenderer::new(io::stdin().lock(), io::sink());
        for prompt in requests {
            let result = reader.read_input().map(|input| Line { prompt, input });
            let done = result.as_ref().is_ok_and(|line| line.input.is_none());
            if send.send(result).is_err() || done {
                return;
            }
        }
    });
    (request, receive)
}

pub fn run(args: &[OsString]) -> Result<()> {
    if args.len() < 2 {
        return Err("Usage: grimm-cli play-scene STORY.yaml TEXT.yaml [--manual-clock] [--manual-reading|--auto-reading] [--debug] [--load SAVE.json]".into());
    }
    let story_path = Path::new(&args[0]);
    let mut engine = parse(
        &story_path.display().to_string(),
        &fs::read_to_string(story_path)?,
    )?;
    let text: TextBindings = serde_yaml_ng::from_str(&fs::read_to_string(&args[1])?)?;
    let mut manual = false;
    let mut reading = None;
    let mut debug = false;
    let mut load = None;
    let mut index = 2;
    while index < args.len() {
        if args[index] == "--manual-clock" {
            manual = true;
            index += 1;
        } else if args[index] == "--manual-reading" {
            reading = Some(false);
            index += 1;
        } else if args[index] == "--auto-reading" {
            reading = Some(true);
            index += 1;
        } else if args[index] == "--debug" {
            debug = true;
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
    let interactive = !manual && io::stdin().is_terminal() && io::stdout().is_terminal();
    let automatic = reading.unwrap_or(interactive);
    if interactive {
        return crate::scene_tui::run(engine, text, automatic, debug);
    }
    let mut session = SceneSession::new(engine, automatic)?;
    let mut clock = DriverClock {
        base: 0,
        origin: Instant::now(),
        epoch: 0,
        running: false,
        paused: false,
        manual,
    };
    clock.reset(&session);
    let mut renderer = CliRenderer::new(io::empty(), io::stdout().lock());
    renderer.render(&[PresentationCommand::Text(session.engine().title().into()),PresentationCommand::Text("Numbered actions; Enter finishes text in manual reading mode. Commands: :menu, :save PATH, :load PATH, :quit. Manual clock: :time MS or :advance MS; @qualified.action selects an exact offered ID.".into())])?;
    let save_path = PathBuf::from(".grimm-saves")
        .join(format!("scene-{}.json", session.engine().save().story_id));
    let (request, receive) = lines();
    let mut pending = false;
    let mut eof = false;
    let mut menu = false;
    let mut shown_actions = vec![];
    let mut shown_seconds = None;
    let mut shown_revision = None;
    loop {
        if !menu {
            clock.observe(&mut session)?;
            for started in session.take_started_text() {
                if let Instruction::Narrative { narrative_id, .. } = started.cue.instruction {
                    let prose = text.narratives.get(&narrative_id).ok_or_else(|| {
                        io::Error::other(format!(
                            "missing text binding for narrative '{narrative_id}'"
                        ))
                    })?;
                    renderer.render(&[PresentationCommand::Text(format!("\n{prose}"))])?;
                    if !automatic {
                        renderer.render(&[PresentationCommand::Text("Press Enter after reading. Available numbered actions may still be used.".into())])?;
                    }
                }
            }
            if shown_revision != Some(session.player().revision()) {
                shown_revision = Some(session.player().revision());
                shown_actions.clear();
            }
            if session.player().actions() != shown_actions {
                let labels = session
                    .player()
                    .actions()
                    .iter()
                    .map(|id| {
                        text.actions.get(id).cloned().ok_or_else(|| {
                            io::Error::other(format!("missing text binding for action '{id}'"))
                        })
                    })
                    .collect::<io::Result<Vec<_>>>()?;
                if !labels.is_empty() {
                    renderer.render(&[PresentationCommand::Choices(labels)])?;
                }
                shown_actions = session.player().actions().to_vec();
            }
            let view = session.player().clock().expect("sheet");
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
            } else {
                shown_seconds = None;
            }
            if session.engine().is_terminal() && !session.player().pending() {
                return Ok(());
            }
        }
        if !eof && !pending {
            request.send(Prompt {
                revision: session.player().revision(),
                actions: session.player().actions().to_vec(),
                reading: session.player().running_narratives().first().cloned(),
            })?;
            pending = true;
        }
        if eof {
            // EOF is never a text acknowledgement. Authored deadlines and
            // automatic scheduled media still run while there is no input.
            let future = if automatic {
                session.player().next_event_ms()
            } else {
                session.player().next_start_ms()
            };
            if manual
                || menu
                || !session.player().clock().expect("sheet").running
                || (session
                    .player()
                    .clock()
                    .expect("sheet")
                    .deadline_ms
                    .is_none()
                    && future.is_none())
            {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(25));
            continue;
        }
        let line = match receive.recv_timeout(Duration::from_millis(25)) {
            Ok(result) => {
                pending = false;
                result?
            }
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => {
                pending = false;
                eof = true;
                continue;
            }
        };
        let Some(raw) = line.input else {
            eof = true;
            continue;
        };
        if !menu {
            clock.observe(&mut session)?;
        }
        if line.prompt.revision != session.player().revision() {
            renderer.render(&[PresentationCommand::Text(
                "Stale input ignored; follow the current prompt.".into(),
            )])?;
            continue;
        }
        let command = raw.line.trim();
        if command == ":quit" || (menu && command == ":menu-quit") {
            return Ok(());
        }
        if command == ":menu" || (menu && command == ":menu-continue") {
            menu = !menu;
            clock.reset(&session);
            clock.paused = menu;
            if menu {
                renderer.render(&[PresentationCommand::Text(format!("Scene session menu — paused\n[1] Continue  [2] Save  [3] Load  [4] Quit\nCheckpoint: {}",save_path.display()))])?;
            } else {
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
                    fs::write(
                        &path,
                        serde_json::to_string_pretty(&session.engine().save())?,
                    )?;
                } else {
                    session.restore(serde_json::from_str(&fs::read_to_string(&path)?)?)?;
                    clock.reset(&session);
                    shown_actions.clear();
                    shown_revision = None;
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
        let at = clock.now();
        let blank = raw.line.ends_with('\n') && command.is_empty();
        let result = if blank {
            line.prompt
                .reading
                .as_deref()
                .map(|id| session.complete_text(id, line.prompt.revision, at))
        } else {
            let id = if let Some(id) = command.strip_prefix('@') {
                line.prompt
                    .actions
                    .iter()
                    .find(|offered| offered.as_str() == id.trim())
            } else {
                command
                    .parse::<usize>()
                    .ok()
                    .and_then(|number| number.checked_sub(1))
                    .and_then(|index| line.prompt.actions.get(index))
            };
            id.map(|id| session.choose(id, line.prompt.revision, at))
        };
        match result {
            Some(Ok(())) => clock.reset(&session),
            Some(Err(error)) => {
                clock.synchronize_after_rejection(&session);
                renderer.render(&[PresentationCommand::Text(format!(
                    "Action rejected: {error}"
                ))])?;
            }
            None if blank => {}
            None => renderer.render(&[PresentationCommand::Text(
                "Choose an available number, or Enter to finish text.".into(),
            )])?,
        }
    }
}
impl DriverClock {
    fn synchronize_after_rejection(&mut self, session: &SceneSession) {
        let current = session.player().clock().expect("sheet");
        if current.epoch != self.epoch || current.running != self.running {
            self.reset(session);
        }
    }
}
