use std::fs;

use std::time::Instant;
use std::{error::Error, time::Duration};

use arboard::Clipboard;
use ratatui::{
    crossterm::event::{self, Event},
    layout::Position,
};

use self::Action::NavigateTo;
use super::{clipboard::ClipboardContent, settings::Options};
use crate::app::clipboard;
use crate::ui::screens::palette::PaletteScreen;
use crate::utility::get_username;


#[derive(Clone, PartialEq)]
#[repr(usize)]
pub enum Screen {
    Palette,
}
type Millis = u64;
type Timeout = u64;

#[derive(PartialEq)]
pub enum Action {
    DelegateKeyUp,
    Exit,
    DrawTime(Millis, Timeout),
    NavigateTo(Screen),
    Recreate(Screen),
    Unfocus,
    OpenMenu(Position),
    PopupSuccess(String),
    PopupError(String),
    CopyToClipboard {
        content: ClipboardContent,
        success_message: String,
        error_message: String,
    },
}

pub struct Objects {
    pub palette: PaletteScreen,
}
struct DrawTime {
    time: u64,
    reset_at: Option<Instant>,
}
pub struct StateManagment {
    pub current_screen: Screen,
    pub screens: Objects,
    clipboard: Clipboard,
    draw_time: DrawTime,
}

impl StateManagment {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let config_path = format!("/home/{}/.config/color-hunter", get_username());
        if fs::read_to_string(format!("{}/config.toml", config_path)).is_err() {
            Self::init_config(&config_path)?;
        }
        Ok(Self {
            current_screen: Screen::Palette,
            screens: Objects {
                palette: PaletteScreen::new()?,
            },
            clipboard: Clipboard::new()?,
            draw_time: DrawTime {
                time: 100,
                reset_at: None,
            },
        })
    }
    fn init_config(config_path: &String) -> Result<(), Box<dyn std::error::Error>> {
        let new_options = Options::default();
        fs::create_dir_all(format!("{}/", config_path))?;
        fs::write(
            format!("{}/config.toml", config_path),
            toml::to_string(&new_options)?,
        )?;
        Ok(())
    }
    pub fn get_screens(&mut self) -> &mut PaletteScreen {
        &mut self.screens.palette
    }
    pub fn set_screen(&mut self, screen: Screen) {
        self.current_screen = screen
    }
    pub fn event(&mut self) -> Result<Option<Action>, Box<dyn Error>> {
        if self
            .draw_time
            .reset_at
            .is_some_and(|reset| Instant::now() > reset)
        {
            self.draw_time.time = 100;
            self.draw_time.reset_at = None;
        }
        if event::poll(Duration::from_millis(self.draw_time.time))? {
            let action = match event::read()? {
                Event::Key(key_event) => match self.current_screen {
                    Screen::Palette => self.screens.palette.key_handle(key_event),
                },
                Event::Mouse(mouse_event) => match self.current_screen {
                    Screen::Palette => self.screens.palette.mouse_handle(mouse_event),
                },
                _ => None,
            };
            if let Some(action) = action {
                return self.handle_action(action);
            }
        }
        Ok(None)
    }

    fn handle_action(&mut self, action: Action) -> Result<Option<Action>, Box<dyn Error>> {
        match action {
            NavigateTo(screen) => self.set_screen(screen),
            Action::Exit => return Ok(Some(Action::Exit)),
            Action::DrawTime(millis, timeout) => {
                self.draw_time.time = millis;
                self.draw_time.reset_at = Some(Instant::now() + Duration::from_millis(timeout));
            }
            Action::CopyToClipboard {
                content,
                success_message,
                error_message,
            } => {
                let popup = match clipboard::set(&mut self.clipboard, content) {
                    Ok(()) => Action::PopupSuccess(success_message),
                    Err(error) => Action::PopupError(format!("{error_message}: {error}")),
                };
                let draw_action = self.screens.palette.show_popup(popup);
                return self.handle_action(draw_action);
            }
            action @ (Action::PopupSuccess(_) | Action::PopupError(_)) => {
                let draw_action = self.screens.palette.show_popup(action);
                return self.handle_action(draw_action);
            }
            _ => {}
        }
        Ok(None)
    }
}
