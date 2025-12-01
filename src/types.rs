use std::path::Path;

use clap::ValueEnum;
use image::{ImageResult, Rgb, RgbImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use rand::Rng;
use rand_distr::{Distribution, Normal};

#[derive(Debug, ValueEnum, Clone)]
pub enum Resolution {
    HD,
    FullHD,
    _4k,
}

pub const RESOLUTION_HD: (u32, u32) = (1280, 720);
pub const RESOLUTION_FULLHD: (u32, u32) = (1920, 1080);
pub const RESOLUTION_4K: (u32, u32) = (4096, 2160);

impl Resolution {
    pub fn size(self) -> (u32, u32) {
        match self {
            Self::HD => RESOLUTION_HD,
            Self::FullHD => RESOLUTION_FULLHD,
            Self::_4k => RESOLUTION_4K,
        }
    }
}

pub trait Save {
    fn save<Q: AsRef<Path>>(self, path: Q) -> ImageResult<()>;
}

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
        num: u32,
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
}
impl Save for RandomDotsWallpaper {
    fn save<Q: AsRef<Path>>(self, path: Q) -> ImageResult<()> {
        self.image.save(path)
    }
}

pub struct XYZWallpaper {
    image: RgbImage,
}

impl XYZWallpaper {
    pub fn new(resolution: (u32, u32)) -> Self {
        XYZWallpaper {
            image: RgbImage::new(resolution.0, resolution.1),
        }
    }

    pub fn paint(&mut self, dot_color: impl Fn(f32, f32) -> Rgb<u8>) {
        for i in 0..self.image.width() {
            for j in 0..self.image.height() {
                let x = 2.0 * (i as f32 / self.image.width() as f32) - 1.0;
                let y = 2.0 * (j as f32 / self.image.height() as f32) - 1.0;
                let color = dot_color(x, y);

                self.image.put_pixel(i, j, color);
            }
        }
    }
}

impl Save for XYZWallpaper {
    fn save<Q: AsRef<Path>>(self, path: Q) -> ImageResult<()> {
        self.image.save(path)
    }
}
