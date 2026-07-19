use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::Stylize,
    text::Text,
    widgets::Paragraph,
};
pub struct Keymap;
impl Keymap {
    pub fn render<'a>(frame: &mut Frame, text: impl Into<Text<'a>>) {
        let inner = frame.area().inner(Margin {
            horizontal: 1,
            vertical: 0,
        });
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(inner);

        let keymap = Paragraph::new(text)
            .fg(super::FOREGROUND_COLOR)
            .alignment(Alignment::Center);

        frame.render_widget(keymap, chunks[1]);
    }
}
