use image::Rgb;
use palette::{FromColor, Hsl, Srgb};
use rand::{Rng, RngExt};

pub fn generate_monocromatic_colors<R>(rng: &mut R, num_colors: u32) -> Vec<Rgb<u8>>
where
    R: Rng + Sized + Clone,
{
    let hue = rng.random::<f32>() * 360.0;
    let coeff = (num_colors + 1).pow(2) as f32;

    (1..=num_colors)
        .map(|i| hsl_color(hue, 1f32, (i * i) as f32 / coeff))
        .collect()
}

pub fn generate_analogous_colors<R>(rng: &mut R, num_colors: u32) -> Vec<Rgb<u8>>
where
    R: Rng + Sized + Clone,
{
    let hue = rng.random::<f32>() * 360.0;
    let hue_angle = 45.0;
    let lightness = rng.random::<f32>() * 0.3 + 0.4;

    (0..num_colors)
        .map(|i| {
            let k = 2.0 * (i as f32 / (num_colors - 1) as f32) - 1.0;
            hsl_color((hue + k * hue_angle) % 360.0, 1f32, lightness)
        })
        .collect()
}

pub fn generate_complementary_colors<R>(rng: &mut R, num_colors: u32) -> Vec<Rgb<u8>>
where
    R: Rng + Sized + Clone,
{
    let hue = rng.random::<f32>() * 360.0;
    let complementary_hue = (hue + 180.0) % 360.0;

    (0..num_colors)
        .map(|i| {
            if i <= num_colors / 2 {
                let lightness = 1.0 - 2.0 * (i + 1) as f32 / (num_colors + 1) as f32;
                hsl_color(hue, 1f32, lightness)
            } else {
                let lightness = 2.0 * i as f32 / (num_colors + 1) as f32 - 1.0;
                hsl_color(complementary_hue, 1f32, lightness)
            }
        })
        .collect()
}

pub fn generate_random_style_colors<R>(rng: &mut R, num_colors: u32) -> Vec<Rgb<u8>>
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
fn hsl_color(hue: f32, saturation: f32, lightness: f32) -> Rgb<u8> {
    let color = Srgb::from_color(Hsl::new(hue, saturation, lightness));

    Rgb([
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
    ])
}
