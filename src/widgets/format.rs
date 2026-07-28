
use image::{Rgba, RgbaImage};
use palette::Srgb;
use ratatui::{
    Frame,
    crossterm::event::{KeyEvent, MouseButton, MouseEventKind},
    layout::{Margin, Position, Rect},
    style::Style,
    widgets::{Block, List, ListItem, ListState},
};
use serde::{Deserialize, Serialize};

use crate::{
    color_names::palette_names,
    effects::{EFFECT_COLOR, FocusEffect},
    key,
    states::Action,
};

#[derive(Clone, Debug)]
pub struct ColorFormatWidget {
    pub format: ColorFormat,
    list_state: ListState,

    focus_effect: FocusEffect,
    highlight_effect: FocusEffect,
    area: Rect,
}

impl ColorFormatWidget {
    pub fn new(format: ColorFormat) -> Self {
        let selected = ColorFormat::ALL.iter().position(|f| *f == format);
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
        let items: Vec<ListItem> = ColorFormat::ALL
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
            if selected < ColorFormat::ALL.len() {
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
            self.format = ColorFormat::ALL[selected];
        }
    }

    pub fn reset_selection(&mut self) {
        self.list_state
            .select(ColorFormat::ALL.iter().position(|f| *f == self.format));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorFormat {
    Hex,
    Rgb,
    Hsl,
}

impl ColorFormat {
    pub const ALL: [ColorFormat; 3] = [ColorFormat::Hex, ColorFormat::Rgb, ColorFormat::Hsl];

    pub fn label(&self) -> &'static str {
        match self {
            ColorFormat::Hex => "Hex",
            ColorFormat::Rgb => "RGB",
            ColorFormat::Hsl => "HSL",
        }
    }
}

fn srgb8(c: &Srgb) -> [u8; 3] {
    let (r, g, b) = c.into_components();
    [r, g, b].map(|channel| (channel.clamp(0.0, 1.0) * 255.0).round() as u8)
}

pub fn format_color(c: &Srgb, format: ColorFormat) -> String {
    let (r, g, b) = c.into_components();
    let [r8, g8, b8] = srgb8(c);

    match format {
        ColorFormat::Hex => format!("#{:02X}{:02X}{:02X}", r8, g8, b8),
        ColorFormat::Rgb => format!("rgb({}, {}, {})", r8, g8, b8),
        ColorFormat::Hsl => {
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let l = (max + min) / 2.0;
            if (max - min).abs() < f32::EPSILON {
                return format!("hsl(0, 0%, {}%)", (l * 100.0).round() as u8);
            }
            let d = max - min;
            let s = if l > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };
            let h = if (max - r).abs() < f32::EPSILON {
                (g - b) / d + if g < b { 6.0 } else { 0.0 }
            } else if (max - g).abs() < f32::EPSILON {
                (b - r) / d + 2.0
            } else {
                (r - g) / d + 4.0
            };
            format!(
                "hsl({}, {}%, {}%)",
                (h * 60.0).round() as u16,
                (s * 100.0).round() as u8,
                (l * 100.0).round() as u8,
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExportFormatWidget {
    pub format: ExportFormat,
    list_state: ListState,

    focus_effect: FocusEffect,
    highlight_effect: FocusEffect,
    area: Rect,
}

impl ExportFormatWidget {
    pub fn new(format: ExportFormat) -> Self {
        let selected = ExportFormat::ALL.iter().position(|f| *f == format);
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
        let items: Vec<ListItem> = ExportFormat::ALL
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
            if selected < ExportFormat::ALL.len() {
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

    pub fn export(&self, colors: &[Srgb], color_format: ColorFormat) -> ExportData {
        match self.format {
            ExportFormat::Css => ExportData::Text(ExportFormat::css(colors, color_format)),
            ExportFormat::Scss => ExportData::Text(ExportFormat::scss(colors, color_format)),
            ExportFormat::Svg => ExportData::Text(ExportFormat::svg(colors)),
            ExportFormat::Png => ExportData::Image(ExportFormat::png(colors)),
            ExportFormat::Tailwind => {
                ExportData::Text(ExportFormat::tailwind(colors, color_format))
            }
        }
    }

    pub fn apply(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.format = ExportFormat::ALL[selected];
        }
    }

    pub fn reset_selection(&mut self) {
        self.list_state
            .select(ExportFormat::ALL.iter().position(|f| *f == self.format));
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExportData {
    Text(String),
    Image(RgbaImage),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Css,
    Scss,
    Svg,
    Png,
    Tailwind,
}

impl ExportFormat {
    pub const ALL: [ExportFormat; 5] = [
        ExportFormat::Css,
        ExportFormat::Scss,
        ExportFormat::Tailwind,
        ExportFormat::Svg,
        ExportFormat::Png,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            ExportFormat::Css => "CSS",
            ExportFormat::Scss => "SCSS",
            ExportFormat::Tailwind => "Tailwind Config",
            ExportFormat::Svg => "SVG",
            ExportFormat::Png => "PNG",
        }
    }

    fn hex(c: &Srgb) -> String {
        let [r, g, b] = srgb8(c);
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    pub fn css(colors: &[Srgb], color_format: ColorFormat) -> String {
        let names = palette_names(colors);
        let vars: Vec<String> = colors
            .iter()
            .zip(names)
            .map(|(c, name)| format!("  --{}: {};", name, format_color(c, color_format)))
            .collect();
        format!(":root {{\n{}\n}}", vars.join("\n"))
    }

    pub fn scss(colors: &[Srgb], color_format: ColorFormat) -> String {
        let names = palette_names(colors);
        colors
            .iter()
            .zip(names)
            .map(|(c, name)| format!("${}: {};", name, format_color(c, color_format)))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn svg(colors: &[Srgb]) -> String {
        let width = 50 * colors.len();
        let height = 60;
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
            width, height
        );
        for (index, color) in colors.iter().enumerate() {
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"0\" width=\"50\" height=\"{}\" fill=\"{}\" />\n",
                index * 50,
                height,
                Self::hex(color)
            ));
        }
        svg.push_str("</svg>");
        svg
    }

    pub fn png(colors: &[Srgb]) -> RgbaImage {
        const SWATCH_WIDTH: u32 = 50;
        const HEIGHT: u32 = 60;

        let width = SWATCH_WIDTH * colors.len() as u32;
        RgbaImage::from_fn(width, HEIGHT, |x, _| {
            let [r, g, b] = srgb8(&colors[x as usize / SWATCH_WIDTH as usize]);
            Rgba([r, g, b, 255])
        })
    }

    pub fn tailwind(colors: &[Srgb], color_format: ColorFormat) -> String {
        let names = palette_names(colors);
        let entries: Vec<String> = colors
            .iter()
            .zip(names)
            .map(|(c, name)| format!("  '{}': '{}',", name, format_color(c, color_format)))
            .collect();
        format!("colors: {{\n{}\n}}", entries.join("\n"))
    }
}
