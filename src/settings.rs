use std::{error::Error, fs};
use crate::{
    generator::harmony::Generator,
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
    #[serde(default = "default_template")]
    pub template: Generator,
    #[serde(default = "default_count")]
    pub count: u8,
    #[serde(default = "default_speed")]
    pub speed: u64,
    #[serde(default = "default_mode")]
    pub mode: Mode,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    #[serde(default = "default_export_format")]
    pub format: ExportFormat,
    #[serde(default = "default_color")]
    pub color: ColorFormat,
}

impl Default for PaletteOptions {
    fn default() -> Self {
        Self {
            template: default_template(),
            count: default_count(),
            speed: default_speed(),
            mode: default_mode(),
        }
    }
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: default_export_format(),
            color: default_color(),
        }
    }
}

fn default_template() -> Generator {
    Generator::ColorHunter
}

fn default_count() -> u8 {
    6
}

fn default_speed() -> u64 {
    200
}

fn default_mode() -> Mode {
    Mode::Running
}

fn default_export_format() -> ExportFormat {
    ExportFormat::Css
}

fn default_color() -> ColorFormat {
    ColorFormat::Hex
}

pub fn save(update: impl FnOnce(&mut Options)) -> Result<(), Box<dyn Error>> {
    let user = crate::utility::get_username();
    let content = fs::read_to_string(format!("/home/{}/.config/color-hunter/config.toml", user))?;
    let mut options: Options = toml::from_str(&content)?;

    update(&mut options);

    fs::write(
        format!("/home/{}/.config/color-hunter/config.toml", user),
        toml::to_string(&options)?,
    )?;

    Ok(())
}
