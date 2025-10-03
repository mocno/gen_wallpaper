use std::path::Path;

use image::{ImageResult, Rgb, RgbImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use rand::Rng;
use rand_distr::{Distribution, Normal};

pub struct RandomDotsWallpaper {
    image: RgbImage,
}

impl RandomDotsWallpaper {
    pub fn new(resolution: (u32, u32), background: Rgb<u8>) -> Self {
        let mut wp = RandomDotsWallpaper {
            image: RgbImage::new(resolution.0, resolution.1),
        };
        draw_filled_rect_mut(
            &mut wp.image,
            Rect::at(0, 0).of_size(resolution.0, resolution.1),
            background,
        );
        wp
    }

    pub fn add_dot(&mut self, dot: (f32, f32), color: Rgb<u8>) {
        let width = self.image.width() as f32;
        let height = self.image.height() as f32;

        let x = 0.5 * (dot.0 + 1.0) * width;
        let y = 0.5 * (dot.1 + 1.0) * height;

        if 0.0 <= x && x < width && 0.0 <= y && y < height {
            self.image.put_pixel(x as u32, y as u32, color);
        }
    }

    pub fn add_normal_colored_dots<R>(
        &mut self,
        mut rng: &mut R,
        colored_dot: impl Fn(f32, f32) -> ((f32, f32), Rgb<u8>),
        num: i32,
    ) where
        R: Rng + Sized,
    {
        let normal = Normal::new(0.0, 0.8).unwrap();
        for _ in 0..num {
            let (x, y): (f32, f32) = (normal.sample(&mut rng), normal.sample(&mut rng));
            let (dot, color) = colored_dot(x, y);

            self.add_dot(dot, color);
        }
    }

    pub fn save<Q: AsRef<Path>>(self, path: Q) -> ImageResult<()> {
        self.image.save(path)
    }
}
