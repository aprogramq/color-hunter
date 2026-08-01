use image::{Rgba, RgbaImage};
use palette::Srgb;
use serde::{Deserialize, Serialize};

use crate::color_names::palette_names;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorExport {
    Hex,
    Rgb,
    Hsl,
}

impl ColorExport {
    pub const ALL: [ColorExport; 3] = [ColorExport::Hex, ColorExport::Rgb, ColorExport::Hsl];

    pub fn label(&self) -> &'static str {
        match self {
            ColorExport::Hex => "Hex",
            ColorExport::Rgb => "RGB",
            ColorExport::Hsl => "HSL",
        }
    }
}

fn srgb8(c: &Srgb) -> [u8; 3] {
    let (r, g, b) = c.into_components();
    [r, g, b].map(|channel| (channel.clamp(0.0, 1.0) * 255.0).round() as u8)
}

pub fn format_color(c: &Srgb, format: ColorExport) -> String {
    let (r, g, b) = c.into_components();
    let [r8, g8, b8] = srgb8(c);

    match format {
        ColorExport::Hex => format!("#{:02X}{:02X}{:02X}", r8, g8, b8),
        ColorExport::Rgb => format!("rgb({}, {}, {})", r8, g8, b8),
        ColorExport::Hsl => {
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

#[derive(Clone, Debug, PartialEq)]
pub enum ExportData {
    Text(String),
    Image(RgbaImage),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetExport {
    Css,
    Scss,
    Svg,
    Png,
    Tailwind,
}

impl TargetExport {
    pub const ALL: [TargetExport; 5] = [
        TargetExport::Css,
        TargetExport::Scss,
        TargetExport::Tailwind,
        TargetExport::Svg,
        TargetExport::Png,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            TargetExport::Css => "CSS",
            TargetExport::Scss => "SCSS",
            TargetExport::Tailwind => "Tailwind Config",
            TargetExport::Svg => "SVG",
            TargetExport::Png => "PNG",
        }
    }

    fn hex(c: &Srgb) -> String {
        let [r, g, b] = srgb8(c);
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    pub fn css(colors: &[Srgb], color_format: ColorExport) -> String {
        let names = palette_names(colors);
        let vars: Vec<String> = colors
            .iter()
            .zip(names)
            .map(|(c, name)| format!("  --{}: {};", name, format_color(c, color_format)))
            .collect();
        format!(":root {{\n{}\n}}", vars.join("\n"))
    }

    pub fn scss(colors: &[Srgb], color_format: ColorExport) -> String {
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

    pub fn tailwind(colors: &[Srgb], color_format: ColorExport) -> String {
        let names = palette_names(colors);
        let entries: Vec<String> = colors
            .iter()
            .zip(names)
            .map(|(c, name)| format!("  '{}': '{}',", name, format_color(c, color_format)))
            .collect();
        format!("colors: {{\n{}\n}}", entries.join("\n"))
    }
}
