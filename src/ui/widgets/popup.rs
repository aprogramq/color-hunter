use std::time::{Duration, Instant};

use crate::ui::effects::{FocusEffect, ShowEffect};

use ratatui::{
    Frame,
    crossterm::event::{MouseButton, MouseEventKind},
    layout::{Alignment, Constraint, Flex, Layout, Position, Rect},
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

const CONTEXT_MENU_WIDTH: u16 = 16;
const CONTEXT_MENU_HEIGHT: u16 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaletteMenuAction {
    Copy,
    Export,
    Close,
}

#[derive(Clone, Debug)]
pub struct ContextMenu {
    position: Position,
    area: Rect,
    selected: usize,
    highlight_effect: FocusEffect,
}

impl ContextMenu {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            area: Rect::default(),
            selected: 0,
            highlight_effect: FocusEffect::highlight(super::POPUP_COLOR),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let frame_area = frame.area();

        let width = CONTEXT_MENU_WIDTH.min(frame_area.width);
        let height = CONTEXT_MENU_HEIGHT.min(frame_area.height);

        let max_x = frame_area.right().saturating_sub(width);
        let max_y = frame_area.bottom().saturating_sub(height);

        let x = self.position.x.clamp(frame_area.x, max_x.max(frame_area.x));
        let y = self.position.y.clamp(frame_area.y, max_y.max(frame_area.y));

        self.area = Rect::new(x, y, width, height);

        frame.render_widget(Clear, self.area);
        frame.render_widget(
            Block::bordered().style(
                Style::default()
                    .fg(super::FOREGROUND_COLOR)
                    .bg(super::POPUP_COLOR),
            ),
            self.area,
        );

        for (index, label) in ["Copy", "Export"].iter().enumerate() {
            let row = Rect::new(
                self.area.x.saturating_add(1),
                self.area.y.saturating_add(1 + index as u16),
                self.area.width.saturating_sub(2),
                1,
            );
            let style = Style::default()
                .fg(super::PRIMARY_COLOR)
                .bg(super::POPUP_COLOR);

            frame.render_widget(Paragraph::new(format!(" {label}")).style(style), row);
        }

        let selected_row = Rect::new(
            self.area.x.saturating_add(1),
            self.area.y.saturating_add(1 + self.selected.min(1) as u16),
            self.area.width.saturating_sub(2),
            1,
        );
        self.highlight_effect.process(frame, selected_row, true);
    }

    pub fn handle_mouse(
        &mut self,
        kind: MouseEventKind,
        position: Position,
    ) -> Option<PaletteMenuAction> {
        let hovered = self.item_at(position);
        match kind {
            MouseEventKind::Moved => {
                if let Some(index) = hovered {
                    self.selected = index;
                }
                None
            }
            MouseEventKind::Down(MouseButton::Left) => Some(match hovered {
                Some(0) => PaletteMenuAction::Copy,
                Some(1) => PaletteMenuAction::Export,
                _ => PaletteMenuAction::Close,
            }),
            _ => None,
        }
    }

    fn item_at(&self, position: Position) -> Option<usize> {
        if !self.area.contains(position)
            || position.x == self.area.x
            || position.x == self.area.right().saturating_sub(1)
        {
            return None;
        }

        let row = position.y.checked_sub(self.area.y.saturating_add(1))? as usize;
        (row < 2).then_some(row)
    }
}
