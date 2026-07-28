use ratatui::crossterm::event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::{Constraint, Layout, Position, Rect};
use std::fs;
use std::time::Duration;

use ratatui::Frame;

use crate::key;
use crate::settings::Options;
use crate::states::Action;
use crate::utility::get_username;

use crate::widgets::{
    ContextMenu, PaletteMenuAction, Popup,
    count::CountWidget,
    export::ExportWidget,
    keymap::Keymap,
    mode::{Mode, ModeWidget},
    palette::{Palette, PaletteWidget},
    speed::SpeedWidget,
    template::TemplateWidget,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Focus {
    Count,
    Template,
    Mode,
    Speed,
    Palette,
}

impl Focus {
    fn next(&self, mode: Mode) -> Self {
        match self {
            Self::Count => Self::Template,
            Self::Template => Self::Mode,
            Self::Mode if mode == Mode::Manual => Self::Palette,
            Self::Mode => Self::Speed,
            Self::Speed => Self::Palette,
            Self::Palette => Self::Count,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PaletteScreen {
    pub count: CountWidget,
    pub template: TemplateWidget,
    mode: ModeWidget,
    speed: SpeedWidget,
    export: ExportWidget,
    palette: PaletteWidget,
    popup: Option<Popup>,
    palette_menu: Option<ContextMenu>,
}
const KEYMAPS: &[&str; 8] = &[
    "Stop: <space>",
    "Running: <space>",
    "Generate: <space>",
    "History: h/l/←/→ || Switch: Tab || Export: e || Quit: Ctrl+c",
    "Select: h/l/←/→ || Range: Shift+V || Copy: y/c || Export: e || Cancel: q/Esc",
    "Select: j/k/↓/↑ || Apply: Enter || Cancel: q/Esc",
    "Apply: Enter || Cancel: q/Esc",
    "Select: j/k/↓/↑ || Switch: Tab || Export: Enter || Cancel: q/Esc || Quit: Ctrl+c",
];

impl PaletteScreen {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config: Options = toml::from_str(
            &fs::read_to_string(format!(
                "/home/{}/.config/color-hunter/config.toml",
                get_username()
            ))?
            .to_string(),
        )?;

        let template = TemplateWidget::new(config.palette.template);
        let normalized_count = config
            .palette
            .template
            .normalize_count(config.palette.count);

        Ok(PaletteScreen {
            mode: ModeWidget::new(config.palette.mode),
            template: TemplateWidget::new(config.palette.template),
            count: CountWidget::new(normalized_count),
            speed: SpeedWidget::new(Duration::from_millis(config.palette.speed)),
            export: ExportWidget::new(config.export.format, config.export.color),
            palette: PaletteWidget::new(
                config.palette.mode == Mode::Running,
                Palette::new(
                    template.generate_palette(config.palette.count),
                    config.palette.template,
                ),
            ),
            popup: None,
            palette_menu: None,
        })
    }

    fn next_focus(&self) -> Option<Focus> {
        Some(
            self.palette
                .focus()
                .map_or(Focus::Count, |focus| focus.next(self.mode.value)),
        )
    }

    pub fn mouse_handle(&mut self, mouse_event: MouseEvent) -> Option<Action> {
        let position = Position::new(mouse_event.column, mouse_event.row);

        if self.palette_menu.is_some() {
            if mouse_event.kind == MouseEventKind::Down(MouseButton::Right) {
                self.palette_menu = None;
            } else {
                let menu_action = self
                    .palette_menu
                    .as_mut()
                    .and_then(|menu| menu.handle_mouse(mouse_event.kind, position));
                return menu_action.and_then(|action| self.handle_palette_menu_action(action));
            }
        }

        if self.export.is_active() {
            let action = self.export.handle_mouse(
                mouse_event.kind,
                position,
                self.palette.selected_colors(),
            );

            return match action {
                Some(action @ (Action::PopupSuccess(_) | Action::PopupError(_))) => {
                    Some(self.show_popup(action))
                }
                action => action,
            };
        }

        let palette_action = self.palette.handle_mouse(mouse_event.kind, position);
        match palette_action {
            Some(Action::OpenMenu(position)) => {
                self.palette_menu = Some(ContextMenu::new(position));
                return None;
            }
            Some(action @ (Action::PopupSuccess(_) | Action::PopupError(_))) => {
                return Some(self.show_popup(action));
            }
            Some(action) => return Some(action),
            None => {}
        }

        self.count
            .handle_mouse(mouse_event.kind, position, self.palette.focus_mut());
        self.template
            .handle_mouse(mouse_event.kind, position, self.palette.focus_mut());
        self.count
            .apply_constraint(&self.template.value.count_constraint());
        self.mode
            .handle_mouse(mouse_event.kind, position, self.palette.focus_mut());
        if self.mode.value == Mode::Running {
            self.speed
                .handle_mouse(mouse_event.kind, position, self.palette.focus_mut());
        }

        None
    }

    pub fn key_handle(&mut self, key_event: KeyEvent) -> Option<Action> {
        if self.export.is_active() {
            let action = self
                .export
                .handle_key(&key_event, self.palette.selected_colors());

            return match action {
                Some(action @ (Action::PopupSuccess(_) | Action::PopupError(_))) => {
                    Some(self.show_popup(action))
                }
                action => action,
            };
        }

        let action = match self.palette.focus() {
            Some(Focus::Count) => self
                .count
                .handle_key(&key_event, &self.template.value.count_constraint()),
            Some(Focus::Template) => self.template.handle_key(&key_event),
            Some(Focus::Mode) => self.mode.handle_key(&key_event),
            Some(Focus::Speed) => self.speed.handle_key(&key_event),
            Some(Focus::Palette) => self.palette.handle_key(&key_event),
            _ => Some(Action::DelegateKeyUp),
        };

        match action {
            Some(Action::Unfocus) => {
                if self.palette.focus() == Some(Focus::Template) {
                    self.count
                        .apply_constraint(&self.template.value.count_constraint());
                }
                self.palette.set_focus(None);
                return None;
            }
            Some(action @ (Action::PopupSuccess(_) | Action::PopupError(_))) => {
                return Some(self.show_popup(action));
            }
            Some(action) if action != Action::DelegateKeyUp => return Some(action),
            None => return None,
            _ => (),
        }

        let not_focused_input_widgets =
            !matches!(self.palette.focus(), Some(Focus::Count | Focus::Speed));

        match key_event {
            key!('c', CONTROL) => return Some(Action::Exit),
            key!('V', SHIFT) | key!('V', NONE) | key!('v', SHIFT) => {
                self.palette.toggle_keyboard_selection();
                self.palette.set_focus(Some(Focus::Palette));
            }
            key!('e', NONE) => {
                self.palette.set_focus(None);
                self.export.open();
            }
            key!(' ', NONE) => {
                if self.mode.value == Mode::Manual {
                    self.generate_palette();
                } else {
                    self.palette.toggle_running();
                }
            }
            key!('h', NONE) | key!(Left) => self.palette.select_previous_palette(),
            key!('l', NONE) | key!(Right) => self.palette.select_next_palette(),
            key!(Tab) => {
                let next_focus = self.next_focus();
                if next_focus == Some(Focus::Palette) {
                    self.palette.select_first();
                }
                self.palette.set_focus(next_focus);
            }
            key!('1', NONE) => {
                if !matches!(self.palette.focus(), Some(Focus::Speed)) {
                    self.palette.set_focus(Some(Focus::Count));
                }
            }
            key!('2', NONE) => {
                if not_focused_input_widgets {
                    self.palette.set_focus(Some(Focus::Template));
                }
            }
            key!('3', NONE) => {
                if not_focused_input_widgets {
                    self.palette.set_focus(Some(Focus::Mode));
                }
            }
            key!('4', NONE) => {
                if !matches!(self.palette.focus(), Some(Focus::Count))
                    && self.mode.value == Mode::Running
                {
                    self.palette.set_focus(Some(Focus::Speed));
                }
            }
            key!('0', NONE) => {
                if not_focused_input_widgets {
                    self.palette.select_first();
                    self.palette.set_focus(Some(Focus::Palette));
                }
            }

            _ => {}
        }
        None
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.count.tick(self.palette.focus() == Some(Focus::Count));
        self.speed.tick(self.palette.focus() == Some(Focus::Speed));
        self.update_palette();

        let main_layout = Layout::vertical([
            Constraint::Length(6),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(frame.area());

        self.render_widgets(frame, main_layout[0]);

        self.palette.render(frame, main_layout[1]);

        let keymap = if self.export.is_active() {
            KEYMAPS[7].to_string()
        } else {
            let context_keymap = match self.palette.focus() {
                Some(Focus::Palette) => KEYMAPS[4],
                Some(Focus::Template | Focus::Mode) => KEYMAPS[5],
                Some(Focus::Count | Focus::Speed) => KEYMAPS[6],
                None => KEYMAPS[3],
            };

            format!("{} || {context_keymap}", KEYMAPS[self.keymaps()])
        };
        Keymap::render(frame, keymap);

        self.export.render(frame);
        self.render_popup(frame);
        if let Some(menu) = self.palette_menu.as_mut() {
            menu.render(frame);
        }
    }

    fn render_widgets(&mut self, frame: &mut Frame, area: Rect) {
        const COUNT_WIDTH: u16 = 15;
        const TEMPLATE_WIDTH: u16 = 25;
        const MODE_WIDTH: u16 = 15;
        const SPEED_WIDTH: u16 = 18;
        const GAP_WIDTH: u16 = 1;

        let is_mode_running = self.mode.value == Mode::Running;
        let widget_widths = if is_mode_running {
            vec![COUNT_WIDTH, TEMPLATE_WIDTH, MODE_WIDTH, SPEED_WIDTH]
        } else {
            vec![COUNT_WIDTH, TEMPLATE_WIDTH, MODE_WIDTH]
        };
        let widgets_width = (widget_widths.iter().sum::<u16>()
            + GAP_WIDTH * widget_widths.len().saturating_sub(1) as u16)
            .min(area.width);

        let centered_area = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(widgets_width),
            Constraint::Min(0),
        ])
        .split(area)[1];

        let widget_constraints = widget_widths
            .iter()
            .enumerate()
            .flat_map(|(index, width)| {
                (index > 0)
                    .then_some(Constraint::Length(GAP_WIDTH))
                    .into_iter()
                    .chain([Constraint::Length(*width)])
            })
            .collect::<Vec<_>>();
        let widget_areas = Layout::horizontal(widget_constraints).split(centered_area);

        self.count.render(
            frame,
            widget_areas[0],
            self.palette.focus() == Some(Focus::Count),
            &self.template.value.count_constraint(),
        );
        self.template.render(
            frame,
            widget_areas[2],
            self.palette.focus() == Some(Focus::Template),
        );
        self.mode.render(
            frame,
            widget_areas[4],
            self.palette.focus() == Some(Focus::Mode),
        );

        if self.mode.value == Mode::Running {
            self.speed.render(
                frame,
                widget_areas[6],
                self.palette.focus() == Some(Focus::Speed),
            );
        }
    }

    fn update_palette(&mut self) {
        if self.mode.value == Mode::Running && self.palette.is_update_due(self.speed.value) {
            self.palette.push_palette(Palette::new(
                self.template.generate_palette(self.count.value),
                self.template.value,
            ));
            self.palette.reset_tick();
        }
    }

    fn generate_palette(&mut self) {
        self.palette.push_palette(Palette::new(
            self.template.generate_palette(self.count.value),
            self.template.value,
        ));
    }

    fn handle_palette_menu_action(&mut self, action: PaletteMenuAction) -> Option<Action> {
        self.palette_menu = None;
        match action {
            PaletteMenuAction::Copy => match self.palette.copy_colors() {
                Some(action @ (Action::PopupSuccess(_) | Action::PopupError(_))) => {
                    Some(self.show_popup(action))
                }
                action => action,
            },
            PaletteMenuAction::Export => {
                self.palette.set_focus(None);
                self.export.open();
                None
            }
            PaletteMenuAction::Close => None,
        }
    }

    fn show_popup(&mut self, action: Action) -> Action {
        self.popup = match action {
            Action::PopupSuccess(message) => Some(Popup::success_message(message)),
            Action::PopupError(message) => Some(Popup::error_message(message)),
            _ => self.popup.take(),
        };
        Action::DrawTime(16, 300)
    }

    fn render_popup(&mut self, frame: &mut Frame) {
        let is_expired = self
            .popup
            .as_ref()
            .is_some_and(|popup| popup.is_expired(Duration::from_millis(600)));
        if is_expired {
            self.popup = None;
            return;
        }

        if let Some(popup) = self.popup.as_mut() {
            popup.render_notification(frame);
        }
    }

    fn keymaps(&self) -> usize {
        if self.mode.value == Mode::Running {
            if self.palette.is_running() { 0 } else { 1 }
        } else {
            2
        }
    }
}
