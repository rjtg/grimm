//! Real-time composition root. The sole clock lives here, never in the engine.
use crate::{read_save, write_save};
use grimm_adapters::{InputAdapter, OutputAdapter, TextAdapter, lifecycle_input};
use grimm_cli_renderer::CliRenderer;
use grimm_engine::Engine;
use grimm_presentation::{PresentationCommand, RawInput};
use grimm_semantic::flow::FlowPhase;
use grimm_semantic::time::{DecisionPhase, TimeEvent, TimeInput};
use std::{
    io,
    path::Path,
    sync::mpsc::{self, Receiver, RecvTimeoutError, Sender},
    thread,
    time::{Duration, Instant},
};

type HostResult<T> = Result<T, Box<dyn std::error::Error>>;

enum Message {
    Line {
        revision: u64,
        submitted_at: Instant,
        input: RawInput,
    },
    Eof,
    Error(io::Error),
}

fn reader() -> (Sender<u64>, Receiver<Message>) {
    let (request, requests) = mpsc::channel::<u64>();
    let (sender, receiver) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let mut input = CliRenderer::new(io::stdin().lock(), io::sink());
        for revision in requests {
            let message = match input.read_input() {
                Ok(Some(input)) => Message::Line {
                    revision,
                    submitted_at: Instant::now(),
                    input,
                },
                Ok(None) => {
                    let _ = sender.send(Message::Eof);
                    return;
                }
                Err(error) => Message::Error(error),
            };
            if sender.send(message).is_err() {
                return;
            }
        }
    });
    (request, receiver)
}

fn input(engine: &mut Engine, event: TimeEvent) -> HostResult<()> {
    engine.time_input(&TimeInput {
        revision: engine.output().revision,
        event,
    })?;
    Ok(())
}

/// One baseline per explicitly activated/resumed interaction; no second clock.
struct Observation {
    origin: Instant,
    last_total_ms: u64,
}
impl Observation {
    fn observe_at(&mut self, engine: &mut Engine, at: Instant) -> HostResult<()> {
        let total_ms = u64::try_from(at.saturating_duration_since(self.origin).as_millis())
            .unwrap_or(u64::MAX);
        if total_ms > self.last_total_ms {
            input(engine, TimeEvent::Elapsed { total_ms })?;
            self.last_total_ms = total_ms;
        }
        Ok(())
    }
}

pub fn play(title: &str, mut engine: Engine) -> HostResult<()> {
    // An embedding host may transfer an already active engine. Freeze that old
    // observation segment before establishing this driver's readiness baseline.
    if engine
        .output()
        .decision
        .as_ref()
        .is_some_and(|decision| decision.phase == DecisionPhase::Active)
    {
        input(&mut engine, TimeEvent::Pause)?;
        input(&mut engine, TimeEvent::Resume)?;
    }
    let (request, receiver) = reader();
    let mut pending_read = None;
    let mut renderer = CliRenderer::new(io::empty(), io::stdout().lock());
    let mut adapter = TextAdapter::default();
    renderer.render(&[PresentationCommand::Text(title.into()), PresentationCommand::Text("Commands: :pause, :resume, :save PATH, :load PATH, :quit. Required reading waits for Enter; countdown starts when the decision choices are available.".into())])?;
    let mut rendered_revision = None;
    let mut clock: Option<Observation> = None;
    let mut shown_seconds = None;
    let mut eof = false;
    loop {
        let output = engine.output();
        if rendered_revision != Some(output.revision) {
            let needs_ack = pending_read.is_some_and(|revision| revision != output.revision);
            renderer.render(&adapter.present(&output))?;
            if needs_ack && output.ending.is_none() {
                renderer.render(&[PresentationCommand::Text("The scene changed while the previous prompt was waiting. Press Enter to acknowledge; old input will not choose here. New deadlines stay frozen until acknowledgement.".into())])?;
            }
            shown_seconds = output
                .decision
                .as_ref()
                .map(|decision| decision.remaining_ms.div_ceil(1000));
            if !needs_ack
                && output.flow.as_ref().is_some_and(|flow| {
                    !flow.paused && flow.phase == FlowPhase::AwaitingInteraction
                })
            {
                let feedback = renderer
                    .interaction_ready()
                    .expect("adapter requested interaction availability");
                let origin = Instant::now();
                engine.flow_input(&lifecycle_input(feedback))?;
                clock = output.decision.as_ref().map(|_| Observation {
                    origin,
                    last_total_ms: 0,
                });
            } else if output.flow.is_none()
                && output
                    .decision
                    .as_ref()
                    .is_some_and(|decision| decision.phase == DecisionPhase::AwaitingReady)
                && !needs_ack
            {
                // Full prompt flush above is the readiness acknowledgement.
                let origin = Instant::now();
                input(&mut engine, TimeEvent::Ready)?;
                clock = Some(Observation {
                    origin,
                    last_total_ms: 0,
                });
            } else {
                clock = None;
            }
            rendered_revision = Some(output.revision);
        }
        let output = engine.output();
        if output.ending.is_some() {
            return Ok(());
        }
        if output.decision.is_none() && output.offers.is_empty() {
            engine.validate_current_state()?;
        }
        if !eof && pending_read.is_none() {
            request.send(output.revision)?;
            pending_read = Some(output.revision);
        }
        let active = output
            .decision
            .as_ref()
            .filter(|decision| decision.phase == DecisionPhase::Active);
        let wait = active.map(|decision| Duration::from_millis(decision.remaining_ms.min(250)));
        let message = if eof {
            let Some(wait) = wait else {
                return Ok(());
            };
            thread::sleep(wait); // host only; EOF still permits authored timeout
            None
        } else if let Some(wait) = wait {
            match receiver.recv_timeout(wait) {
                Ok(message) => Some(message),
                Err(RecvTimeoutError::Timeout) => None,
                Err(RecvTimeoutError::Disconnected) => Some(Message::Eof),
            }
        } else {
            Some(receiver.recv().unwrap_or(Message::Eof))
        };
        // Elapsed time is observed before any action/command at this instant.
        if active.is_some() {
            let at = match &message {
                Some(Message::Line {
                    revision,
                    submitted_at,
                    ..
                }) if *revision == output.revision => *submitted_at,
                _ => Instant::now(),
            };
            clock
                .as_mut()
                .expect("readiness established host baseline")
                .observe_at(&mut engine, at)?;
        }
        let current = engine.output();
        if current.revision == output.revision {
            let seconds = current
                .decision
                .as_ref()
                .map(|decision| decision.remaining_ms.div_ceil(1000));
            if seconds != shown_seconds {
                renderer.render(&adapter.present_deadline(&current))?;
                shown_seconds = seconds;
            }
        }
        let Some(message) = message else {
            continue;
        };
        pending_read = None;
        let (revision, raw) = match message {
            Message::Eof => {
                eof = true;
                // EOF releases an obsolete read's acknowledgement barrier.
                // Re-present and activate any successor deadline; EOF is not
                // a player action and must not abandon authored timeouts.
                if engine
                    .output()
                    .decision
                    .as_ref()
                    .is_some_and(|decision| decision.phase == DecisionPhase::AwaitingReady)
                {
                    rendered_revision = None;
                }
                continue;
            }
            Message::Error(error) => return Err(error.into()),
            Message::Line {
                revision, input, ..
            } => (revision, input),
        };
        if revision != engine.output().revision {
            renderer.render(&[PresentationCommand::Text(
                "Input from the previous prompt was ignored. Old read cleared; follow the current prompt.".into(),
            )])?;
            rendered_revision = None;
            continue;
        }
        let line = raw.line.trim();
        if line == ":quit" {
            return Ok(());
        }
        // Lifecycle/file commands, like choices, belong to the read's revision.
        // A command queued for an expired prompt cannot control its successor.
        if line.starts_with(':') {
            let (command, path) = line
                .split_once(char::is_whitespace)
                .map_or((line, ""), |(command, path)| (command, path.trim()));
            let result: HostResult<String> = match (command, path) {
                (":pause", "") => input(&mut engine, TimeEvent::Pause)
                    .map(|()| "Paused. Use :resume or :save PATH.".into()),
                (":resume", "") => input(&mut engine, TimeEvent::Resume)
                    .map(|()| "Resuming after prompt readiness.".into()),
                (":save", "") | (":load", "") => {
                    Ok("Provide a file path after :save or :load.".into())
                }
                (":save", path) => write_save(Path::new(path), &engine)
                    .map(|()| format!("Saved to {path}."))
                    .map_err(Into::into),
                (":load", path) => {
                    read_save(Path::new(path))
                        .map_err(Into::into)
                        .and_then(|save| {
                            engine
                                .restore(save)
                                .map(|_| format!("Loaded {path}."))
                                .map_err(Into::into)
                        })
                }
                _ => Ok(
                    "Unknown command. Use :pause, :resume, :save PATH, :load PATH, or :quit."
                        .into(),
                ),
            };
            renderer.render(&[PresentationCommand::Text(
                result.unwrap_or_else(|error| error.to_string()),
            )])?;
            if engine
                .output()
                .decision
                .as_ref()
                .is_some_and(|decision| decision.phase == DecisionPhase::AwaitingReady)
            {
                rendered_revision = None;
            }
            continue;
        }
        if engine
            .output()
            .flow
            .as_ref()
            .is_some_and(|flow| flow.phase == FlowPhase::Presenting && !flow.paused)
        {
            if let Some(feedback) = renderer.acknowledge(&raw) {
                engine.flow_input(&lifecycle_input(feedback))?;
            } else {
                renderer.render(&[PresentationCommand::Text(
                    "Press Enter after reading, or use a session command.".into(),
                )])?;
            }
            continue;
        }
        let Some(action) = adapter.resolve(&raw) else {
            renderer.render(&[PresentationCommand::Text(
                "Enter a displayed choice number (or :resume when paused).".into(),
            )])?;
            continue;
        };
        if let Err(error) = engine.apply(&action) {
            renderer.render(&[PresentationCommand::Text(format!(
                "Choice rejected: {error}"
            ))])?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grimm_semantic::GameAction;

    #[test]
    fn submitted_timestamp_drives_boundary_order_without_sleeping() {
        for (elapsed, expected) in [(14999, "briefing"), (15000, "call_lost")] {
            let mut engine = crate::parse_story(
                "train",
                include_str!("../../../stories/the-last-train.yaml"),
            )
            .unwrap()
            .1;
            engine
                .apply(&GameAction {
                    revision: 0,
                    choice_id: "answer-phone".into(),
                })
                .unwrap();
            input(&mut engine, TimeEvent::Ready).unwrap();
            let revision = engine.output().revision;
            let origin = Instant::now();
            let mut observation = Observation {
                origin,
                last_total_ms: 0,
            };
            observation
                .observe_at(&mut engine, origin + Duration::from_millis(elapsed))
                .unwrap();
            let result = engine.apply(&GameAction {
                revision,
                choice_id: "reassure-rowan".into(),
            });
            assert_eq!(result.is_ok(), elapsed < 15000);
            assert_eq!(engine.save().node_id, expected);
        }
    }
}
