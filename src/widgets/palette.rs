use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use palette::Srgb;
use ratatui::Frame;
use ratatui::crossterm::event::{KeyEvent, MouseButton, MouseEventKind};
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::prelude::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};

use crate::effects::FocusEffect;
use crate::generator::harmony::Generator;
use crate::screens::palette::Focus;
use crate::states::ClipboardState;
use crate::{
    key,
    states::Action,
    widgets::format::{ColorFormat, format_color},
};

const COLOR_WIDTH: u16 = 12;
const FOCUSED_COLOR_WIDTH: u16 = 16;
const COLOR_HEIGHT: u16 = 4;
const COLOR_BORDER_SIZE: u16 = 2;
const SELECTION_COLOR: Color = Color::Rgb(61, 219, 217);
const HISTORY_LIMIT: usize = 100;

#[derive(Clone, Debug)]
pub struct Palette {
    pub colors: Vec<Srgb>,

    // TODO: implement the display of the generator type
    #[allow(unused)]
    generator: Generator,
}

impl Palette {
    pub fn new(colors: Vec<Srgb>, generator: Generator) -> Self {
        Self { colors, generator }
    }
}

#[derive(Clone, Debug)]
pub struct PaletteWidget {
    focus_effect: FocusEffect,
    area: Rect,
    clipboard: ClipboardState,
    selected_color: usize,
    selection_anchor: Option<usize>,
    mouse_selecting: bool,
    running: bool,
    focus: Option<Focus>,
    history: VecDeque<Palette>,
    current_index: usize,
    last_tick: Instant,
}

impl PaletteWidget {
    pub fn new(running: bool, initial_palette: Palette) -> Self {
        Self {
            focus_effect: FocusEffect::border(),
            area: Rect::default(),
            clipboard: ClipboardState::default(),
            selected_color: 0,
            selection_anchor: None,
            mouse_selecting: false,
            running,
            focus: None,
            history: VecDeque::from([initial_palette]),
            current_index: 0,
            last_tick: Instant::now(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = area;
        let colors_len = self.current_colors().len();
        self.clamp_selected_color(colors_len);
        let selection = self.selection_bounds(colors_len);
        let is_focused = self.focus == Some(Focus::Palette);
        let colors = self.current_colors();

        let block = Block::bordered()
            .title("[0] Palette")
            .style(super::FOREGROUND_COLOR);

        let inner_area = block.inner(area);

        frame.render_widget(block, area);

        let Some(colors_layout) = Self::colors_layout(
            inner_area,
            colors.len(),
            self.selected_color,
            selection,
            is_focused,
        ) else {
            return;
        };

        for (index, (color, color_area)) in colors.iter().zip(colors_layout.iter()).enumerate() {
            let (r, g, b) = color.into_components();
            let r = (r * 255.0) as u8;
            let g = (g * 255.0) as u8;
            let b = (b * 255.0) as u8;

            let background = Color::Rgb(r, g, b);

            let is_cursor = is_focused && index == self.selected_color;
            let is_in_selection =
                selection.is_some_and(|(start, end)| (start..=end).contains(&index));

            if is_cursor || is_in_selection {
                let border_color = if is_in_selection {
                    SELECTION_COLOR
                } else {
                    Color::Rgb(
                        (u16::from(r) * 3 / 4) as u8,
                        (u16::from(g) * 3 / 4) as u8,
                        (u16::from(b) * 3 / 4) as u8,
                    )
                };
                let color_block = Block::bordered().border_style(border_color);
                let color_inner = color_block.inner(*color_area);
                frame.render_widget(color_block, *color_area);
                frame.render_widget(Block::new().bg(background), color_inner);

                if is_cursor && color_inner.width > 0 && color_inner.height > 0 {
                    let luminance =
                        (299 * u32::from(r) + 587 * u32::from(g) + 114 * u32::from(b)) / 1000;
                    let foreground = if luminance > 150 {
                        Color::Black
                    } else {
                        super::FOREGROUND_COLOR
                    };
                    let text_area = Layout::vertical([Constraint::Length(1)])
                        .flex(Flex::Center)
                        .split(color_inner)[0];
                    let hex = Paragraph::new(format!("#{r:02X}{g:02X}{b:02X}"))
                        .centered()
                        .fg(foreground)
                        .bg(background);
                    frame.render_widget(hex, text_area);
                }
            } else {
                let color_height = color_area.height.min(COLOR_HEIGHT);
                let color_inner = Layout::vertical([Constraint::Length(color_height)])
                    .flex(Flex::Center)
                    .split(*color_area)[0];
                frame.render_widget(Block::new().bg(background), color_inner);
            }
        }

        self.focus_effect.process(frame, area, is_focused);
    }

    fn colors_layout(
        inner_area: Rect,
        colors_len: usize,
        selected_color: usize,
        selection: Option<(usize, usize)>,
        is_focused: bool,
    ) -> Option<Vec<Rect>> {
        if colors_len == 0 || inner_area.width == 0 || inner_area.height == 0 {
            return None;
        }

        let selected_color = selected_color.min(colors_len - 1);
        let requested_height = if is_focused || selection.is_some() {
            COLOR_HEIGHT.saturating_add(COLOR_BORDER_SIZE)
        } else {
            COLOR_HEIGHT
        };
        let colors_height = inner_area.height.min(requested_height);
        let colors_row = Layout::vertical([Constraint::Length(colors_height)])
            .flex(Flex::Center)
            .split(inner_area)[0];
        let color_widths: Vec<u16> = (0..colors_len)
            .map(|index| {
                let is_in_selection =
                    selection.is_some_and(|(start, end)| (start..=end).contains(&index));
                if is_focused && index == selected_color {
                    FOCUSED_COLOR_WIDTH.saturating_add(COLOR_BORDER_SIZE)
                } else if is_in_selection {
                    COLOR_WIDTH.saturating_add(COLOR_BORDER_SIZE)
                } else {
                    COLOR_WIDTH
                }
            })
            .collect();
        let desired_width = color_widths
            .iter()
            .copied()
            .fold(0_u16, u16::saturating_add)
            .min(colors_row.width);
        let colors_area = Layout::horizontal([Constraint::Length(desired_width)])
            .flex(Flex::Center)
            .split(colors_row)[0];

        Some(
            Layout::horizontal(
                color_widths
                    .iter()
                    .copied()
                    .map(Constraint::Length)
                    .collect::<Vec<_>>(),
            )
            .split(colors_area)
            .to_vec(),
        )
    }

    pub fn handle_mouse(&mut self, kind: MouseEventKind, position: Position) -> Option<Action> {
        let is_focused = self.focus == Some(Focus::Palette);
        let colors_len = self.current_colors().len();
        let hovered_color = self.color_at_position(position);

        match kind {
            MouseEventKind::Moved if !self.has_selection() => {
                if let Some(index) = hovered_color {
                    self.selected_color = index;
                    self.focus = Some(Focus::Palette);
                } else if is_focused {
                    self.focus = None;
                }
                None
            }
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(index) = hovered_color {
                    self.start_mouse_selection(index, colors_len);
                    self.focus = Some(Focus::Palette);
                } else {
                    self.clear_selection();
                    self.focus = None;
                }
                None
            }
            MouseEventKind::Down(MouseButton::Right) => {
                let index = hovered_color?;
                let is_in_selection = self
                    .selection_bounds(colors_len)
                    .is_some_and(|(start, end)| (start..=end).contains(&index));

                if !is_in_selection {
                    self.clear_selection();
                    self.selected_color = index;
                }
                self.focus = Some(Focus::Palette);
                Some(Action::OpenMenu(position))
            }
            MouseEventKind::Drag(MouseButton::Left) if self.mouse_selecting => {
                if let Some(index) = hovered_color {
                    self.selected_color = index;
                    self.focus = Some(Focus::Palette);
                }
                None
            }
            MouseEventKind::Up(MouseButton::Left) if self.mouse_selecting => {
                if let Some(index) = hovered_color {
                    self.selected_color = index;
                }
                self.mouse_selecting = false;
                None
            }
            _ => None,
        }
    }

    fn start_mouse_selection(&mut self, index: usize, colors_len: usize) {
        if colors_len == 0 {
            self.clear_selection();
            return;
        }

        let index = index.min(colors_len - 1);
        self.selected_color = index;
        self.selection_anchor = Some(index);
        self.mouse_selecting = true;
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent) -> Option<Action> {
        let colors_len = self.current_colors().len();
        match key_event {
            key!('V', SHIFT) | key!('V', NONE) | key!('v', SHIFT) => {
                self.toggle_keyboard_selection();
                None
            }
            key!('h', NONE) | key!(Left) => {
                self.selected_color = self.selected_color.saturating_sub(1);
                None
            }
            key!('l', NONE) | key!(Right) => {
                if self.selected_color.saturating_add(1) < colors_len {
                    self.selected_color += 1;
                }
                None
            }
            key!('c', NONE) | key!('y', NONE) => self.copy_colors(),
            key!('q', NONE) | key!(Esc) if self.has_selection() => {
                self.clear_selection();
                None
            }
            key!('q', NONE) | key!(Esc) => Some(Action::Unfocus),
            _ => Some(Action::DelegateKeyUp),
        }
    }

    pub fn copy_colors(&self) -> Option<Action> {
        let colors = self.current_colors();
        let selected_colors = if let Some((start, end)) = self.selection_bounds(colors.len()) {
            &colors[start..=end]
        } else {
            let Some(color) = colors.get(self.selected_color) else {
                return Some(Action::PopupError("No color selected".to_string()));
            };
            std::slice::from_ref(color)
        };
        let text = selected_colors
            .iter()
            .map(|color| format_color(color, ColorFormat::Hex))
            .collect::<Vec<_>>()
            .join("\n");
        let count = selected_colors.len();

        match self.clipboard.set_text(text) {
            Ok(()) => Some(Action::PopupSuccess(if count == 1 {
                "Color copied to clipboard!".to_string()
            } else {
                format!("{count} colors copied to clipboard!")
            })),
            Err(error) => Some(Action::PopupError(format!(
                "Failed to copy colors: {error}"
            ))),
        }
    }
    pub fn push_palette(&mut self, palette: Palette) {
        if self.history.len() == HISTORY_LIMIT {
            self.history.pop_front();
        }
        self.history.push_back(palette);
        self.current_index = self.history.len() - 1;
        self.clear_selection();
    }

    pub fn select_previous_palette(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.clear_selection();
        }
    }

    pub fn select_next_palette(&mut self) {
        if self.current_index + 1 < self.history.len() {
            self.current_index += 1;
            self.clear_selection();
        }
    }

    pub fn selection_bounds(&self, colors_len: usize) -> Option<(usize, usize)> {
        let anchor = self.selection_anchor?;
        if colors_len == 0 {
            return None;
        }

        let anchor = anchor.min(colors_len - 1);
        let cursor = self.selected_color.min(colors_len - 1);
        Some((anchor.min(cursor), anchor.max(cursor)))
    }

    pub fn selected_colors(&self) -> &[Srgb] {
        let colors = self.current_colors();
        if let Some((start, end)) = self.selection_bounds(colors.len()) {
            &colors[start..=end]
        } else {
            colors
        }
    }

    pub fn has_selection(&self) -> bool {
        self.selection_anchor.is_some()
    }

    pub fn toggle_keyboard_selection(&mut self) {
        let colors_len = self.current_colors().len();
        if self.has_selection() {
            self.clear_selection();
        } else if colors_len > 0 {
            self.selection_anchor = Some(self.selected_color.min(colors_len - 1));
            self.mouse_selecting = false;
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection_anchor = None;
        self.mouse_selecting = false;
    }

    pub fn select_first(&mut self) {
        self.selected_color = 0;
    }

    fn clamp_selected_color(&mut self, colors_len: usize) {
        self.selected_color = if colors_len == 0 {
            0
        } else {
            self.selected_color.min(colors_len - 1)
        };
    }

    pub fn color_at_position(&self, position: Position) -> Option<usize> {
        let colors_len = self.current_colors().len();
        let inner_area = Block::bordered().inner(self.area);
        let colors_layout = Self::colors_layout(
            inner_area,
            colors_len,
            self.selected_color,
            self.selection_bounds(colors_len),
            self.focus == Some(Focus::Palette),
        )?;

        colors_layout
            .iter()
            .position(|color_area| color_area.contains(position))
    }

    pub fn focus(&self) -> Option<Focus> {
        self.focus
    }

    pub fn focus_mut(&mut self) -> &mut Option<Focus> {
        &mut self.focus
    }

    pub fn set_focus(&mut self, focus: Option<Focus>) {
        self.focus = focus;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn toggle_running(&mut self) {
        self.running = !self.running;
    }

    pub fn is_update_due(&self, interval: Duration) -> bool {
        self.running && self.last_tick.elapsed() >= interval
    }

    pub fn reset_tick(&mut self) {
        self.last_tick = Instant::now();
    }

    pub fn current_colors(&self) -> &[Srgb] {
        &self.history[self.current_index].colors
    }
}
