//! Renderer-only structured scene view. The host supplies labels and read-only
//! debug rows; no engine or canonical-state type crosses this module boundary.
use crate::cues::{CuePlayer, Lifecycle};
use grimm_presentation::cues::{Clock, Instruction};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

#[derive(Debug, Clone)]
pub struct NarrativeEntry {
    pub epoch: u64,
    pub cue_id: String,
    pub text: String,
}
#[derive(Debug, Clone)]
pub struct WorldRow {
    pub scope: String,
    pub name: String,
    pub value: String,
    pub changed: bool,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    Narrative,
    Choices,
    Inspector,
    World,
}
#[derive(Debug, Clone, Default)]
pub struct SceneUi {
    pub inspector: bool,
    pub focus: Focus,
    pub narrative_scroll: u16,
    pub inspector_scroll: u16,
    pub world_scroll: u16,
    pub selected_choice: usize,
    /// Zero selects the current sheet; positive offsets select older revisions.
    pub history_back: usize,
    pub help: bool,
}
pub struct SceneView<'a> {
    pub title: &'a str,
    pub context: &'a str,
    pub clock: &'a Clock,
    pub status: &'a str,
    pub narrative: &'a [NarrativeEntry],
    pub choices: &'a [(String, String)],
    pub player: &'a CuePlayer,
    pub world: &'a [WorldRow],
}

pub fn cue_color(id: &str) -> Color {
    let index = id.bytes().fold(0usize, |hash, byte| {
        hash.wrapping_mul(31).wrapping_add(usize::from(byte))
    }) % 6;
    [
        Color::Cyan,
        Color::Yellow,
        Color::Green,
        Color::Magenta,
        Color::Blue,
        Color::LightRed,
    ][index]
}
impl SceneUi {
    pub fn next_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Narrative => Focus::Choices,
            Focus::Choices if self.inspector => Focus::Inspector,
            Focus::Inspector => Focus::World,
            _ => Focus::Narrative,
        };
    }
    pub fn scroll(&mut self, delta: i16, choice_count: usize) {
        if self.focus == Focus::Choices {
            self.selected_choice = self
                .selected_choice
                .saturating_add_signed(isize::from(delta))
                .min(choice_count.saturating_sub(1));
        } else {
            let offset = match self.focus {
                Focus::Inspector => &mut self.inspector_scroll,
                Focus::World => &mut self.world_scroll,
                _ => &mut self.narrative_scroll,
            };
            *offset = offset.saturating_add_signed(delta);
        }
    }
    pub fn draw(&mut self, frame: &mut Frame<'_>, view: &SceneView<'_>) {
        let area = frame.area();
        if area.width < 32 || area.height < 10 {
            frame.render_widget(
                Paragraph::new("GRIMM: enlarge terminal (32×10 minimum)"),
                area,
            );
            return;
        }
        let bands = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(4),
                Constraint::Length(2),
            ])
            .split(area);
        let deadline = if view.clock.deadline_ms.is_none() {
            "no deadline".into()
        } else if view.clock.visible {
            view.clock
                .deadline_ms
                .map(|end| {
                    format!(
                        "deadline {:.1}s",
                        end.saturating_sub(view.clock.elapsed_ms) as f64 / 1000.0
                    )
                })
                .unwrap_or_else(|| "no deadline".into())
        } else {
            "deadline hidden".into()
        };
        let header = if area.width < 80 {
            format!(
                "{} | {}\nT={:.1}s · {}",
                view.context,
                view.status,
                view.clock.elapsed_ms as f64 / 1000.0,
                deadline
            )
        } else {
            format!(
                "GRIMM / {} | {} · T={:.1}s | {}\n{}",
                view.title,
                view.context,
                view.clock.elapsed_ms as f64 / 1000.0,
                deadline,
                view.status
            )
        };
        frame.render_widget(
            Paragraph::new(header).block(Block::default().borders(Borders::BOTTOM)),
            bands[0],
        );
        if self.inspector && area.width >= 110 {
            let panes = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(54), Constraint::Percentage(46)])
                .split(bands[1]);
            self.draw_play(frame, panes[0], view);
            self.draw_inspector(frame, panes[1], view);
        } else if self.inspector {
            self.draw_inspector(frame, bands[1], view);
        } else {
            self.draw_play(frame, bands[1], view);
        }
        let footer = if self.help {
            "Tab focus · arrows/PgUp/PgDn scroll · [ ] revision history · Enter select/finish text · F2 inspector · Esc menu · F4 capture"
        } else {
            "1–9 Choose · Tab Focus · ↑↓ Scroll · Esc Pause/Save/Load · F2 Debug · ? Help"
        };
        frame.render_widget(Paragraph::new(footer).wrap(Wrap { trim: false }), bands[2]);
    }
    fn block(&self, title: &str, focus: Focus) -> Block<'_> {
        Block::default()
            .title(title.to_owned())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.focus == focus {
                Color::Cyan
            } else {
                Color::DarkGray
            }))
    }
    fn draw_play(&mut self, frame: &mut Frame<'_>, area: Rect, view: &SceneView<'_>) {
        self.selected_choice = self
            .selected_choice
            .min(view.choices.len().saturating_sub(1));
        let lines: Vec<Line<'_>> = if view.choices.is_empty() {
            vec![Line::raw(if view.player.pending() {
                "Presentation in progress…"
            } else {
                "No further actions · Esc for session menu"
            })]
        } else {
            view.choices
                .iter()
                .enumerate()
                .map(|(i, (_, label))| {
                    Line::styled(
                        format!(
                            "{} [{}] {}",
                            if self.focus == Focus::Choices && i == self.selected_choice {
                                "▶"
                            } else {
                                " "
                            },
                            i + 1,
                            label
                        ),
                        if self.focus == Focus::Choices && i == self.selected_choice {
                            Style::default().add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        },
                    )
                })
                .collect()
        };
        let choice_width = area.width.saturating_sub(2);
        let choice_lines = lines;
        let choices_paragraph = Paragraph::new(choice_lines.clone()).wrap(Wrap { trim: false });
        let choices_height = (choices_paragraph
            .line_count(choice_width)
            .saturating_add(2)
            .min(usize::from(u16::MAX)) as u16)
            .clamp(3, (area.height / 2).max(3));
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(choices_height)])
            .split(area);
        let mut lines = vec![];
        for entry in view.narrative {
            if self.inspector {
                lines.push(Line::from(Span::styled(
                    format!("● {} [epoch {}]", entry.cue_id, entry.epoch),
                    Style::default().fg(cue_color(&entry.cue_id)),
                )));
            }
            lines.extend(entry.text.lines().map(|line| Line::raw(line.to_owned())));
            lines.push(Line::raw(""));
        }
        let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
        let max = paragraph
            .line_count(sections[0].width.saturating_sub(2))
            .saturating_sub(usize::from(sections[0].height.saturating_sub(2)))
            .min(usize::from(u16::MAX)) as u16;
        self.narrative_scroll = self.narrative_scroll.min(max);
        frame.render_widget(
            paragraph
                .scroll((self.narrative_scroll, 0))
                .block(self.block("NARRATIVE", Focus::Narrative)),
            sections[0],
        );
        let before =
            Paragraph::new(choice_lines[..self.selected_choice.min(choice_lines.len())].to_vec())
                .wrap(Wrap { trim: false })
                .line_count(choice_width);
        let selected_height = choice_lines.get(self.selected_choice).map_or(0, |line| {
            Paragraph::new(line.clone())
                .wrap(Wrap { trim: false })
                .line_count(choice_width)
        });
        let scroll = if self.focus == Focus::Choices {
            before
                .saturating_add(selected_height)
                .saturating_sub(usize::from(sections[1].height.saturating_sub(2)))
                .min(before)
                .min(usize::from(u16::MAX)) as u16
        } else {
            0
        };
        frame.render_widget(
            choices_paragraph
                .scroll((scroll, 0))
                .block(self.block("CHOICES", Focus::Choices)),
            sections[1],
        );
    }
    fn draw_inspector(&mut self, frame: &mut Frame<'_>, area: Rect, view: &SceneView<'_>) {
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(area);
        self.history_back = self
            .history_back
            .min(view.player.history().len().saturating_sub(1));
        let mut lines = vec![
            Line::raw(format!(
                "context: {} epoch: {} rev: {}",
                view.context,
                view.clock.epoch,
                view.player.revision()
            )),
            Line::raw(format!(
                "clock: {}ms deadline: {:?} visible: {}",
                view.clock.elapsed_ms, view.clock.deadline_ms, view.clock.visible
            )),
        ];
        if let Some(trace) = view.player.history().iter().rev().nth(self.history_back) {
            lines.push(Line::raw(format!(
                "Sheet rev {} epoch {} ({} previous)",
                trace.revision, trace.epoch, self.history_back
            )));
            for row in &trace.cues {
                let status = match row.lifecycle {
                    Lifecycle::Pending => "○ pending".into(),
                    Lifecycle::Running { at_ms } => format!("▶ running since {at_ms}ms"),
                    Lifecycle::Completed { at_ms } => format!("✓ completed at {at_ms}ms"),
                    Lifecycle::Cancelled => "× replaced/cancelled".into(),
                };
                lines.push(Line::styled(
                    format!("{} {status}", row.cue.cue_id),
                    Style::default().fg(cue_color(&row.cue.cue_id)),
                ));
                lines.push(Line::raw(format!(
                    "at_ms: {} after_cue: {}",
                    row.cue.at_ms,
                    row.cue.after_cue.as_deref().unwrap_or("—")
                )));
                match &row.cue.instruction {
                    Instruction::Narrative {narrative_id,duration_ms} => {
                        lines.push(Line::raw(format!("narrative_id: {narrative_id}")));
                        lines.push(Line::raw(format!("duration_ms: {duration_ms:?}")));
                    }
                    Instruction::Actions {available_actions} => lines.push(Line::raw(format!("actions: {available_actions:?}"))),
                    Instruction::Timer {start_ms,deadline_at_ms,unset_deadline,extend_ms,visible} => lines.push(Line::raw(format!("timer relative:{start_ms:?} absolute:{deadline_at_ms:?} unset:{unset_deadline} extend:{extend_ms:?} visible:{visible:?}"))),
                }
            }
        }
        let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
        self.inspector_scroll = self.inspector_scroll.min(
            paragraph
                .line_count(sections[0].width.saturating_sub(2))
                .saturating_sub(usize::from(sections[0].height.saturating_sub(2)))
                .min(usize::from(u16::MAX)) as u16,
        );
        frame.render_widget(
            paragraph
                .scroll((self.inspector_scroll, 0))
                .block(self.block("CUE INSPECTOR · [ ] history", Focus::Inspector)),
            sections[0],
        );
        let rows: Vec<Line<'_>> = view
            .world
            .iter()
            .map(|row| {
                Line::styled(
                    format!(
                        "{} {}.{}: {}",
                        if row.changed { "*" } else { " " },
                        row.scope,
                        row.name,
                        row.value
                    ),
                    Style::default().fg(if row.changed {
                        Color::Yellow
                    } else {
                        Color::Gray
                    }),
                )
            })
            .collect();
        self.world_scroll = self.world_scroll.min(
            rows.len()
                .saturating_sub(usize::from(sections[1].height.saturating_sub(2)))
                .min(usize::from(u16::MAX)) as u16,
        );
        frame.render_widget(
            Paragraph::new(rows)
                .scroll((self.world_scroll, 0))
                .block(self.block("WORLD STATE · READ-ONLY", Focus::World)),
            sections[1],
        );
    }
}

/// Render a portable text capture of the same widgets used by the real terminal.
/// Capturing a view does not advance time, acknowledge cues or mutate gameplay.
pub fn capture(ui: &SceneUi, view: &SceneView<'_>, inspector: bool) -> String {
    let mut copy = ui.clone();
    copy.inspector = inspector;
    copy.narrative_scroll = u16::MAX;
    copy.inspector_scroll = 0;
    copy.history_back = 0;
    let width = if inspector { 150 } else { 100 };
    let height = 40;
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(width, height))
        .unwrap_or_else(|never| match never {});
    terminal
        .draw(|frame| copy.draw(frame, view))
        .unwrap_or_else(|never| match never {});
    let buffer = terminal.backend().buffer();
    let mut text = String::new();
    for y in 0..height {
        let line: String = (0..width).map(|x| buffer[(x, y)].symbol()).collect();
        text.push_str(line.trim_end());
        text.push('\n');
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use grimm_presentation::cues::{Cue, CueSheet};
    use ratatui::{Terminal, backend::TestBackend};

    fn player() -> CuePlayer {
        let mut player = CuePlayer::default();
        player.replace(CueSheet {
            protocol_version: 2,
            revision: 7,
            clock: Clock {
                epoch: 3,
                elapsed_ms: 10000,
                running: true,
                deadline_ms: Some(30000),
                visible: true,
            },
            cues: vec![Cue {
                cue_id: "generator.warning".into(),
                at_ms: 10000,
                started_at_ms: None,
                after_cue: None,
                instruction: Instruction::Narrative {
                    narrative_id: "lea.warning".into(),
                    duration_ms: Some(2200),
                },
            }],
        });
        player.started("generator.warning").unwrap();
        player
    }
    fn render(ui: &mut SceneUi, width: u16, height: u16) -> String {
        let player = player();
        let narrative = vec![NarrativeEntry {
            epoch: 3,
            cue_id: "generator.warning".into(),
            text: "Lea: Die Spule verliert Strom. 雪 fällt.".into(),
        }];
        let choices = vec![
            ("force".into(), "Force the latch".into()),
            ("retreat".into(), "Return to dispatch".into()),
        ];
        let world = vec![WorldRow {
            scope: "global".into(),
            name: "cold_exposure".into(),
            value: "28000".into(),
            changed: true,
        }];
        let view = SceneView {
            title: "WHITEOUT",
            context: "generator",
            clock: player.clock().unwrap(),
            status: "Playing",
            narrative: &narrative,
            choices: &choices,
            player: &player,
            world: &world,
        };
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| ui.draw(frame, &view)).unwrap();
        let buffer = terminal.backend().buffer();
        (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| buffer[(x, y)].symbol())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    #[test]
    fn ordinary_and_docked_layouts_keep_choices_and_status_visible() {
        let mut ui = SceneUi::default();
        let ordinary = render(&mut ui, 100, 30);
        for label in [
            "WHITEOUT",
            "T=10.0s",
            "deadline 20.0s",
            "NARRATIVE",
            "CHOICES",
            "Force the latch",
            "雪",
        ] {
            assert!(ordinary.contains(label), "missing {label}: {ordinary}");
        }
        assert!(!ordinary.contains("cold_exposure"));
        ui.inspector = true;
        let docked = render(&mut ui, 150, 40);
        for label in [
            "NARRATIVE",
            "CHOICES",
            "CUE INSPECTOR",
            "WORLD STATE",
            "READ-ONLY",
            "generator.warning",
            "lea.warning",
            "duration_ms: Some(2200)",
            "at_ms: 10000",
            "cold_exposure: 28000",
        ] {
            assert!(docked.contains(label), "missing {label}: {docked}");
        }
        assert_eq!(docked.matches("generator.warning").count(), 2);
    }
    #[test]
    fn resize_focus_and_scroll_are_bounded_without_changing_presentation_plan() {
        let mut ui = SceneUi {
            inspector: true,
            ..SceneUi::default()
        };
        let narrow = render(&mut ui, 70, 24);
        assert!(narrow.contains("CUE INSPECTOR"));
        assert!(!narrow.contains("NARRATIVE"));
        let compact = render(&mut ui, 32, 24);
        assert!(compact.contains("T=10.0s"));
        assert!(compact.contains("deadline 20.0s"));
        assert!(compact.contains("Playing"));
        assert!(render(&mut ui, 20, 5).contains("GRIMM"));
        ui.next_focus();
        assert_eq!(ui.focus, Focus::Choices);
        ui.scroll(100, 2);
        assert_eq!(ui.selected_choice, 1);
        ui.next_focus();
        ui.scroll(i16::MAX, 2);
        ui.history_back = usize::MAX;
        render(&mut ui, 150, 40);
        assert_eq!(ui.history_back, 0);
        assert!(ui.inspector_scroll < 100);
        ui.next_focus();
        assert_eq!(ui.focus, Focus::World);
        ui.next_focus();
        assert_eq!(ui.focus, Focus::Narrative);
    }
}
