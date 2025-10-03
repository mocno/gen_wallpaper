use std::{env, f32::consts::PI};

use image::Rgb;
use rand::Rng;

use crate::types::RandomDotsWallpaper;

mod types;

pub const RESOLUTION_HD: (u32, u32) = (1280, 720);
pub const RESOLUTION_FULLHD: (u32, u32) = (1920, 1080);
pub const RESOLUTION_4K: (u32, u32) = (4096, 2160);

const BACKGROUND: Rgb<u8> = Rgb([20, 20, 20]);

const FUNCS_1INPUT: [fn(f32) -> f32; 6] = [
    |a: f32| a,
    |a: f32| -a,
    |a: f32| a * a.abs(),
    |a: f32| (a * 10.0).round() / 10.0,
    |a: f32| (a * PI / 2.0).sin(),
    |a: f32| (a * PI / 2.0).cos(),
];

const FUNCS_2INPUTS: [fn(f32, f32) -> f32; 4] = [
    |a: f32, b: f32| a + b,
    |a: f32, b: f32| a - b,
    |a: f32, b: f32| b - a,
    |a: f32, b: f32| a * b.abs(),
];

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

const LEAVE_PROBABILY: f32 = 0.01;
const FUNC_1INPUT_PROBABILY: f32 = 0.2;

fn linear_color_map(t: f32, color1: [f32; 3], color2: [f32; 3]) -> Rgb<u8> {
    let base = [color1[0] * 255.0, color1[1] * 255.0, color1[2] * 255.0];
    let delta = [
        (color2[0] - color1[0]) * 255.0,
        (color2[1] - color1[1]) * 255.0,
        (color2[2] - color1[2]) * 255.0,
    ];

    Rgb([
        (t * delta[0] + base[0]) as u8,
        (t * delta[1] + base[1]) as u8,
        (t * delta[2] + base[2]) as u8,
    ])
}

fn rn_f(p: f32, a: f32, b: f32) -> f32 {
    if p < LEAVE_PROBABILY {
        let p = p / LEAVE_PROBABILY;

        if p < 0.5 {
            a
        } else {
            b
        }
    } else {
        let p = (p - LEAVE_PROBABILY) / (1.0 - LEAVE_PROBABILY);

        if p < FUNC_1INPUT_PROBABILY {
            let p = p / FUNC_1INPUT_PROBABILY;
            let f = FUNCS_1INPUT[(p * FUNCS_1INPUT.len() as f32) as usize];

            f(rn_f(p, a, b))
        } else {
            let p = (p - FUNC_1INPUT_PROBABILY) / (1.0 - FUNC_1INPUT_PROBABILY);

            if p < 0.5 {
                let p = 2.0 * p;
                let f = FUNCS_2INPUTS[(p * FUNCS_2INPUTS.len() as f32) as usize];

                f(rn_f(p, a, b), b)
            } else {
                let p = 2.0 * p - 1.0;
                let f = FUNCS_2INPUTS[(p * FUNCS_2INPUTS.len() as f32) as usize];

                f(a, rn_f(p, a, b))
            }
        }
    }
}

fn color_map(p: f32, a: f32, b: f32) -> Rgb<u8> {
    let (a, b) = ((a + 1.0) / 2.0, (b + 1.0) / 2.0);
    let value = COLOR_MAPS[(p * COLOR_MAPS.len() as f32) as usize](a, b);
    let p = (p - (p * COLOR_MAPS.len() as f32).floor() / COLOR_MAPS.len() as f32)
        * COLOR_MAPS.len() as f32;
    let [color1, color2] = LINEAR_COLOR_MAP[(p * LINEAR_COLOR_MAP.len() as f32) as usize];

    linear_color_map(value, color1, color2)
}

fn rn_f2(p1: f32, p2: f32, x: f32, y: f32) -> (f32, f32) {
    (rn_f(p1, x, y), rn_f(p2, y, x))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut rng = rand::rng();
    let p_color_map: f32 = rng.random();
    let p1: f32 = rng.random();
    let p2: f32 = rng.random();

    let filename = if args.len() == 2 {
        args[1].to_owned()
    } else {
        format!(
            "./imgs/wallpaper-{}x{}-p({}, {}).png",
            RESOLUTION_4K.0, RESOLUTION_4K.1, p1, p2
        )
    };
    let mut wp = RandomDotsWallpaper::new(RESOLUTION_4K, BACKGROUND);
    wp.add_normal_colored_dots(
        &mut rng,
        |x, y| (rn_f2(p1, p2, x, y), color_map(p_color_map, x, y)),
        1_000_000,
    );

    wp.save(&filename)?;
    Ok(())
}
