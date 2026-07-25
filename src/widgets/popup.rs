use std::time::{Duration, Instant};

use crate::effects::ShowEffect;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph},
};

const POPUP_WIDTH: u16 = 42;
const POPUP_HEIGHT: u16 = 5;

#[derive(Clone, Copy, Debug)]
enum PopupKind {
    Success,
    Error,
}

#[derive(Clone, Debug)]
pub struct Popup {
    show_effect: ShowEffect,
    notification: Option<(String, PopupKind, Instant)>,
}

impl Popup {
    pub fn new() -> Self {
        Self {
            show_effect: ShowEffect::new(),
            notification: None,
        }
    }
    fn render(
        &mut self,
        frame: &mut Frame,
        title: &str,
        message: &str,
        title_color: Color,
        border_color: Color,
    ) {
        let area = Self::centered_rect(
            POPUP_WIDTH.min(frame.area().width),
            POPUP_HEIGHT.min(frame.area().height),
            frame.area(),
        );
        frame.render_widget(Clear, area);

        let block = Block::bordered()
            .title(format!(" {title} "))
            .style(Style::default().bg(super::POPUP_COLOR))
            .title_style(Style::default().fg(title_color).bold())
            .border_style(Style::default().fg(border_color));
        let paragraph = Paragraph::new(message)
            .style(
                Style::default()
                    .fg(super::FOREGROUND_COLOR)
                    .bg(super::POPUP_COLOR),
            )
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);

        self.show_effect.process(frame, area);
    }

    pub fn success_message(message: String) -> Self {
        Self {
            show_effect: ShowEffect::new(),
            notification: Some((message, PopupKind::Success, Instant::now())),
        }
    }

    pub fn error_message(message: String) -> Self {
        Self {
            show_effect: ShowEffect::new(),
            notification: Some((message, PopupKind::Error, Instant::now())),
        }
    }

    pub fn is_expired(&self, duration: Duration) -> bool {
        self.notification
            .as_ref()
            .is_some_and(|(_, _, created_at)| created_at.elapsed() >= duration)
    }

    pub fn render_notification(&mut self, frame: &mut Frame) {
        let Some((message, kind, _)) = self.notification.clone() else {
            return;
        };

        match kind {
            PopupKind::Success => self.success(frame, "✓ Copied!", &message),
            PopupKind::Error => self.error(frame, "Copy failed", &message),
        }
    }

    pub fn success(&mut self, frame: &mut Frame, title: &str, message: &str) {
        self.render(
            frame,
            title,
            message,
            Color::Rgb(61, 219, 217),
            Color::Rgb(61, 219, 217),
        );
    }

    pub fn error(&mut self, frame: &mut Frame, title: &str, message: &str) {
        self.render(
            frame,
            title,
            message,
            Color::Rgb(238, 83, 150),
            Color::Rgb(238, 83, 150),
        );
    }

    fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
        let vertical = Layout::vertical([Constraint::Length(height)])
            .flex(Flex::Center)
            .split(area);
        let horizontal = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .split(vertical[0]);
        horizontal[0]
    }
}

impl Default for Popup {
    fn default() -> Self {
        Self::new()
    }
}
