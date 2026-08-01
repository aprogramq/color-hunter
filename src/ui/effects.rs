use std::time::Instant;

use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::Color,
};
use tachyonfx::{CellFilter, Effect, Interpolation, color_from_hsl, fx, pattern::SweepPattern};

//This color will be used as a marker to indicate exactly where the effect should be applied
pub const EFFECT_COLOR: Color = Color::Rgb(61, 219, 217);

#[derive(Debug, Clone)]
pub struct FocusEffect {
    effect: Effect,
    last_tick: Instant,
}

impl FocusEffect {
    pub fn new() -> Self {
        Self {
            effect: Self::create_foreground_effect(),
            last_tick: Instant::now(),
        }
    }

    pub fn border() -> Self {
        Self {
            effect: Self::create_foreground_effect()
                .with_filter(CellFilter::Outer(Margin::new(1, 1))),
            last_tick: Instant::now(),
        }
    }

    pub fn excluding_foreground(color: Color) -> Self {
        Self {
            effect: Self::create_foreground_effect()
                .with_filter(CellFilter::NoneOf(vec![CellFilter::FgColor(color)])),
            last_tick: Instant::now(),
        }
    }

    pub fn highlight(color: Color) -> Self {
        Self {
            effect: Self::create_background_effect().with_filter(CellFilter::BgColor(color)),
            last_tick: Instant::now(),
        }
    }

    pub fn process(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);

        self.last_tick = now;

        if !is_focused {
            return;
        }

        self.effect
            .process(elapsed.into(), frame.buffer_mut(), area);
    }

    fn create_background_effect() -> Effect {
        const HUE_START: f32 = 179.0;
        const HUE_END: f32 = 194.0;

        let effect = fx::effect_fn(
            (),
            (2500, Interpolation::Linear),
            |_state, context, cells| {
                let width = f32::from(context.area.width.max(1));
                let animation_offset = context.alpha() * width;

                for (position, cell) in cells {
                    let cell_offset = f32::from(position.x - context.area.x);
                    let phase = (cell_offset - animation_offset) / width * std::f32::consts::TAU;
                    let smooth = phase.sin() * 0.5 + 0.5;
                    let hue = HUE_START + (HUE_END - HUE_START) * smooth;

                    cell.set_bg(color_from_hsl(hue, 100.0, 44.0));
                    cell.set_fg(Color::Black);
                }
            },
        );

        fx::repeating(effect)
    }

    pub fn create_foreground_effect() -> Effect {
        const HUE_START: f32 = 179.0;
        const HUE_END: f32 = 194.0;

        let effect = fx::effect_fn(
            (),
            (2500, Interpolation::Linear),
            |_state, context, cells| {
                let width = f32::from(context.area.width.max(1));
                let animation_offset = context.alpha() * width;

                for (position, cell) in cells {
                    let cell_offset = f32::from(position.x - context.area.x);
                    let phase = (cell_offset - animation_offset) / width * std::f32::consts::TAU;
                    let smooth = phase.sin() * 0.5 + 0.5;
                    let hue = HUE_START + (HUE_END - HUE_START) * smooth;

                    cell.set_fg(color_from_hsl(hue, 100.0, 44.0));
                }
            },
        );

        fx::repeating(effect)
    }
}

impl Default for FocusEffect {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ShowEffect {
    effect: Effect,
    last_tick: Instant,
}

impl ShowEffect {
    pub fn new() -> Self {
        Self {
            effect: fx::coalesce((300, Interpolation::QuadOut))
                .with_pattern(SweepPattern::right_to_left(20)),
            last_tick: Instant::now(),
        }
    }

    pub fn process(&mut self, frame: &mut Frame, area: Rect) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.last_tick = now;

        self.effect
            .process(elapsed.into(), frame.buffer_mut(), area);
    }
}

impl Default for ShowEffect {
    fn default() -> Self {
        Self::new()
    }
}
