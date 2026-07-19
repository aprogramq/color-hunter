mod color_names;
pub mod effects;
pub mod generator;
pub mod macros;
pub mod screens;
pub mod settings;
pub mod states;
pub mod utility;
pub mod widgets;

use std::{error::Error, io::stdout};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
    },
    style::Style,
    widgets::Block,
};

use crate::states::{
    Action,
    Screen::{self},
    StateManagment,
};
fn main() -> Result<(), Box<dyn Error>> {
    let terminal = ratatui::init();
    let state = StateManagment::init()?;

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
        Block::new().style(Style::new().bg(widgets::BACKGROUND_COLOR)),
        frame.area(),
    );

    let screen = state.current_screen.clone();

    match screen {
        Screen::Palette => state.screens.palette.render(frame),
    }
}
