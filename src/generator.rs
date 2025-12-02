use std::{f32::consts::PI, usize};

use image::Rgb;
use rand::Rng;

use crate::types::{RandomDotsWallpaper, XYZWallpaper};

const BACKGROUND: Rgb<u8> = Rgb([20, 20, 20]);

const FUNCS_1INPUT: [fn(f32) -> f32; 5] = [
    |a: f32| -a,
    |a: f32| a * a.abs(),
    |a: f32| (a * 20.0).round() / 40.0,
    |a: f32| (a * PI).sin(),
    |a: f32| (a * PI).cos(),
];

const FUNCS_2INPUTS: [fn(f32, f32) -> f32; 2] = [|a: f32, b: f32| a + b, |a: f32, b: f32| a * b];

const COLOR_MAPS: [fn(f32, f32) -> f32; 7] = [
    |a, b| 2.0 * (((a - 0.5) * (a - 0.5) + (b - 0.5) * (b - 0.5)) / 2.0).sqrt(),
    |a, b| 2.0 * a * b - a - b + 1.0,
    |a, b| (20.0 * (a - 0.5) * (b - 0.5)).floor() / 10.0 + 0.5,
    |a, b| ((a * PI * 2.0).sin() * (b * PI * 2.0).sin() + 1.0) / 2.0,
    |a, b| (a + b) / 2.0,
    |_a, b| ((b * PI * 10.0).sin() + 1.0) / 2.0,
    |a, _b| ((a * PI * 8.0).cos() + 1.0) / 2.0,
];

static LINEAR_COLOR_MAP: [[[f32; 3]; 2]; 9] = [
    [[0.2, 0.0, 1.0], [1.0, 0.0, 0.6]],
    [[0.4, 0.2, 1.0], [1.0, 0.4, 0.4]],
    [[0.3, 0.2, 1.0], [0.0, 1.0, 0.0]],
    [[0.1, 0.1, 0.6], [0.2, 1.0, 0.8]],
    [[0.1, 0.5, 1.0], [0.4, 0.4, 0.4]],
    [[0.0, 1.0, 1.0], [1.0, 0.2, 1.0]],
    [[0.7, 0.0, 1.0], [1.0, 0.3, 0.1]],
    [[1.0, 0.0, 0.3], [1.0, 1.0, 1.0]],
    [[1.0, 1.0, 0.3], [1.0, 0.0, 0.4]],
];

fn linear_color_map<const U: usize>(t: f32, colors: [[f32; 3]; U]) -> Rgb<u8> {
    let i = ((t * U as f32) as usize) % (U - 1);

    let base0 = colors[i][0] * 255.0;
    let base1 = colors[i][1] * 255.0;
    let base2 = colors[i][2] * 255.0;
    let delta0 = (colors[i + 1][0] - colors[i][0]) * 255.0;
    let delta1 = (colors[i + 1][1] - colors[i][1]) * 255.0;
    let delta2 = (colors[i + 1][2] - colors[i][2]) * 255.0;

    Rgb([
        (t * delta0 + base0) as u8,
        (t * delta1 + base1) as u8,
        (t * delta2 + base2) as u8,
    ])
}

fn get_combination(p: f32, a: f32, b: f32) -> f32 {
    a * p + b * (1.0 - p)
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

fn rn_f1(p: f32, a: f32, b: f32, n1: usize, n2: usize) -> f32 {
    if n1 == 0 && n2 == 0 {
        get_combination(p, a, b)
    } else {
        // Número "aleatório" no intervalo [-n2, n1)
        let num = p * (n1 + n2) as f32 - n2 as f32;

        if num > 0f32 {
            let p = num / n1 as f32;
            apply_1input_func(p, rn_f1(p, a, b, n1 - 1, n2))
        } else {
            let p = -num / n2 as f32;
            apply_2input_func(p, a, b, rn_f1(p, a, b, n1, n2 - 1))
        }
    }
}

pub fn color_map(p: f32, a: f32, b: f32) -> Rgb<u8> {
    let (a, b) = ((a + 1.0) / 2.0, (b + 1.0) / 2.0);
    let value = COLOR_MAPS[(p * COLOR_MAPS.len() as f32) as usize](a, b);
    let p = (p - (p * COLOR_MAPS.len() as f32).floor() / COLOR_MAPS.len() as f32)
        * COLOR_MAPS.len() as f32;
    let colors = LINEAR_COLOR_MAP[(p * LINEAR_COLOR_MAP.len() as f32) as usize];

    linear_color_map(value, colors)
}

pub fn dots_generator<R>(rng: &mut R, resolution: (u32, u32)) -> RandomDotsWallpaper
where
    R: Rng + Sized,
{
    let p_color_map: f32 = rng.random();
    let p1: f32 = rng.random();
    let p2: f32 = rng.random();

    let (n1, n2) = (5, 20);

    let mut wp = RandomDotsWallpaper::new(resolution, BACKGROUND);
    wp.add_normal_colored_dots(
        rng,
        |x, y| {
            (
                (rn_f1(p1, x, y, n1, n2), rn_f1(p2, y, x, n1, n2)),
                color_map(p_color_map, x, y),
            )
        },
        resolution.0 * resolution.1 / 4,
    );

    wp
}

pub fn xyz_generator<R>(rng: &mut R, resolution: (u32, u32)) -> XYZWallpaper
where
    R: Rng + Sized,
{
    let mut color1: [f32; 3] = rng.random();
    let background_color = [20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0];
    let mut color3: [f32; 3] = rng.random();

    // Limita as cores para não ficar verde, pior cor :P
    color1[1] *= color1[0].max(color1[1]);
    color3[1] *= color3[0].max(color3[1]);

    let p: f32 = rng.random();
    let (n1, n2) = (20, 5);

    let mut wp = XYZWallpaper::new(resolution);
    wp.paint(|x, y| {
        linear_color_map(
            rn_f1(p, x * 1.5, y * 1.5, n1, n2),
            [background_color, color1, background_color, color3],
        )
    });

    wp
}
