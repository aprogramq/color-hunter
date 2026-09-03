use palette::Srgb;
use ratatui::{
    Frame,
    crossterm::event::{KeyEvent, MouseButton, MouseEventKind},
    layout::{Alignment, Constraint, Flex, Layout, Margin, Position, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph},
};

use crate::{
    app::{
        clipboard::ClipboardContent,
        export::{ColorExport, ExportData, TargetExport},
        settings,
        state::Action,
    },
    key,
    ui::widgets::format::{ColorFormatWidget, ExportFormatWidget},
};

const BUTTON_WIDTH: u16 = 16;

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
    button_area: Rect,
}

impl ExportWidget {
    pub fn new(export_format: TargetExport, color_format: ColorExport) -> Self {
        Self {
            export_format: ExportFormatWidget::new(export_format),
            color_format: ColorFormatWidget::new(color_format),
            state: ExportState::Closed,
            focus: ExportFocus::ExportFormat,
            button_area: Rect::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match self.state {
            ExportState::Closed => {}
            ExportState::Selecting => self.render_popup(frame),
        }
    }

    fn render_popup(&mut self, frame: &mut Frame) {
        let area = centered_rect(68, 12, frame.area());
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
        let rows = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(7),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);
        let columns = Layout::horizontal([
            Constraint::Percentage(55),
            Constraint::Length(1),
            Constraint::Percentage(45),
        ])
        .split(rows[1]);

        let show_color_format = self.show_color_format();
        let export_area = if show_color_format {
            columns[0]
        } else {
            centered_rect(columns[0].width, rows[1].height, rows[1])
        };

        self.export_format
            .render(frame, export_area, self.focus == ExportFocus::ExportFormat);
        if show_color_format {
            self.color_format
                .render(frame, columns[2], self.focus == ExportFocus::ColorFormat);
        }
        self.render_button(frame, rows[3]);
    }

    fn render_button(&mut self, frame: &mut Frame, area: Rect) {
        self.button_area = centered_rect(BUTTON_WIDTH.min(area.width), area.height.min(1), area);

        let button = Paragraph::new("Export").alignment(Alignment::Center).style(
            Style::default()
                .fg(super::BACKGROUND_COLOR)
                .bg(super::FOREGROUND_COLOR)
                .bold(),
        );

        frame.render_widget(button, self.button_area);
    }

    pub fn handle_mouse(
        &mut self,
        kind: MouseEventKind,
        position: Position,
        colors: &[Srgb],
    ) -> Option<Action> {
        if self.state == ExportState::Closed {
            return Some(Action::DelegateKeyUp);
        }

        if matches!(kind, MouseEventKind::Down(MouseButton::Left))
            && self.button_area.contains(position)
        {
            return Some(self.confirm_export(colors));
        }

        if self.export_format.handle_mouse(kind, position) {
            self.focus = ExportFocus::ExportFormat;
        } else if self.show_color_format() && self.color_format.handle_mouse(kind, position) {
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
                    if self.show_color_format() {
                        self.focus = match self.focus {
                            ExportFocus::ExportFormat => ExportFocus::ColorFormat,
                            ExportFocus::ColorFormat => ExportFocus::ExportFormat,
                        };
                    }
                }
                key!('j', NONE) | key!('k', NONE) | key!(Down) | key!(Up) => match self.focus {
                    ExportFocus::ExportFormat => return self.export_format.handle_key(key_event),
                    ExportFocus::ColorFormat => return self.color_format.handle_key(key_event),
                },
                key!(Enter) => return Some(self.confirm_export(colors)),
                key!('q', NONE) | key!(Esc) => {
                    self.state = ExportState::Closed;
                }
                _ => {}
            },
        }
        None
    }

    fn confirm_export(&mut self, colors: &[Srgb]) -> Action {
        self.export_format.apply();
        if self.show_color_format() {
            self.color_format.apply();
        }

        let _ = settings::save(|option| option.export.format = self.export_format.format);
        let _ = settings::save(|option| option.export.color = self.color_format.format);

        let content = match self.export_format.export(colors, self.color_format.format) {
            ExportData::Text(text) => ClipboardContent::Text(text),
            ExportData::Image(image) => ClipboardContent::Image(image),
        };
        self.state = ExportState::Closed;

        Action::CopyToClipboard {
            content,
            success_message: "Palette exported to clipboard!".to_string(),
            error_message: "Failed to export palette".to_string(),
        }
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

    fn show_color_format(&self) -> bool {
        matches!(
            self.export_format.selected_format(),
            TargetExport::Css | TargetExport::Scss | TargetExport::Tailwind
        )
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
