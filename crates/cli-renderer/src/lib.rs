//! Text I/O only. This crate cannot see game actions or engine state.
pub mod cues;

use grimm_presentation::{PresentationCommand, PresentationEvent, PresentationFeedback, RawInput};
use std::io::{self, BufRead, Write};

pub struct CliRenderer<R, W> {
    input: R,
    output: W,
    completion: Option<u64>,
    ready: Option<u64>,
}

impl<R: BufRead, W: Write> CliRenderer<R, W> {
    pub fn new(input: R, output: W) -> Self {
        Self {
            input,
            output,
            completion: None,
            ready: None,
        }
    }

    pub fn render(&mut self, commands: &[PresentationCommand]) -> io::Result<()> {
        for command in commands {
            match command {
                PresentationCommand::AwaitCompletion { revision, prompt } => {
                    self.completion = Some(*revision);
                    self.ready = None;
                    writeln!(self.output, "{prompt}")?;
                }
                PresentationCommand::InteractionAvailable { revision } => {
                    self.completion = None;
                    self.ready = Some(*revision);
                }
                PresentationCommand::Text(text) => writeln!(self.output, "{text}")?,
                PresentationCommand::Countdown {
                    remaining_ms,
                    label,
                } => {
                    writeln!(
                        self.output,
                        "{label}: {}.{}s",
                        remaining_ms / 1000,
                        (remaining_ms % 1000) / 100
                    )?;
                }
                PresentationCommand::Choices(labels) => {
                    for (index, label) in labels.iter().enumerate() {
                        writeln!(self.output, "{}. {label}", index + 1)?;
                    }
                    write!(self.output, "> ")?;
                }
            }
        }
        self.output.flush()
    }

    /// Reading completion is explicit blank input, never inferred from flush.
    pub fn acknowledge(&self, input: &RawInput) -> Option<PresentationFeedback> {
        // read_line may return an unterminated whitespace fragment at EOF.
        // That is interrupted input, not an explicit Enter acknowledgement.
        (input.line.ends_with('\n') && input.line.trim().is_empty()).then_some(())?;
        Some(PresentationFeedback {
            revision: self.completion?,
            event: PresentationEvent::Complete,
        })
    }

    /// Called by the host only after a successful render/flush of available offers.
    pub fn interaction_ready(&self) -> Option<PresentationFeedback> {
        Some(PresentationFeedback {
            revision: self.ready?,
            event: PresentationEvent::InteractionReady,
        })
    }

    /// EOF is distinct from an empty line so redirected input cannot spin.
    pub fn read_input(&mut self) -> io::Result<Option<RawInput>> {
        let mut line = String::new();
        match self.input.read_line(&mut line)? {
            0 => Ok(None),
            _ => Ok(Some(RawInput { line })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feedback_distinguishes_explicit_reading_completion_from_offer_availability() {
        let mut bytes = Vec::new();
        let mut renderer = CliRenderer::new(&b""[..], &mut bytes);
        renderer
            .render(&[PresentationCommand::AwaitCompletion {
                revision: 4,
                prompt: "Read, then Enter".into(),
            }])
            .unwrap();
        assert!(renderer.interaction_ready().is_none());
        for line in ["1", "", " \t"] {
            assert!(
                renderer
                    .acknowledge(&RawInput { line: line.into() })
                    .is_none()
            );
        }
        assert_eq!(
            renderer.acknowledge(&RawInput { line: "\n".into() }),
            Some(PresentationFeedback {
                revision: 4,
                event: PresentationEvent::Complete
            })
        );
        renderer
            .render(&[
                PresentationCommand::Choices(vec!["Act".into()]),
                PresentationCommand::InteractionAvailable { revision: 5 },
            ])
            .unwrap();
        assert!(
            renderer
                .acknowledge(&RawInput { line: "\n".into() })
                .is_none()
        );
        assert_eq!(
            renderer.interaction_ready(),
            Some(PresentationFeedback {
                revision: 5,
                event: PresentationEvent::InteractionReady
            })
        );
    }

    #[test]
    fn countdown_is_only_formatted_presentation_data() {
        let mut bytes = Vec::new();
        CliRenderer::new(&b""[..], &mut bytes)
            .render(&[PresentationCommand::Countdown {
                remaining_ms: 1234,
                label: "Paused; time remaining".into(),
            }])
            .unwrap();
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            "Paused; time remaining: 1.2s\n"
        );
    }

    #[test]
    fn renders_commands_and_preserves_raw_input() {
        let mut bytes = Vec::new();
        let mut renderer = CliRenderer::new(&b" 1\n\n"[..], &mut bytes);
        renderer
            .render(&[
                PresentationCommand::Text("Hello".into()),
                PresentationCommand::Choices(vec!["Continue".into()]),
            ])
            .unwrap();
        assert_eq!(
            renderer.read_input().unwrap(),
            Some(RawInput {
                line: " 1\n".into()
            })
        );
        assert_eq!(
            renderer.read_input().unwrap(),
            Some(RawInput { line: "\n".into() })
        );
        assert_eq!(renderer.read_input().unwrap(), None);
        assert_eq!(String::from_utf8(bytes).unwrap(), "Hello\n1. Continue\n> ");
    }

    #[test]
    fn io_errors_are_propagated() {
        struct Broken;
        impl Write for Broken {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::ErrorKind::BrokenPipe.into())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut renderer = CliRenderer::new(&b""[..], Broken);
        assert_eq!(
            renderer
                .render(&[PresentationCommand::Text("Hello".into())])
                .unwrap_err()
                .kind(),
            io::ErrorKind::BrokenPipe
        );
        let mut renderer = CliRenderer::new(&b"\xff"[..], Vec::new());
        assert!(renderer.read_input().is_err());
    }
}
