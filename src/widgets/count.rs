use std::{error::Error, fs, time::Instant};

use ratatui::{
    Frame,
    crossterm::event::{KeyEvent, MouseButton, MouseEventKind},
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{effects::FocusEffect, generator::harmony::CountConstraint};
use crate::{key, settings::Options, states::Action};
use crate::{screens::palette::Focus, widgets::COMMENT_COLOR};

#[derive(Clone, Debug)]
pub struct CountWidget {
    pub value: u8,
    pub input: String,
    cursor_visible: bool,
    last_cursor_tick: Instant,

    focus_effect: FocusEffect,
    area: Rect,
}

impl CountWidget {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            input: value.to_string(),
            cursor_visible: true,
            last_cursor_tick: Instant::now(),
            focus_effect: FocusEffect::excluding_foreground(COMMENT_COLOR),
            area: Rect::default(),
        }
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        is_focused: bool,
        constraint: &CountConstraint,
    ) {
        self.area = area;

        let block = Block::bordered()
            .title("[1] Count")
            .style(super::FOREGROUND_COLOR);

        let display = if self.input.is_empty() {
            let mut spans = vec![Span::styled(
                constraint.hint(),
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
            *focus = Some(Focus::Count);
        }
    }

    pub fn handle_key(
        &mut self,
        key_event: &KeyEvent,
        constraint: &CountConstraint,
    ) -> Option<Action> {
        match key_event {
            key!(Tab) => return Some(Action::DelegateKeyUp),
            key!(Enter) => {
                self.apply_with_constraint(constraint);
                let _ = self.save();
                return Some(Action::Unfocus);
            }
            key!(Char(c)) if c.is_ascii_digit() => {
                self.input.push(*c);
            }
            key!(Backspace) => {
                self.input.pop();
            }
            key!('q', NONE) | key!(Esc) => {
                return {
                    self.input = self.value.to_string();
                    Some(Action::Unfocus)
                };
            }
            _ => return Some(Action::DelegateKeyUp),
        }
        None
    }

    pub fn tick(&mut self, is_focused: bool) {
        if is_focused && self.last_cursor_tick.elapsed() >= std::time::Duration::from_millis(400) {
            self.cursor_visible = !self.cursor_visible;
            self.last_cursor_tick = Instant::now();
        }
    }

    pub fn apply(&mut self) {
        self.value = self.input.parse().unwrap_or(1);
    }
    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        let user = crate::utility::get_username();
        let content =
            fs::read_to_string(format!("/home/{}/.config/color-hunter/config.toml", user))?;
        let mut options: Options = toml::from_str(&content)?;
        options.palette.count = self.value;
        fs::write(
            format!("/home/{}/.config/color-hunter/config.toml", user),
            toml::to_string(&options)?,
        )?;

        Ok(())
    }
    pub fn apply_constraint(&mut self, constraint: &CountConstraint) {
        self.value = constraint.normalize(self.value);
        self.input = self.value.to_string();
    }

    pub fn apply_with_constraint(&mut self, constraint: &CountConstraint) {
        let parsed = self.input.parse().unwrap_or(1);
        self.value = constraint.normalize(parsed);
        self.input = self.value.to_string();
    }
}
