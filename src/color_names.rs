use std::{collections::HashMap, sync::OnceLock};

use palette::{IntoColor, Oklab, Srgb};
use serde::Deserialize;

#[derive(Deserialize)]
struct ColorRecord {
    name: String,
    hex: String,
}

struct NamedColor {
    name: String,
    oklab: Oklab,
}

static COLORS: OnceLock<Vec<NamedColor>> = OnceLock::new();

fn colors() -> &'static [NamedColor] {
    COLORS.get_or_init(|| {
        let records: Vec<ColorRecord> =
            serde_json::from_str(include_str!("../data/colornames.short.json"))
                .expect("embedded color name catalog must be valid JSON");

        records
            .into_iter()
            .map(|record| {
                let rgb = u32::from_str_radix(record.hex.trim_start_matches('#'), 16)
                    .expect("catalog color must contain a valid hex value");
                let color = Srgb::new(
                    ((rgb >> 16) & 0xff) as f32 / 255.0,
                    ((rgb >> 8) & 0xff) as f32 / 255.0,
                    (rgb & 0xff) as f32 / 255.0,
                );
                NamedColor {
                    name: slug(&record.name),
                    oklab: color.into_color(),
                }
            })
            .collect()
    })
}

fn slug(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut needs_separator = false;

    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            if needs_separator && !result.is_empty() {
                result.push('-');
            }
            result.push(character.to_ascii_lowercase());
            needs_separator = false;
        } else {
            needs_separator = true;
        }
    }

    if result.is_empty() {
        return "color".to_owned();
    }
    if result.starts_with(|character: char| character.is_ascii_digit()) {
        result.insert_str(0, "color-");
    }
    result
}

fn nearest_name(color: &Srgb) -> &'static str {
    let target: Oklab = (*color).into_color();
    colors()
        .iter()
        .min_by(|left, right| {
            distance_squared(&target, &left.oklab)
                .total_cmp(&distance_squared(&target, &right.oklab))
        })
        .map(|color| color.name.as_str())
        .unwrap_or("color")
}

fn distance_squared(left: &Oklab, right: &Oklab) -> f32 {
    (left.l - right.l).powi(2) + (left.a - right.a).powi(2) + (left.b - right.b).powi(2)
}

pub fn palette_names(colors: &[Srgb]) -> Vec<String> {
    let mut occurrences = HashMap::new();
    colors
        .iter()
        .map(|color| {
            let name = nearest_name(color);
            let count = occurrences.entry(name).or_insert(0usize);
            *count += 1;
            if *count == 1 {
                name.to_owned()
            } else {
                format!("{}-{}", name, count)
            }
        })
        .collect()
}
