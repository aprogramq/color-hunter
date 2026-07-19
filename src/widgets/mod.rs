pub mod count;
pub mod export;
pub mod format;
pub mod keymap;
pub mod mode;
pub mod palette;
pub mod popup;
pub mod speed;
pub mod template;

pub use keymap::Keymap;
pub use popup::Popup;

use ratatui::style::Color;
pub const BACKGROUND_COLOR: Color = Color::Rgb(22, 22, 22);
pub const FOREGROUND_COLOR: Color = Color::Rgb(221, 225, 230);
pub const PRIMARY_COLOR: Color = Color::Rgb(61, 219, 217);
pub const COMMENT_COLOR: Color = Color::Rgb(87, 87, 85);
