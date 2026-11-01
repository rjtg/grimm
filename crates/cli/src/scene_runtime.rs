//! Concrete text game composition. The synchronous coordinator is shared by the
//! interactive TUI and deterministic/noninteractive line transport.
use crate::text_resources::TextResources;
use crate::{FileError, scene_session::SceneSession};
use grimm_cli_renderer::CliRenderer;
use grimm_engine::scene::SceneEngine;
use grimm_engine::scene::lifecycle::LifecycleEngine;
use grimm_presentation::lifecycle::ScenePhase;
use grimm_presentation::{PresentationCommand, RawInput, cues::Instruction};
use grimm_semantic::{lifecycle::SceneConfiguration, scene::SceneStory};
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
    #[serde(deserialize_with = "crate::catalog::unique_map")]
    pub(crate) narratives: BTreeMap<String, String>,
    #[serde(deserialize_with = "crate::catalog::unique_map")]
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
        return Err("Usage: grimm-cli play-scene STORY.yaml TEXT.yaml [--manual-clock] [--manual-reading|--auto-reading] [--debug] [--load SAVE.json] [--lifecycle SCENES.yaml [--catalog CATALOG.yaml]]".into());
    }
    let story_path = Path::new(&args[0]);
    let story_source = fs::read_to_string(story_path)?;
    let text_source = fs::read_to_string(&args[1])?;
    let text: TextBindings = serde_yaml_ng::from_str(&text_source)?;
    let mut manual = false;
    let mut reading = None;
    let mut debug = false;
    let mut load = None;
    let mut lifecycle = None;
    let mut catalog_path = None;
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
        } else if args[index] == "--lifecycle" && index + 1 < args.len() {
            lifecycle = Some(PathBuf::from(&args[index + 1]));
            index += 2;
        } else if args[index] == "--catalog" && index + 1 < args.len() {
            catalog_path = Some(PathBuf::from(&args[index + 1]));
            index += 2;
        } else if args[index] == "--load" && index + 1 < args.len() {
            load = Some(PathBuf::from(&args[index + 1]));
            index += 2;
        } else {
            return Err("unknown/missing play-scene option".into());
        }
    }
    let interactive = !manual && io::stdin().is_terminal() && io::stdout().is_terminal();
    let automatic = reading.unwrap_or(interactive);
    let mut resources = None;
    let mut session = if let Some(path) = lifecycle {
        let configuration: SceneConfiguration =
            serde_yaml_ng::from_str(&fs::read_to_string(path)?)?;
        let catalog = catalog_path
            .as_ref()
            .map(|path| -> Result<crate::catalog::Catalog> {
                Ok(serde_yaml_ng::from_str(&fs::read_to_string(path)?)?)
            })
            .transpose()?;
        resources = Some(TextResources::from_sources(
            &story_source,
            &configuration,
            &text_source,
            catalog,
        )?);
        let engine = LifecycleEngine::new(serde_yaml_ng::from_str(&story_source)?, configuration)?;
        SceneSession::new_lifecycle(engine, automatic)?
    } else {
        if catalog_path.is_some() {
            return Err("--catalog requires --lifecycle".into());
        }
        SceneSession::new(
            parse(&story_path.display().to_string(), &story_source)?,
            automatic,
        )?
    };
    if let Some(path) = load {
        session.restore_json(&fs::read_to_string(path)?)?;
    }
    if session
        .scene()
        .is_some_and(|scene| scene.phase == ScenePhase::Prepare)
    {
        prepare_renderer(&mut session, &mut resources)?;
    }
    if interactive {
        return crate::scene_tui::run(session, text, resources, debug);
    }
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
    renderer.render(&[PresentationCommand::Text(session.engine().title().into()),PresentationCommand::Text("Numbered actions; Enter finishes text in manual reading mode. Commands: :menu, :save PATH, :load PATH, :quit. Lifecycle boundaries: :continue; manual outgoing media: :present MS. Manual clock: :time MS or :advance MS; @qualified.action selects an exact offered ID.".into())])?;
    let save_path = checkpoint_path(&session);
    let (request, receive) = lines();
    let mut pending = false;
    let mut eof = false;
    let mut menu = false;
    let mut shown_actions = vec![];
    let mut shown_seconds = None;
    let mut shown_revision = None;
    let mut last_frame = Instant::now();
    loop {
        let elapsed = last_frame
            .elapsed()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);
        last_frame = Instant::now();
        if !menu {
            if !manual {
                session.advance_presentation_by(elapsed)?;
            }
            clock.synchronize_after_rejection(&session);
            clock.observe(&mut session)?;
            for started in session.take_started_text() {
                if let Instruction::Narrative { narrative_id, .. } = started.cue.instruction {
                    let prose = narrative_text(&text, resources.as_ref(), &narrative_id)
                        .ok_or_else(|| {
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
                if session.scene().is_some_and(|scene| {
                    scene.phase == ScenePhase::Stable && scene.next_scene_id.is_some()
                }) {
                    renderer.render(&[PresentationCommand::Text("Scene ended — checkpoint available. Use :save PATH, :load PATH, or :continue for the next scene.".into())])?;
                }
            }
            if session.player().actions() != shown_actions {
                let labels = session
                    .player()
                    .actions()
                    .iter()
                    .map(|id| {
                        action_text(&text, resources.as_ref(), id)
                            .map(str::to_owned)
                            .ok_or_else(|| {
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
            if session.is_complete() && !session.player().pending() {
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
                || (!session.player().clock().expect("sheet").running
                    && !session.presentation_pending())
                || (session
                    .player()
                    .clock()
                    .expect("sheet")
                    .deadline_ms
                    .is_none()
                    && future.is_none()
                    && !session.presentation_pending())
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
        if command == ":continue" && !menu && session.scene().is_some() {
            prepare_renderer(&mut session, &mut resources)?;
            clock.reset(&session);
            shown_actions.clear();
            continue;
        }
        if command == ":menu" || (menu && command == ":menu-continue") {
            menu = !menu;
            clock.reset(&session);
            clock.paused = menu;
            if menu {
                renderer.render(&[PresentationCommand::Text(format!("Scene session menu — paused\n[1] Continue  [2] Save  [3] Load  [4] Quit\nCheckpoint: {}",save_path.display()))])?;
            } else {
                prepare_renderer(&mut session, &mut resources)?;
                clock.reset(&session);
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
                    let payload = session.save_json()?;
                    if let Some(parent) = path.parent()
                        && !parent.as_os_str().is_empty()
                    {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&path, payload)?;
                } else {
                    session.restore_json(&fs::read_to_string(&path)?)?;
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
                .strip_prefix(":present ")
                .and_then(|value| value.parse::<u64>().ok())
            {
                session.advance_presentation_by(value)?;
                clock.reset(&session);
                continue;
            }
            if let Some(value) = command
                .strip_prefix(":time ")
                .and_then(|value| value.parse::<u64>().ok())
            {
                if session
                    .scene()
                    .is_some_and(|scene| scene.phase != ScenePhase::Active)
                {
                    renderer.render(&[PresentationCommand::Text("Scene time is frozen; use :present MS for outgoing media or :continue at a checkpoint.".into())])?;
                    continue;
                }
                clock.base = value;
                continue;
            }
            if let Some(value) = command
                .strip_prefix(":advance ")
                .and_then(|value| value.parse::<u64>().ok())
            {
                if session
                    .scene()
                    .is_some_and(|scene| scene.phase != ScenePhase::Active)
                {
                    renderer.render(&[PresentationCommand::Text(
                        "Scene time is frozen; use :present MS or :continue.".into(),
                    )])?;
                    continue;
                }
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

pub(crate) fn narrative_text<'a>(
    text: &'a TextBindings,
    resources: Option<&'a TextResources>,
    id: &str,
) -> Option<&'a str> {
    match resources {
        Some(resources) => resources.narrative(id),
        None => text.narratives.get(id).map(String::as_str),
    }
}
pub(crate) fn action_text<'a>(
    text: &'a TextBindings,
    resources: Option<&'a TextResources>,
    id: &str,
) -> Option<&'a str> {
    match resources {
        Some(resources) => resources.action(id),
        None => text.actions.get(id).map(String::as_str),
    }
}
pub(crate) fn checkpoint_path(session: &SceneSession) -> PathBuf {
    PathBuf::from(".grimm-saves").join(format!(
        "scene-{}{}.json",
        session.engine().save().story_id,
        if session.scene().is_some() {
            "-boundary"
        } else {
            ""
        }
    ))
}
pub(crate) fn prepare_renderer(
    session: &mut SceneSession,
    resources: &mut Option<TextResources>,
) -> Result<()> {
    if !session.scene().is_some_and(|scene| {
        scene.phase == ScenePhase::Prepare
            || (scene.phase == ScenePhase::Stable && scene.next_scene_id.is_some())
    }) {
        return Ok(());
    }
    let scene = session.preparation_scene()?;
    let resources = resources
        .as_mut()
        .ok_or("lifecycle scene has no concrete resource registry")?;
    if let Err(error) = resources.prepare(&scene) {
        session.preparation_failed()?;
        return Err(error);
    }
    session.begin_prepared_scene()?;
    Ok(())
}
