use core::iter::zip;
use image::Pixel;
use image::Rgba;

#[derive(Debug)]
pub struct LayerWood {}

impl Default for LayerWood {
    fn default() -> Self {
        Self {}
    }
}
impl LayerWood {
    fn mix_f32(x: f32, y: f32, a: f32) -> f32 {
        x * (1.0 - a) + y * a
    }
    fn mix_rgba_f32(x: &Rgba<f32>, y: &Rgba<f32>, a: f32) -> Rgba<f32> {
        let mixed: Vec<_> = zip(x.channels(), y.channels())
            .map(|(x, y)| Self::mix_f32(*x, *y, a))
            .collect();
        *Rgba::from_slice(mixed.as_slice())
    }
    pub fn apply(
        &self,
        x: f32,
        y: f32,
        bw: usize,
        colors: &[Rgba<f32>; 2],
        color: Rgba<f32>,
    ) -> Rgba<f32> {
        let c = if bw == 0 {
            let s = 0.5 + 0.5 * (20.0 * x * 6.28 + (2.0 * y * 6.28).sin()).sin();
            Self::mix_rgba_f32(&colors[0], &colors[1], s)
        } else {
            let s = 0.5 + 0.5 * (20.0 * y * 6.28).sin();
            Self::mix_rgba_f32(&colors[0], &colors[1], s)
        };

        Self::mix_rgba_f32(&color, &c, 0.5)
    }
}
