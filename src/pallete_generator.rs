use rand::{Rng, RngExt};

use image::Rgb;
use palette::{FromColor, Hsl, Srgb};

pub fn generate_monocromatic_colors<R>(rng: &mut R, num_colors: u32) -> Vec<Srgb<f32>>
where
    R: Rng + Sized + Clone,
{
    let hue = rng.random::<f32>() * 360.0;
    let colors = (1..=num_colors)
        .map(|i| {
            let k = (i as f32 / (num_colors + 1) as f32).powi(2);
            Srgb::from_color(Hsl::new(hue, 1f32, k))
        })
        .collect();
    return colors;
}

pub fn generate_analogous_colors<R>(rng: &mut R, num_colors: u32) -> Vec<Srgb<f32>>
where
    R: Rng + Sized + Clone,
{
    let hue = rng.random::<f32>() * 360.0;
    let hue_angle = 45.0;
    let lightness = rng.random::<f32>() * 0.3 + 0.4;

    let colors = (0..num_colors)
        .map(|i| {
            let k = 2.0 * (i as f32 / (num_colors - 1) as f32) - 1.0;
            Srgb::from_color(Hsl::new((hue + k * hue_angle) % 360.0, 1f32, lightness))
        })
        .collect();
    return colors;
}

pub fn generate_complementary_colors<R>(rng: &mut R, num_colors: u32) -> Vec<Srgb<f32>>
where
    R: Rng + Sized + Clone,
{
    let hue = rng.random::<f32>() * 360.0;
    let complementary_hue = hue + if hue < 180.0 { 180.0 } else { -180.0 };

    let colors = (0..num_colors)
        .map(|i| {
            if i <= num_colors / 2 {
                let lightness = 1.0 - 2.0 * (i + 1) as f32 / (num_colors + 1) as f32;
                Srgb::from_color(Hsl::new(hue, 1f32, lightness))
            } else {
                let lightness = 2.0 * i as f32 / (num_colors + 1) as f32 - 1.0;
                Srgb::from_color(Hsl::new(complementary_hue, 1f32, lightness))
            }
        })
        .collect();
    return colors;
}

pub fn generate_random_style_colors<R>(rng: &mut R, num_colors: u32) -> Vec<Srgb<f32>>
where
    R: Rng + Sized + Clone,
{
    match rng.random_range(..3u8) {
        0u8 => generate_complementary_colors(rng, num_colors),
        1u8 => generate_analogous_colors(rng, num_colors),
        2u8 => generate_monocromatic_colors(rng, num_colors),
        _ => unreachable!(),
    }
}

#[inline]
pub fn convert_rgb(color: Srgb<f32>) -> Rgb<u8> {
    Rgb([
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
    ])
}
