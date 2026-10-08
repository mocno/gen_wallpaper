use crate::{
    generator::{create_colormap, random_function, BACKGROUND},
    palette_generator::generate_random_style_colors,
};
use clap::ValueEnum;
use image::{ImageResult, Rgb, RgbImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use rand::{Rng, RngExt};
use rand_distr::{Distribution, Normal};
use std::path::Path;

#[derive(Debug, ValueEnum, Clone)]
pub enum Resolution {
    HD,
    FullHD,
    _4k,
}

impl Resolution {
    pub fn size(self) -> (u32, u32) {
        match self {
            Self::HD => (1280, 720),
            Self::FullHD => (1920, 1080),
            Self::_4k => (4096, 2160),
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
    pub fn new(resolution: Resolution, background: Rgb<u8>) -> Self {
        let resolution = resolution.size();
        let mut wp = Self {
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

    pub fn generator<R>(rng: &mut R, resolution: Resolution) -> Self
    where
        R: Rng + Sized + Clone,
    {
        let (n1, n2, n1_color, n2_color, num_colors_range, coeff_colored_pixels) =
            (5, 20, 3, 3, 2..6, 0.4);

        let resolution_sizes = resolution.clone().size();
        let num_colored_dots =
            ((resolution_sizes.0 * resolution_sizes.1) as f32 * coeff_colored_pixels) as u32;
        let num_colors = rng.random_range(num_colors_range.clone());
        let colors = generate_random_style_colors(rng, num_colors);

        let mut wp = Self::new(resolution, BACKGROUND);
        wp.add_normal_colored_dots(
            &mut rng.clone(),
            |x, y| {
                let mut rng_seeded = rng.clone();

                let (px, py) = (
                    random_function(&mut rng_seeded, x, y, n1, n2),
                    random_function(&mut rng_seeded, x, y, n1, n2),
                );
                let color = create_colormap(&mut rng_seeded, x, y, n1_color, n2_color, &colors);

                ((px, py), color)
            },
            num_colored_dots,
        );

        wp
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
    pub fn new(resolution: Resolution) -> Self {
        let resolution = resolution.size();
        Self {
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

    pub fn generator<R>(rng: &mut R, resolution: Resolution) -> Self
    where
        R: Rng + Sized + Clone,
    {
        let (n1, n2, num_colors_range) = (15, 10, 3..8);

        let num_colors = rng.random_range(num_colors_range);
        let colors = generate_random_style_colors(rng, num_colors);

        let mut wp = Self::new(resolution);
        wp.paint(|x, y| create_colormap(&mut rng.clone(), x, y, n1, n2, &colors));
        wp
    }
}

impl Save for XYZWallpaper {
    fn save<Q: AsRef<Path>>(self, path: Q) -> ImageResult<()> {
        self.image.save(path)
    }
}

pub struct TilesWallpaper {
    image: RgbImage,
    size: u32,
}

impl TilesWallpaper {
    pub fn new(resolution: Resolution, size: u32) -> Self {
        let resolution = resolution.size();
        Self {
            image: RgbImage::new(resolution.0, resolution.1),
            size,
        }
    }

    pub fn paint(&mut self, dot_color: impl Fn(f32, f32) -> Rgb<u8>) {
        let (width, height) = (
            self.image.width().div_ceil(self.size),
            self.image.height().div_ceil(self.size),
        );

        let inicial_position = (
            (width * self.size - self.image.width()) / 2,
            (height * self.size - self.image.height()) / 2,
        );

        for i in 0..=width {
            for j in 0..=height {
                let (x, y) = (
                    2.0 * i as f32 / width as f32 - 1.0,
                    2.0 * j as f32 / height as f32 - 1.0,
                );
                let color = dot_color(x, y);
                let rect = Rect::at(
                    (i * self.size - inicial_position.0) as i32,
                    (j * self.size - inicial_position.1) as i32,
                )
                .of_size(self.size, self.size);

                draw_filled_rect_mut(&mut self.image, rect, color);
            }
        }
    }

    pub fn generator<R>(rng: &mut R, resolution: Resolution) -> Self
    where
        R: Rng + Sized + Clone,
    {
        let (n1, n2, num_colors_range, size_range, length_range) = (5, 10, 5..8, 2..8, 1.0..4.0);

        let num_colors = rng.random_range(num_colors_range);
        let colors = generate_random_style_colors(rng, num_colors);
        let size = rng.random_range(size_range) * 10;
        let length = rng.random_range(length_range);

        let mut wp = Self::new(resolution, size);
        wp.paint(|i, j| create_colormap(&mut rng.clone(), length * i, length * j, n1, n2, &colors));
        wp
    }
}

impl Save for TilesWallpaper {
    fn save<Q: AsRef<Path>>(self, path: Q) -> ImageResult<()> {
        self.image.save(path)
    }
}
