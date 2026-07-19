#[macro_export]
macro_rules! key {
    ($char:expr, $modifier:ident) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Char($char),
            modifiers: ratatui::crossterm::event::KeyModifiers::$modifier,
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: _,
        }
    };
    ($code:ident) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::$code,
            modifiers: ratatui::crossterm::event::KeyModifiers::NONE,
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: _,
        }
    };
    (Char($var:ident)) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Char($var),
            modifiers: ratatui::crossterm::event::KeyModifiers::NONE,
            kind: ratatui::crossterm::event::KeyEventKind::Press,
            state: _,
        }
    };
}
