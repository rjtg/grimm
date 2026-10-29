//! Terminal-only input/output details. No engine or canonical state access.
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use grimm_presentation::RawInput;
use std::{
    io::{self, Write},
    time::Duration,
};

pub(super) struct Terminal {
    command: Option<(u64, String)>,
    menu: bool,
}

impl Terminal {
    pub fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        Ok(Self {
            command: None,
            menu: false,
        })
    }

    pub fn set_menu(&mut self, open: bool) -> io::Result<()> {
        self.command = None;
        self.menu = open;
        self.begin_prompt()
    }

    /// Drop queued keys from an old prompt before displaying its successor.
    /// A command being edited keeps its original revision until Enter or Esc.
    pub fn begin_prompt(&mut self) -> io::Result<()> {
        if self.command.is_none() {
            while event::poll(Duration::ZERO)? {
                let _ = event::read()?;
            }
        }
        Ok(())
    }

    pub fn poll(
        &mut self,
        revision: u64,
        wait: Duration,
        single_digit: bool,
        output: &mut impl Write,
    ) -> io::Result<Option<(u64, RawInput)>> {
        if !event::poll(wait)? {
            return Ok(None);
        }
        let Event::Key(key) = event::read()? else {
            return Ok(None);
        };
        if key.kind != KeyEventKind::Press {
            return Ok(None);
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            if matches!(key.code, KeyCode::Char('c' | 'd')) {
                return Ok(Some((
                    revision,
                    RawInput {
                        line: ":quit".into(),
                    },
                )));
            }
            return Ok(None);
        }
        if key
            .modifiers
            .intersects(KeyModifiers::ALT | KeyModifiers::SUPER)
        {
            return Ok(None);
        }
        if key.code == KeyCode::Esc {
            self.command = None;
            return Ok(Some((
                revision,
                RawInput {
                    line: ":menu".into(),
                },
            )));
        }
        if self.menu {
            let line = match key.code {
                KeyCode::Char('1') => ":menu-continue",
                KeyCode::Char('2') => ":menu-save",
                KeyCode::Char('3') => ":menu-load",
                KeyCode::Char('4') => ":quit",
                _ => return Ok(None),
            };
            return Ok(Some((revision, RawInput { line: line.into() })));
        }
        if let Some((original, buffer)) = &mut self.command {
            match key.code {
                KeyCode::Enter => {
                    let original = *original;
                    let line = std::mem::take(buffer);
                    self.command = None;
                    writeln!(output)?;
                    return Ok(Some((original, RawInput { line })));
                }
                KeyCode::Backspace => {
                    if buffer.pop().is_some() {
                        write!(output, "\x08 \x08")?;
                    }
                }
                KeyCode::Char(ch) => {
                    buffer.push(ch);
                    write!(output, "{ch}")?;
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char(ch @ '1'..='9') if single_digit => {
                    writeln!(output, "{ch}")?;
                    return Ok(Some((
                        revision,
                        RawInput {
                            line: ch.to_string(),
                        },
                    )));
                }
                // More than nine offers retain an explicit numeric line editor.
                KeyCode::Char(ch @ '0'..='9') if !single_digit => {
                    self.command = Some((revision, ch.to_string()));
                    write!(output, "{ch}")?;
                }
                KeyCode::Enter => {
                    writeln!(output)?;
                    return Ok(Some((revision, RawInput { line: "\n".into() })));
                }
                _ => {}
            }
        }
        output.flush()?;
        Ok(None)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

/// Raw mode disables terminal LF translation; keep ordinary renderer newlines.
pub(super) struct Output<W> {
    pub inner: W,
    pub raw: bool,
}
impl<W: Write> Write for Output<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if !self.raw {
            return self.inner.write(bytes);
        }
        for part in bytes.split_inclusive(|byte| *byte == b'\n') {
            if let Some(line) = part.strip_suffix(b"\n") {
                self.inner.write_all(line)?;
                self.inner.write_all(if line.ends_with(b"\r") {
                    b"\n"
                } else {
                    b"\r\n"
                })?;
            } else {
                self.inner.write_all(part)?;
            }
        }
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
