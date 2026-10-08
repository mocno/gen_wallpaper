use image::Rgb;
use rand::{seq::IndexedRandom, Rng, RngExt};
use std::f32::consts::PI;

use crate::{
    palette_generator::generate_random_style_colors,
    types::{RandomDotsWallpaper, TilesWallpaper, XYZWallpaper},
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

fn linear_color_map_vec(t: f32, colors: &[Rgb<u8>]) -> Rgb<u8> {
    let num_colors = colors.len() as f32;
    let i = (t * num_colors).div_euclid(1.0);
    let new_t = (t * num_colors).rem_euclid(1.0);
    let i = i.rem_euclid(num_colors) as usize;
    let next_i = if i + 1 < colors.len() { i + 1 } else { 0 };

    let (color1, color2) = (colors[i].0, colors[next_i].0);

    Rgb::from([
        ((color2[0] - color1[0]) as f32 * new_t + color1[0] as f32) as u8,
        ((color2[1] - color1[1]) as f32 * new_t + color1[1] as f32) as u8,
        ((color2[2] - color1[2]) as f32 * new_t + color1[2] as f32) as u8,
    ])
}

fn random_function<R>(rng: &mut R, a: f32, b: f32, n1: usize, n2: usize) -> f32
where
    R: Rng + Sized,
{
    if n1 == 0 && n2 == 0 {
        rng.random()
    } else if rng.random_ratio(n1 as u32, (n1 + n2) as u32) {
        let r = random_function(rng, a, b, n1 - 1, n2);
        let f = FUNCS_1INPUT.choose(rng).unwrap();
        f(r)
    } else {
        let r = random_function(rng, a, b, n1, n2 - 1);
        let f = FUNCS_2INPUTS.choose(rng).unwrap();

        if rng.random_bool(0.5) {
            f(r, b)
        } else {
            f(a, r)
        }
    }
}

fn create_colormap<R>(
    rng: &mut R,
    x: f32,
    y: f32,
    n1: usize,
    n2: usize,
    colors: &[Rgb<u8>],
) -> Rgb<u8>
where
    R: Rng + Sized + Clone,
{
    let value = random_function(rng, x, y, n1, n2);
    linear_color_map_vec(value, colors)
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
            let color = create_colormap(&mut rng_seeded, x, y, n1_color, n2_color, &colors);

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
    wp.paint(|x, y| create_colormap(&mut rng.clone(), x, y, n1, n2, &colors));
    wp
}

pub fn tiles_generator<R>(rng: &mut R, resolution: (u32, u32)) -> TilesWallpaper
where
    R: Rng + Sized + Clone,
{
    let (n1, n2, num_colors_range, size_range, length_range) = (5, 10, 5..8, 2..8, 1.0..4.0);

    let num_colors = rng.random_range(num_colors_range);
    let colors = generate_random_style_colors(rng, num_colors);
    let size = rng.random_range(size_range) * 10;
    let length = rng.random_range(length_range);

    let mut wp = TilesWallpaper::new(resolution, size);
    wp.paint(|i, j| create_colormap(&mut rng.clone(), length * i, length * j, n1, n2, &colors));
    wp
}
