use std::f32::consts::PI;

use image::Rgb;
use palette::Srgb;
use rand::{Rng, RngExt};

use crate::{
    pallete_generator::{convert_rgb, generate_random_style_colors},
    types::{RandomDotsWallpaper, XYZWallpaper},
};

const BACKGROUND: Rgb<u8> = Rgb([20, 20, 20]);

const FUNCS_1INPUT: [fn(f32) -> f32; 3] = [
    |a: f32| a * a.abs(),
    |a: f32| (a * PI).cos(),
    |a: f32| (a * 20.0).round() / 20.0,
];

const FUNCS_2INPUTS: [fn(f32, f32) -> f32; 3] = [
    |a: f32, b: f32| a + b,
    |a: f32, b: f32| a * b,
    |a: f32, b: f32| a - b,
];

const COLORED_PIXEL_BY_PIXELS_IN_DOTS: f32 = 0.4;

fn linear_color_map_vec(t: f32, colors: Vec<Srgb>) -> Srgb {
    let num_colors = colors.len() as f32;
    let i = (t * num_colors).div_euclid(1.0);
    let new_t = (t * num_colors).rem_euclid(1.0);
    let i = i.rem_euclid(num_colors) as usize;
    let next_i = if i + 1 < colors.len() { i + 1 } else { 0 };

    let color1 = colors[i];
    let color2 = colors[next_i];

    (color2 - color1) * new_t + color1
}

fn apply_1input_func(p: f32, a: f32) -> f32 {
    let f = FUNCS_1INPUT[(p * FUNCS_1INPUT.len() as f32) as usize];

    f(a)
}

fn apply_2input_func(p: f32, a: f32, b: f32, r: f32) -> f32 {
    if p < 0.5 {
        let p = 2.0 * p;
        let f = FUNCS_2INPUTS[(p * FUNCS_2INPUTS.len() as f32) as usize];

        f(r, b)
    } else {
        let p = 2.0 * p - 1.0;
        let f = FUNCS_2INPUTS[(p * FUNCS_2INPUTS.len() as f32) as usize];

        f(a, r)
    }
}

fn random_function<R>(rng: &mut R, a: f32, b: f32, n1: usize, n2: usize) -> f32
where
    R: Rng + Sized,
{
    let p = rng.random();

    if n1 == 0 && n2 == 0 {
        p
    } else if rng.random_ratio(n1 as u32, (n1 + n2) as u32) {
        let a = random_function(rng, a, b, n1 - 1, n2);

        apply_1input_func(p, a)
    } else {
        let r = random_function(rng, a, b, n1, n2 - 1);

        apply_2input_func(p, a, b, r)
    }
}

fn create_colormap<R>(
    rng: &mut R,
    x: f32,
    y: f32,
    n1: usize,
    n2: usize,
    colors: Vec<Srgb>,
) -> Rgb<u8>
where
    R: Rng + Sized + Clone,
{
    let value = random_function(rng, x, y, n1, n2);
    let color = linear_color_map_vec(value, colors);
    convert_rgb(color)
}

pub fn dots_generator<R>(rng: &mut R, resolution: (u32, u32)) -> RandomDotsWallpaper
where
    R: Rng + Sized + Clone,
{
    let (n1, n2, n1_color, n2_color, num_colors_range) = (5, 20, 3, 3, 2..6);

    let num_colored_dots =
        ((resolution.0 * resolution.1) as f32 * COLORED_PIXEL_BY_PIXELS_IN_DOTS) as u32;
    let num_colors = rng.random_range(num_colors_range.clone());
    let colors = generate_random_style_colors(rng, num_colors);

    let mut wp = RandomDotsWallpaper::new(resolution, BACKGROUND);
    wp.add_normal_colored_dots(
        &mut rng.clone(),
        |x, y| {
            let mut rng_seeded = rng.clone();

            let (px, py) = (
                random_function(&mut rng_seeded, x, y, n1, n2),
                random_function(&mut rng_seeded, x, y, n1, n2),
            );
            let color = create_colormap(&mut rng_seeded, x, y, n1_color, n2_color, colors.clone());

            ((px, py), color)
        },
        num_colored_dots,
    );

    wp
}

pub fn xyz_generator<R>(rng: &mut R, resolution: (u32, u32)) -> XYZWallpaper
where
    R: Rng + Sized + Clone,
{
    let (n1, n2, num_colors_range) = (15, 10, 3..8);

    let num_colors = rng.random_range(num_colors_range);
    let colors = generate_random_style_colors(rng, num_colors);

    let mut wp = XYZWallpaper::new(resolution);
    wp.paint(|x, y| create_colormap(&mut rng.clone(), x, y, n1, n2, colors.clone()));

    wp
}

pub fn tiles_generator<R>(rng: &mut R, resolution: (u32, u32)) -> XYZWallpaper
where
    R: Rng + Sized + Clone,
{
    let (n1, n2, num_colors_range) = (15, 10, 3..8);

    let num_colors = rng.random_range(num_colors_range);
    let colors = generate_random_style_colors(rng, num_colors);

    let mut wp = XYZWallpaper::new(resolution);
    wp.paint(|x, y| create_colormap(&mut rng.clone(), x, y, n1, n2, colors.clone()));

    wp
}
