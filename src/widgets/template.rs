use std::{error::Error, fs};

use palette::Srgb;
use ratatui::{
    Frame,
    crossterm::event::{KeyEvent, MouseButton, MouseEventKind},
    layout::{Margin, Position, Rect},
    style::Style,
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Paragraph},
};

use crate::{
    effects::{EFFECT_COLOR, FocusEffect},
    generator::harmony::{
        Analogus, ColorHunter, Complementary, Coolors, Generator, SplitComplementary, Triadic,
    },
    key,
    screens::palette::Focus,
    settings::Options,
    states::Action,
};

const ITEMS: &[&str] = &[
    "Color Hunter",
    "Complementary",
    "Analogus",
    "Triadic",
    "SplitComplementary",
    "coolors.co",
];

#[derive(Clone, Debug)]
pub struct TemplateWidget {
    pub value: Generator,
    list_state: ListState,

    highlight_effect: FocusEffect,
    focus_effect: FocusEffect,
    area: Rect,
}

impl TemplateWidget {
    pub fn new(value: Generator) -> Self {
        let selected = Generator::ALL.iter().position(|g| *g == value);
        Self {
            value,
            list_state: ListState::default().with_selected(selected),
            highlight_effect: FocusEffect::highlight(EFFECT_COLOR),
            focus_effect: FocusEffect::new(),
            area: Rect::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        self.area = area;

        let block = Block::bordered()
            .title("[2] Template")
            .style(super::FOREGROUND_COLOR);

        let items: Vec<ListItem> = ITEMS.iter().map(|s| ListItem::new(*s)).collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(EFFECT_COLOR))
            .highlight_symbol(">>")
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(list, area, &mut self.list_state);

        self.render_scrollbar(frame, area);
        self.focus_effect.process(frame, area, is_focused);

        let mut highlight_area = area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        highlight_area.width = highlight_area.width.saturating_sub(1);
        self.highlight_effect.process(frame, highlight_area, true);
    }

    fn render_scrollbar(&self, frame: &mut Frame, area: Rect) {
        let visible_rows = usize::from(area.height.saturating_sub(2));
        if visible_rows > 0 && Generator::ALL.len() > visible_rows {
            let track_height = visible_rows;
            let thumb_height = visible_rows
                .saturating_mul(track_height)
                .div_ceil(Generator::ALL.len())
                .clamp(1, track_height);
            let max_offset = Generator::ALL.len().saturating_sub(visible_rows);
            let max_thumb_offset = track_height.saturating_sub(thumb_height);
            let thumb_offset = self
                .list_state
                .offset()
                .saturating_mul(max_thumb_offset)
                .div_ceil(max_offset);
            let lines: Vec<Line> = (0..track_height)
                .map(|row| {
                    let is_thumb = (thumb_offset..thumb_offset + thumb_height).contains(&row);
                    let symbol = if is_thumb { "█" } else { "│" };
                    Line::styled(symbol, Style::new().fg(super::FOREGROUND_COLOR))
                })
                .collect();
            let inner = area.inner(Margin {
                horizontal: 1,
                vertical: 1,
            });
            let scrollbar_area =
                Rect::new(inner.right().saturating_sub(1), inner.y, 1, inner.height);

            frame.render_widget(
                Paragraph::new(lines).style(Style::new().bg(super::BACKGROUND_COLOR)),
                scrollbar_area,
            );
        }
    }
    pub fn handle_mouse(
        &mut self,
        kind: MouseEventKind,
        position: Position,
        focus: &mut Option<Focus>,
    ) {
        match kind {
            MouseEventKind::Down(MouseButton::Left) if self.area.contains(position) => {
                let inner = self.area.inner(Margin {
                    horizontal: 1,
                    vertical: 1,
                });
                if !inner.contains(position) {
                    return;
                }
                let visible_row = usize::from(position.y - inner.y);
                let selected = self.list_state.offset() + visible_row;

                if selected < Generator::ALL.len() {
                    self.list_state.select(Some(selected));
                    self.apply();
                    *focus = Some(Focus::Template);
                }
            }
            MouseEventKind::ScrollDown if self.area.contains(position) => {
                let visible_rows = usize::from(self.area.height.saturating_sub(2));
                let max_offset = Generator::ALL.len().saturating_sub(visible_rows);

                self.list_state.select(None);
                let offset = self.list_state.offset().saturating_add(2).min(max_offset);
                *self.list_state.offset_mut() = offset;
            }
            MouseEventKind::ScrollUp if self.area.contains(position) => {
                self.list_state.select(None);
                let offset = self.list_state.offset().saturating_sub(1);
                *self.list_state.offset_mut() = offset;
            }
            _ => (),
        }
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event {
            key!(Tab) => return Some(Action::DelegateKeyUp),
            key!(Enter) => {
                self.apply();
                let _ = self.save();
                return Some(Action::Unfocus);
            }
            key!('j', NONE) | key!(Down) => {
                self.list_state.select_next();
            }
            key!('k', NONE) | key!(Up) => {
                self.list_state.select_previous();
            }
            key!('q', NONE) | key!(Esc) => {
                let selected = Generator::ALL
                    .iter()
                    .position(|generator| *generator == self.value);
                self.list_state.select(selected);
                return Some(Action::Unfocus);
            }
            _ => return Some(Action::DelegateKeyUp),
        }
        None
    }

    pub fn generate_palette(&self, count: u8) -> Vec<Srgb> {
        let count = self.value.normalize_count(count);
        match self.value {
            Generator::Complementary => Complementary::generate_palette(count),
            Generator::Analogus => Analogus::generate_palette(count),
            Generator::Triadic => Triadic::generate_palette(count),
            Generator::SplitComplementary => SplitComplementary::generate_palette(count),
            Generator::Coolors => Coolors::generate_palette(count),
            Generator::ColorHunter => ColorHunter::generate_palette(count),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let user = crate::utility::get_username();
        let content =
            fs::read_to_string(format!("/home/{}/.config/color-hunter/config.toml", user))?;
        let mut options: Options = toml::from_str(&content)?;
        options.palette.template = self.value;
        fs::write(
            format!("/home/{}/.config/color-hunter/config.toml", user),
            toml::to_string(&options)?,
        )?;
        Ok(())
    }

    pub fn apply(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.value = Generator::ALL[selected];
        }
    }
}
