pub mod app;
mod color_names;
pub mod generator;
pub mod macros;
pub mod ui;
pub mod utility;

use std::{error::Error, io::stdout};

use crate::app::state::{Action, Screen, StateManagment};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
    },
    style::Style,
    widgets::Block,
};

fn main() -> Result<(), Box<dyn Error>> {
    let state = StateManagment::init()?;
    let terminal = ratatui::init();

    execute!(stdout(), EnableMouseCapture)?;
    run(terminal, state)?;

    execute!(stdout(), DisableMouseCapture)?;
    ratatui::restore();

    Ok(())
}

fn run(mut terminal: DefaultTerminal, mut state: StateManagment) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|frame| view(frame, &mut state))?;
        if let Some(Action::Exit) = state.event()? {
            return Ok(());
        }
    }
}

fn view(frame: &mut Frame, state: &mut StateManagment) {
    frame.render_widget(
        Block::new().style(Style::new().bg(ui::widgets::BACKGROUND_COLOR)),
        frame.area(),
    );

    let screen = state.current_screen.clone();

    match screen {
        Screen::Palette => state.screens.palette.render(frame),
    }
}
