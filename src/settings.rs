use crate::{
    generator::harmony::Generator as GeneratorKind,
    widgets::{
        format::{ColorFormat, ExportFormat},
        mode::Mode,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Options {
    #[serde(default)]
    pub palette: PaletteOptions,
    #[serde(default)]
    pub export: ExportOptions,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaletteOptions {
    #[serde(default = "default_kind")]
    pub kind: GeneratorKind,
    #[serde(default = "default_count")]
    pub count: u8,
    #[serde(default = "default_tick_rate")]
    pub tick_rate: u64,
    #[serde(default = "default_mode")]
    pub mode: Mode,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    #[serde(default = "default_export_format")]
    pub format: ExportFormat,
    #[serde(default = "default_color_format")]
    pub color_format: ColorFormat,
}

impl Default for PaletteOptions {
    fn default() -> Self {
        Self {
            kind: default_kind(),
            count: default_count(),
            tick_rate: default_tick_rate(),
            mode: default_mode(),
        }
    }
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: default_export_format(),
            color_format: default_color_format(),
        }
    }
}

fn default_kind() -> GeneratorKind {
    GeneratorKind::ColorHunter
}

fn default_count() -> u8 {
    6
}

fn default_tick_rate() -> u64 {
    200
}

fn default_mode() -> Mode {
    Mode::Running
}

fn default_export_format() -> ExportFormat {
    ExportFormat::Css
}

fn default_color_format() -> ColorFormat {
    ColorFormat::Hex
}
