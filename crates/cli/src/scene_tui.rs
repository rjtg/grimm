//! Interactive composition root: time and files stay outside the renderer/core.
use crate::{scene_runtime::TextBindings, scene_session::SceneSession};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use grimm_cli_renderer::scene_ui::{Focus, NarrativeEntry, SceneUi, SceneView, WorldRow};
use grimm_engine::scene::SceneEngine;
use grimm_presentation::cues::Instruction;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{Block, Borders, Clear, Paragraph},
};
use std::{
    collections::BTreeSet,
    fs, io,
    path::PathBuf,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
struct Screen;
impl Screen {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        let guard = Self;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(guard)
    }
}
impl Drop for Screen {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
        let _ = disable_raw_mode();
    }
}
struct Clock {
    base: u64,
    epoch: u64,
    running: bool,
    origin: Instant,
}
impl Clock {
    fn now(&self) -> u64 {
        if self.running {
            self.base.saturating_add(
                self.origin
                    .elapsed()
                    .as_millis()
                    .try_into()
                    .unwrap_or(u64::MAX),
            )
        } else {
            self.base
        }
    }
    fn anchor(&mut self, session: &SceneSession) {
        let clock = session.player().clock().expect("sheet");
        self.base = clock.elapsed_ms;
        self.epoch = clock.epoch;
        self.running = clock.running;
        self.origin = Instant::now();
    }
    fn synchronize(&mut self, session: &SceneSession) {
        let clock = session.player().clock().expect("sheet");
        if clock.epoch != self.epoch || clock.running != self.running {
            self.anchor(session);
        }
    }
}

pub(crate) fn run(
    engine: SceneEngine,
    text: TextBindings,
    automatic: bool,
    debug: bool,
) -> Result<()> {
    let mut session = SceneSession::new(engine, automatic)?;
    let mut ui = SceneUi {
        inspector: debug,
        ..SceneUi::default()
    };
    let mut entries = Vec::new();
    let mut seen = BTreeSet::new();
    let mut previous = session.engine().inspect_world();
    let mut changed = BTreeSet::new();
    let mut clock = Clock {
        base: 0,
        epoch: 0,
        running: false,
        origin: Instant::now(),
    };
    clock.anchor(&session);
    let save_path = PathBuf::from(".grimm-saves")
        .join(format!("scene-{}.json", session.engine().save().story_id));
    let _screen = Screen::enter()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut menu = false;
    let mut status = String::from("Playing");
    loop {
        if !menu {
            session.advance_to(clock.now())?;
            clock.synchronize(&session);
        }
        for started in session.take_started_text() {
            if !seen.insert((started.epoch, started.cue.cue_id.clone())) {
                continue;
            }
            if let Instruction::Narrative { narrative_id, .. } = started.cue.instruction {
                let prose = text.narratives.get(&narrative_id).ok_or_else(|| {
                    io::Error::other(format!(
                        "missing text binding for narrative '{narrative_id}'"
                    ))
                })?;
                entries.push(NarrativeEntry {
                    epoch: started.epoch,
                    cue_id: started.cue.cue_id,
                    text: prose.clone(),
                });
                // Bound renderer-owned scrollback independently of semantic history.
                if entries.len() > 512 {
                    entries.remove(0);
                }
                ui.narrative_scroll = u16::MAX;
            }
        }
        let snapshot = session.engine().inspect_world();
        if snapshot.context_id != previous.context_id
            || snapshot.globals != previous.globals
            || snapshot.locals != previous.locals
        {
            changed.clear();
            for (scope, values, old) in [
                ("global", &snapshot.globals, &previous.globals),
                ("local", &snapshot.locals, &previous.locals),
            ] {
                for (name, value) in values {
                    if old.get(name) != Some(value)
                        || (scope == "local" && snapshot.scene_epoch != previous.scene_epoch)
                    {
                        changed.insert((scope.to_owned(), name.clone()));
                    }
                }
            }
            previous = snapshot.clone();
        }
        let world: Vec<_> = [("global", &snapshot.globals), ("local", &snapshot.locals)]
            .into_iter()
            .flat_map(|(scope, values)| {
                values.iter().map(|(name, value)| WorldRow {
                    scope: scope.into(),
                    name: name.clone(),
                    value: format!("{value:?}"),
                    changed: changed.contains(&(scope.into(), name.clone())),
                })
            })
            .collect();
        let choices: Vec<_> = session
            .player()
            .actions()
            .iter()
            .map(|id| {
                text.actions
                    .get(id)
                    .map(|label| (id.clone(), label.clone()))
                    .ok_or_else(|| {
                        io::Error::other(format!("missing text binding for action '{id}'"))
                    })
            })
            .collect::<io::Result<_>>()?;
        let displayed_revision = session.player().revision();
        let running_text = session.player().running_narratives().first().cloned();
        let view = SceneView {
            title: session.engine().title(),
            context: &snapshot.context_id,
            clock: session.player().clock().expect("sheet"),
            status: if menu {
                "Paused"
            } else if session.engine().is_terminal() {
                "Ending · Esc to quit"
            } else {
                &status
            },
            narrative: &entries,
            choices: &choices,
            player: session.player(),
            world: &world,
        };
        terminal.draw(|frame| {
            ui.draw(frame, &view);
            if menu {
                let area = frame.area();
                let width = area.width.min(66);
                let height = area.height.min(8);
                let popup = Rect::new(
                    (area.width - width) / 2,
                    (area.height - height) / 2,
                    width,
                    height,
                );
                frame.render_widget(Clear, popup);
                frame.render_widget(
                    Paragraph::new(format!(
                        "[1] Continue\n[2] Save\n[3] Load\n[4] Quit\n{}\n{status}",
                        save_path.display()
                    ))
                    .block(
                        Block::default()
                            .title("SESSION · PAUSED")
                            .borders(Borders::ALL),
                    ),
                    popup,
                );
            }
        })?;
        if !event::poll(Duration::from_millis(33))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind == KeyEventKind::Release {
            continue;
        }
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return Ok(());
        }
        match key.code {
            KeyCode::F(4) => {
                let result: Result<()> = (|| {
                    let directory =
                        PathBuf::from(".grimm-captures").join(&session.engine().save().story_id);
                    fs::create_dir_all(&directory)?;
                    for (inspector, name) in [(false, "play.txt"), (true, "inspector.txt")] {
                        fs::write(
                            directory.join(name),
                            grimm_cli_renderer::scene_ui::capture(&ui, &view, inspector),
                        )?;
                    }
                    Ok(())
                })();
                status = match result {
                    Ok(()) => "Captured both layouts in .grimm-captures".into(),
                    Err(error) => format!("Capture failed: {error}"),
                };
                continue;
            }
            KeyCode::F(2) => {
                ui.inspector = !ui.inspector;
                if !ui.inspector {
                    ui.focus = Focus::Narrative;
                }
                continue;
            }
            KeyCode::Tab => {
                ui.next_focus();
                continue;
            }
            KeyCode::Up => {
                ui.scroll(-1, choices.len());
                continue;
            }
            KeyCode::Down => {
                ui.scroll(1, choices.len());
                continue;
            }
            KeyCode::PageUp => {
                ui.scroll(-10, choices.len());
                continue;
            }
            KeyCode::PageDown => {
                ui.scroll(10, choices.len());
                continue;
            }
            KeyCode::Char('?') => {
                ui.help = !ui.help;
                continue;
            }
            KeyCode::Char('[') if ui.inspector => {
                ui.history_back = ui.history_back.saturating_add(1);
                continue;
            }
            KeyCode::Char(']') if ui.inspector => {
                ui.history_back = ui.history_back.saturating_sub(1);
                continue;
            }
            _ => {}
        }
        // Freeze at selection time, drive all earlier milestones, and reject
        // lifecycle commands as well as actions if the displayed revision expired.
        let at = if menu {
            session.player().clock().expect("sheet").elapsed_ms
        } else {
            clock.now()
        };
        if !menu {
            session.advance_to(at)?;
            clock.synchronize(&session);
        }
        if displayed_revision != session.player().revision() {
            status = "Stale input ignored; follow the current scene.".into();
            continue;
        }
        if menu {
            match key.code {
                KeyCode::Esc | KeyCode::Char('1') => {
                    menu = false;
                    clock.anchor(&session);
                    status = "Playing".into();
                }
                KeyCode::Char('2') => {
                    let result: Result<()> = (|| {
                        fs::create_dir_all(save_path.parent().expect("save directory"))?;
                        fs::write(
                            &save_path,
                            serde_json::to_string_pretty(&session.engine().save())?,
                        )?;
                        Ok(())
                    })();
                    status = match result {
                        Ok(()) => "Saved".into(),
                        Err(e) => format!("Save failed: {e}"),
                    };
                }
                KeyCode::Char('3') => {
                    let result: Result<()> = (|| {
                        session.restore(serde_json::from_str(&fs::read_to_string(&save_path)?)?)?;
                        Ok(())
                    })();
                    status = match result {
                        Ok(()) => {
                            entries.clear();
                            seen.clear();
                            clock.anchor(&session);
                            "Loaded".into()
                        }
                        Err(e) => format!("Load failed: {e}"),
                    };
                }
                KeyCode::Char('4') => return Ok(()),
                _ => {}
            }
            continue;
        }
        match key.code {
            KeyCode::Esc => {
                menu = true;
                clock.anchor(&session);
            }
            KeyCode::Enter if ui.focus != Focus::Choices => {
                if let Some(id) = running_text {
                    if let Err(error) = session.complete_text(&id, displayed_revision, at) {
                        status = error.to_string();
                    }
                    clock.anchor(&session);
                }
            }
            KeyCode::Char('1'..='9') | KeyCode::Enter => {
                let index = match key.code {
                    KeyCode::Char(number) => number.to_digit(10).expect("digit") as usize - 1,
                    _ => ui.selected_choice,
                };
                if let Some((id, _)) = choices.get(index) {
                    status = match session.choose(id, displayed_revision, at) {
                        Ok(()) => "Playing".into(),
                        Err(error) => format!("Action rejected: {error}"),
                    };
                    clock.anchor(&session);
                }
            }
            _ => {}
        }
    }
}
