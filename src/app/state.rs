use std::fs;

use std::time::Instant;
use std::{error::Error, time::Duration};

use ratatui::{
    crossterm::event::{self, Event},
    layout::Position,
};

use self::Action::NavigateTo;
use super::settings::Options;
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
}

#[derive(Clone)]
pub struct Objects {
    pub palette: PaletteScreen,
}
#[derive(Clone)]
struct DrawTime {
    time: u64,
    reset_at: Option<Instant>,
}
#[derive(Clone)]
pub struct StateManagment {
    pub current_screen: Screen,
    pub screens: Objects,
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
            match action {
                Some(NavigateTo(screen)) => self.set_screen(screen),
                Some(Action::Exit) => return Ok(Some(Action::Exit)),
                Some(Action::DrawTime(millis, timeout)) => {
                    let draw_time = &mut self.draw_time;

                    draw_time.time = millis;
                    draw_time.reset_at = Some(Instant::now() + Duration::from_millis(timeout))
                }
                _ => (),
            }
        }
        Ok(None)
    }
}
