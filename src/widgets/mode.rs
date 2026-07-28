
use ratatui::{
    Frame,
    crossterm::event::{KeyEvent, MouseButton, MouseEventKind},
    layout::{Margin, Position, Rect},
    style::Style,
    widgets::{Block, List, ListItem, ListState},
};
use serde::{Deserialize, Serialize};

use crate::{effects::EFFECT_COLOR, screens::palette::Focus, settings};
use crate::{effects::FocusEffect, key, states::Action};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Running,
    Manual,
}

impl Mode {
    pub const ALL: [Mode; 2] = [Mode::Running, Mode::Manual];
}

const ITEMS: &[&str] = &["Running", "Manual"];

#[derive(Clone, Debug)]
pub struct ModeWidget {
    pub value: Mode,
    list_state: ListState,
    focus_effect: FocusEffect,
    highlight_effect: FocusEffect,
    area: Rect,
}

impl ModeWidget {
    pub fn new(value: Mode) -> Self {
        let selected = Mode::ALL.iter().position(|m| *m == value);
        Self {
            value,
            list_state: ListState::default().with_selected(selected),
            focus_effect: FocusEffect::new(),
            highlight_effect: FocusEffect::highlight(EFFECT_COLOR),
            area: Rect::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        self.area = area;

        let block = Block::bordered().title("[3] Mode");

        let items: Vec<ListItem> = ITEMS.iter().map(|s| ListItem::new(*s)).collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(EFFECT_COLOR))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, area, &mut self.list_state);

        self.focus_effect.process(frame, area, is_focused);
        self.highlight_effect.process(frame, area, true);
    }

    pub fn handle_mouse(
        &mut self,
        kind: MouseEventKind,
        position: Position,
        focus: &mut Option<Focus>,
    ) {
        if let MouseEventKind::Down(MouseButton::Left) = kind {
            if !self.area.contains(position) {
                return;
            }
            let inner = self.area.inner(Margin {
                horizontal: 1,
                vertical: 1,
            });
            if !inner.contains(position) {
                return;
            }

            let selected = usize::from(position.y - inner.y);

            if selected < Mode::ALL.len() {
                self.list_state.select(Some(selected));
                self.apply();
                *focus = Some(Focus::Mode);
            }
        }
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event {
            key!(Tab) => return Some(Action::DelegateKeyUp),
            key!(Enter) => {
                self.apply();
                let _ = settings::save(|option| option.palette.mode = self.value);
                return Some(Action::Unfocus);
            }
            key!('j', NONE) | key!(Down) => {
                self.list_state.select_next();
            }
            key!('k', NONE) | key!(Up) => {
                self.list_state.select_previous();
            }
            key!('q', NONE) | key!(Esc) => {
                let selected = Mode::ALL.iter().position(|mode| *mode == self.value);
                self.list_state.select(selected);

                return Some(Action::Unfocus);
            }
            _ => return Some(Action::DelegateKeyUp),
        }
        None
    }

    pub fn apply(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.value = Mode::ALL[selected];
        }
    }
}
