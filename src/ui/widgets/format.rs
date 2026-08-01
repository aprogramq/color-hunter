use crate::{
    app::{
        export::{ColorExport, ExportData, TargetExport},
        state::Action,
    },
    key,
    ui::effects::{EFFECT_COLOR, FocusEffect},
};
use palette::Srgb;
use ratatui::{
    Frame,
    crossterm::event::{KeyEvent, MouseButton, MouseEventKind},
    layout::{Margin, Position, Rect},
    style::Style,
    widgets::{Block, List, ListItem, ListState},
};

#[derive(Clone, Debug)]
pub struct ColorFormatWidget {
    pub format: ColorExport,
    list_state: ListState,

    focus_effect: FocusEffect,
    highlight_effect: FocusEffect,
    area: Rect,
}

impl ColorFormatWidget {
    pub fn new(format: ColorExport) -> Self {
        let selected = ColorExport::ALL.iter().position(|f| *f == format);
        Self {
            format,
            list_state: ListState::default().with_selected(selected),
            focus_effect: FocusEffect::new(),
            highlight_effect: FocusEffect::highlight(EFFECT_COLOR),
            area: Rect::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        self.area = area;

        let block = Block::bordered()
            .title("Color")
            .style(super::FOREGROUND_COLOR);
        let items: Vec<ListItem> = ColorExport::ALL
            .iter()
            .map(|f| ListItem::new(f.label()))
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(EFFECT_COLOR))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, area, &mut self.list_state);

        self.focus_effect.process(frame, area, is_focused);
        self.highlight_effect.process(frame, area, true);
    }

    pub fn handle_mouse(&mut self, kind: MouseEventKind, position: Position) -> bool {
        if !matches!(kind, MouseEventKind::Down(MouseButton::Left)) || !self.area.contains(position)
        {
            return false;
        }

        let inner = self.area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });
        if inner.contains(position) {
            let selected = usize::from(position.y - inner.y);
            if selected < ColorExport::ALL.len() {
                self.list_state.select(Some(selected));
            }
        }

        true
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event {
            key!(Tab) => return Some(Action::DelegateKeyUp),
            key!(Enter) => {
                self.apply();
                return Some(Action::Unfocus);
            }
            key!('j', NONE) | key!(Down) => {
                self.list_state.select_next();
            }
            key!('k', NONE) | key!(Up) => {
                self.list_state.select_previous();
            }
            key!('q', NONE) | key!(Esc) => return Some(Action::Unfocus),
            _ => return Some(Action::DelegateKeyUp),
        }
        None
    }

    pub fn apply(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.format = ColorExport::ALL[selected];
        }
    }

    pub fn reset_selection(&mut self) {
        self.list_state
            .select(ColorExport::ALL.iter().position(|f| *f == self.format));
    }
}

#[derive(Clone, Debug)]
pub struct ExportFormatWidget {
    pub format: TargetExport,
    list_state: ListState,

    focus_effect: FocusEffect,
    highlight_effect: FocusEffect,
    area: Rect,
}

impl ExportFormatWidget {
    pub fn new(format: TargetExport) -> Self {
        let selected = TargetExport::ALL.iter().position(|f| *f == format);
        Self {
            format,
            list_state: ListState::default().with_selected(selected),
            focus_effect: FocusEffect::new(),
            highlight_effect: FocusEffect::highlight(EFFECT_COLOR),
            area: Rect::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        self.area = area;

        let block = Block::bordered()
            .title("Format")
            .style(super::FOREGROUND_COLOR);
        let items: Vec<ListItem> = TargetExport::ALL
            .iter()
            .map(|format| ListItem::new(format.label()))
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(EFFECT_COLOR))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, area, &mut self.list_state);

        self.focus_effect.process(frame, area, is_focused);
        self.highlight_effect.process(frame, area, true);
    }

    pub fn handle_mouse(&mut self, kind: MouseEventKind, position: Position) -> bool {
        if !matches!(kind, MouseEventKind::Down(MouseButton::Left)) || !self.area.contains(position)
        {
            return false;
        }

        let inner = self.area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });
        if inner.contains(position) {
            let selected = usize::from(position.y - inner.y);
            if selected < TargetExport::ALL.len() {
                self.list_state.select(Some(selected));
            }
        }

        true
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent) -> Option<Action> {
        match key_event {
            key!(Tab) => Some(Action::DelegateKeyUp),
            key!(Enter) => {
                self.apply();
                Some(Action::Unfocus)
            }
            key!('j', NONE) | key!(Down) => {
                self.list_state.select_next();
                None
            }
            key!('k', NONE) | key!(Up) => {
                self.list_state.select_previous();
                None
            }
            key!('q', NONE) | key!(Esc) => Some(Action::Unfocus),
            _ => Some(Action::DelegateKeyUp),
        }
    }

    pub fn export(&self, colors: &[Srgb], color_format: ColorExport) -> ExportData {
        match self.format {
            TargetExport::Css => ExportData::Text(TargetExport::css(colors, color_format)),
            TargetExport::Scss => ExportData::Text(TargetExport::scss(colors, color_format)),
            TargetExport::Svg => ExportData::Text(TargetExport::svg(colors)),
            TargetExport::Png => ExportData::Image(TargetExport::png(colors)),
            TargetExport::Tailwind => {
                ExportData::Text(TargetExport::tailwind(colors, color_format))
            }
        }
    }

    pub fn apply(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.format = TargetExport::ALL[selected];
        }
    }

    pub fn reset_selection(&mut self) {
        self.list_state
            .select(TargetExport::ALL.iter().position(|f| *f == self.format));
    }
}
