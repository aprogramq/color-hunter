use std::time::Duration;

use ratatui::{
    Frame,
    crossterm::event::{KeyEvent, MouseButton, MouseEventKind},
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    app::{settings, state::Action},
    key,
    ui::{effects::FocusEffect, screens::palette::Focus, widgets::COMMENT_COLOR},
};

const MIN_SPEED_MS: u64 = 100;

#[derive(Clone, Debug)]
pub struct SpeedWidget {
    pub value: Duration,
    pub input: String,
    cursor_visible: bool,
    last_cursor_tick: std::time::Instant,
    focus_effect: FocusEffect,
    area: Rect,
}

impl SpeedWidget {
    pub fn new(value: Duration) -> Self {
        let value = value.max(Duration::from_millis(MIN_SPEED_MS));

        Self {
            value,
            input: value.as_millis().to_string(),
            cursor_visible: true,
            last_cursor_tick: std::time::Instant::now(),
            focus_effect: FocusEffect::excluding_foreground(COMMENT_COLOR),
            area: Rect::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        self.area = area;

        let block = Block::bordered()
            .title("[4] Speed (ms)")
            .style(super::FOREGROUND_COLOR);

        let display = if self.input.is_empty() {
            let mut spans = vec![Span::styled(
                format!("from {} ms", MIN_SPEED_MS),
                Style::new().fg(COMMENT_COLOR),
            )];
            if is_focused && self.cursor_visible {
                spans.push(Span::raw("▋"));
            }
            Line::from(spans)
        } else if is_focused && self.cursor_visible {
            Line::from(format!("{}▋", self.input))
        } else {
            Line::from(self.input.clone())
        };

        let widget = Paragraph::new(display).block(block);

        frame.render_widget(widget, area);

        self.focus_effect.process(frame, area, is_focused);
    }

    pub fn handle_mouse(
        &self,
        kind: MouseEventKind,
        position: ratatui::layout::Position,
        focus: &mut Option<Focus>,
    ) {
        if let MouseEventKind::Down(MouseButton::Left) = kind
            && self.area.contains(position)
        {
            *focus = Some(Focus::Speed);
        }
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event {
            key!(Tab) => return Some(Action::DelegateKeyUp),
            key!(Enter) => {
                self.apply();
                let _ =
                    settings::save(|option| option.palette.speed = self.value.as_millis() as u64);

                return Some(Action::Unfocus);
            }
            key!(Char(c)) if c.is_ascii_digit() => {
                self.input.push(*c);
            }
            key!(Backspace) => {
                self.input.pop();
            }
            key!('q', NONE) | key!(Esc) => {
                self.input = self.value.as_millis().to_string();
                return Some(Action::Unfocus);
            }
            _ => return Some(Action::DelegateKeyUp),
        }
        None
    }

    pub fn tick(&mut self, is_focused: bool) {
        if is_focused && self.last_cursor_tick.elapsed() >= Duration::from_millis(400) {
            self.cursor_visible = !self.cursor_visible;
            self.last_cursor_tick = std::time::Instant::now();
        }
    }

    pub fn apply(&mut self) {
        let ms = self
            .input
            .parse::<u64>()
            .unwrap_or(MIN_SPEED_MS)
            .max(MIN_SPEED_MS);
        self.value = Duration::from_millis(ms);
        self.input = ms.to_string();
    }
}
