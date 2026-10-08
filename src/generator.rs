use image::Rgb;
use rand::{seq::IndexedRandom, Rng, RngExt};
use std::f32::consts::PI;

pub const BACKGROUND: Rgb<u8> = Rgb([20, 20, 20]);

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

fn linear_color_map_vec(value: f32, colors: &[Rgb<u8>]) -> Rgb<u8> {
    let num_colors = colors.len() as f32;
    let i = (value * num_colors).div_euclid(1.0);
    let t = (value * num_colors).rem_euclid(1.0);
    let i = i.rem_euclid(num_colors) as usize;
    let next_i = if i + 1 < colors.len() { i + 1 } else { 0 };

    let (color1, color2) = (colors[i].0, colors[next_i].0);

    Rgb::from([
        ((color2[0] - color1[0]) as f32 * t + color1[0] as f32) as u8,
        ((color2[1] - color1[1]) as f32 * t + color1[1] as f32) as u8,
        ((color2[2] - color1[2]) as f32 * t + color1[2] as f32) as u8,
    ])
}

pub fn random_function<R>(rng: &mut R, a: f32, b: f32, n1: usize, n2: usize) -> f32
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

pub fn create_colormap<R>(
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
