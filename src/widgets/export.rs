use palette::Srgb;
use ratatui::{
    Frame,
    crossterm::event::{KeyEvent, MouseEventKind},
    layout::{Constraint, Flex, Layout, Margin, Position, Rect},
    style::Style,
    widgets::{Block, Clear},
};

use crate::{
    key,
    states::Action,
    states::ClipboardState,
    widgets::format::{
        ColorFormat, ColorFormatWidget, ExportData, ExportFormat, ExportFormatWidget,
    },
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum ExportState {
    Closed,
    Selecting,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ExportFocus {
    ExportFormat,
    ColorFormat,
}

#[derive(Clone, Debug)]
pub struct ExportWidget {
    export_format: ExportFormatWidget,
    color_format: ColorFormatWidget,
    state: ExportState,
    focus: ExportFocus,
    clipboard: ClipboardState,
}

impl ExportWidget {
    pub fn new(export_format: ExportFormat, color_format: ColorFormat) -> Self {
        Self {
            export_format: ExportFormatWidget::new(export_format),
            color_format: ColorFormatWidget::new(color_format),
            state: ExportState::Closed,
            focus: ExportFocus::ExportFormat,
            clipboard: ClipboardState::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match self.state {
            ExportState::Closed => {}
            ExportState::Selecting => self.render_popup(frame),
        }
    }

    fn render_popup(&mut self, frame: &mut Frame) {
        let area = centered_rect(68, 11, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(
            Block::bordered().title(" Export ").style(
                Style::default()
                    .fg(super::FOREGROUND_COLOR)
                    .bg(super::POPUP_COLOR),
            ),
            area,
        );

        let inner = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });
        let rows = Layout::vertical([Constraint::Length(7)])
            .flex(Flex::Center)
            .split(inner);
        let columns = Layout::horizontal([
            Constraint::Percentage(55),
            Constraint::Length(1),
            Constraint::Percentage(45),
        ])
        .split(rows[0]);

        self.export_format
            .render(frame, columns[0], self.focus == ExportFocus::ExportFormat);
        self.color_format
            .render(frame, columns[2], self.focus == ExportFocus::ColorFormat);
    }

    pub fn handle_mouse(&mut self, kind: MouseEventKind, position: Position) -> Option<Action> {
        if self.state == ExportState::Closed {
            return Some(Action::DelegateKeyUp);
        }

        if self.export_format.handle_mouse(kind, position) {
            self.focus = ExportFocus::ExportFormat;
        } else if self.color_format.handle_mouse(kind, position) {
            self.focus = ExportFocus::ColorFormat;
        }

        None
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent, colors: &[Srgb]) -> Option<Action> {
        match self.state {
            ExportState::Closed => return Some(Action::DelegateKeyUp),
            ExportState::Selecting => match key_event {
                key!('c', CONTROL) => return Some(Action::Exit),
                key!(Tab) => {
                    self.focus = match self.focus {
                        ExportFocus::ExportFormat => ExportFocus::ColorFormat,
                        ExportFocus::ColorFormat => ExportFocus::ExportFormat,
                    };
                }
                key!('j', NONE) | key!('k', NONE) | key!(Down) | key!(Up) => match self.focus {
                    ExportFocus::ExportFormat => return self.export_format.handle_key(key_event),
                    ExportFocus::ColorFormat => return self.color_format.handle_key(key_event),
                },
                key!(Enter) => {
                    self.export_format.apply();
                    self.color_format.apply();
                    let _ = self.export_format.save();
                    let _ = self.color_format.save();

                    let result = self.export_palette(colors);
                    self.state = ExportState::Closed;
                    return match result {
                        Ok(()) => Some(Action::PopupSuccess(
                            "Palette exported to clipboard!".to_string(),
                        )),
                        Err(error) => Some(Action::PopupError(format!(
                            "Failed to export palette: {error}"
                        ))),
                    };
                }
                key!('q', NONE) | key!(Esc) => {
                    self.state = ExportState::Closed;
                }
                _ => {}
            },
        }
        None
    }
    pub fn open(&mut self) {
        self.export_format.reset_selection();
        self.color_format.reset_selection();
        self.focus = ExportFocus::ExportFormat;
        self.state = ExportState::Selecting;
    }

    pub fn is_active(&self) -> bool {
        self.state != ExportState::Closed
    }

    fn export_palette(&self, colors: &[Srgb]) -> Result<(), arboard::Error> {
        match self.export_format.export(colors, self.color_format.format) {
            ExportData::Text(text) => self.clipboard.set_text(text),
            ExportData::Image(image) => self.clipboard.set_image(image),
        }
    }
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
