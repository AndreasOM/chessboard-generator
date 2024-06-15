use crate::layers::LayerWood;
use color_eyre::Result;
use image::DynamicImage;
use image::ImageBuffer;
use image::ImageFormat;
use image::Pixel;
use image::Rgba;
use image::RgbaImage;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChessboardGeneratorError {
    #[error("overlay has wrong size, expected {size}x{size} got {width}x{height}")]
    WrongOverlaySize { size: u32, width: u32, height: u32 },
}

type RgbaImage32F = ImageBuffer<Rgba<f32>, Vec<f32>>;
#[derive(Debug)]
pub struct ChessboardGenerator {
    size: u32,
    colors: [Rgba<f32>; 2],
    buffer: Option<RgbaImage>,
    overlays: Vec<RgbaImage32F>,
    layers: Vec<Box<LayerWood>>,
}

impl Default for ChessboardGenerator {
    fn default() -> Self {
        Self {
            size: 128,
            colors: [
                Rgba::from([1.0, 1.0, 1.0, 1.0]),
                Rgba::from([0.0, 0.0, 0.0, 1.0]),
            ],
            buffer: None,
            overlays: Vec::default(),
            layers: Vec::default(),
        }
    }
}
impl ChessboardGenerator {
    pub fn new(colors: &[[u8; 4]; 2]) -> Self {
        Self {
            colors: colors.map(|c| Rgba::from(c.map(|c| (c as f32) / 255.0))),
            ..Default::default()
        }
    }
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }
    pub fn add_layer(&mut self, layer: Box<LayerWood>) {
        self.layers.push(layer);
    }
    pub fn add_overlay_image(&mut self, img: DynamicImage) -> Result<()> {
        // :TODO: verify sizes match
        if img.width() != self.size || img.height() != self.size {
            Err(ChessboardGeneratorError::WrongOverlaySize {
                size: self.size,
                width: img.width(),
                height: img.height(),
            }
            .into())
        } else {
            self.overlays.push(img.into_rgba32f());
            Ok(())
        }
    }
    pub fn render(&mut self) {
        let mut buffer = RgbaImage32F::new(self.size, self.size);
        let size = self.size as f32;
        let ppc = size / 8.0;
        let colors = self.colors;
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let fx = x as f32;
            let fy = y as f32;
            let col = (fx / ppc).floor();
            let row = (fy / ppc).floor();
            let p = (row + col).rem_euclid(2.0) as usize;
            let mut c = self.colors[p];
            //let r = (255.0 * p as f32) / 255.0;
            //let b = (32.0 * row as f32) / 255.0;
            //*pixel = image::Rgba([r, 0.0, 0.0, 1.0]);
            let nx = fx / size;
            let ny = fy / size;
            for layer in self.layers.iter() {
                c = layer.apply(nx, ny, p, &colors, c);
            }
            *pixel = c;
        }

        for overlay in self.overlays.iter() {
            for (x, y, pixel) in buffer.enumerate_pixels_mut() {
                let o = overlay.get_pixel(x, y);
                pixel.blend(&o);
            }
        }
        let buffer = DynamicImage::ImageRgba32F(buffer).into_rgba8();
        self.buffer = Some(buffer);
    }
    pub fn save_as_png(&self, path: &Path) {
        if let Some(buffer) = &self.buffer {
            let _todo = buffer
                .save_with_format(path, ImageFormat::Png)
                .expect("Can save PNG");
        } else {
            // :TODO:
        }
    }
}
