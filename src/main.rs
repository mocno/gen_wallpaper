mod generator;
mod palette_generator;
mod types;

use crate::types::{RandomDotsWallpaper, Resolution, Save, TilesWallpaper, XYZWallpaper};
use clap::Parser;
use rand::{rngs::SmallRng, Rng, RngExt, SeedableRng};
use std::process::ExitCode;

/// Gerador simples de wallpapers aleatórios e bonitos.
/// A ideia desse projeto é gerar de forma simples e rápida planos de fundos, mas sem perder a beleza e o estilo que seu plano de fundo precisa :P
#[derive(Debug, Parser)]
#[command(name = "gen-wallpaper")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
#[command(version, about)]
enum Commands {
    /// Gera um wallpaper baseado em pontos posicionados e coloridos a partir de funções
    Dots {
        /// Nome da imagem a ser criada
        filepath: String,

        /// Resolução do papel de parede
        #[arg(short, long, default_value = "full-hd")]
        resolution: Resolution,

        /// Semente para gerar o wallpaper
        #[arg(short, long)]
        seed: Option<u64>,
    },

    /// Gera um wallpaper baseado numa função que mapeia cada pixel da tela a uma cor
    Xyz {
        /// Nome da imagem a ser criada
        filepath: String,

        /// Resolução do papel de parede
        #[arg(short, long, default_value = "full-hd")]
        resolution: Resolution,

        /// Semente para gerar o wallpaper
        #[arg(short, long)]
        seed: Option<u64>,
    },

    /// Gera um wallpaper baseado em uma grade
    Tiles {
        /// Nome da imagem a ser criada
        filepath: String,

        /// Resolução do papel de parede
        #[arg(short, long, default_value = "full-hd")]
        resolution: Resolution,

        /// Semente para gerar o wallpaper
        #[arg(short, long)]
        seed: Option<u64>,
    },

    /// Gera um wallpaper de qualquer tipo
    Random {
        /// Nome da imagem a ser criada
        filepath: String,

        /// Resolução do papel de parede
        #[arg(short, long, default_value = "full-hd")]
        resolution: Resolution,

        /// Semente para gerar o wallpaper - também é usado para gerar o tipo do wallpaper
        #[arg(short, long)]
        seed: Option<u64>,
    },
}

fn generate_seed() -> u64 {
    let mut rng = rand::rng();
    rng.next_u64()
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Dots {
            filepath,
            resolution,
            seed,
        } => {
            let seed = seed.unwrap_or_else(generate_seed);
            let mut rng = SmallRng::seed_from_u64(seed);

            println!("{seed}");

            let wp = RandomDotsWallpaper::generator(&mut rng, resolution);
            wp.save(filepath)
        }
        Commands::Xyz {
            filepath,
            resolution,
            seed,
        } => {
            let seed = seed.unwrap_or_else(generate_seed);
            let mut rng = SmallRng::seed_from_u64(seed);

            println!("{seed}");

            let wp = XYZWallpaper::generator(&mut rng, resolution);
            wp.save(filepath)
        }
        Commands::Tiles {
            filepath,
            resolution,
            seed,
        } => {
            let seed = seed.unwrap_or_else(generate_seed);
            let mut rng = SmallRng::seed_from_u64(seed);

            println!("{seed}");

            let wp = TilesWallpaper::generator(&mut rng, resolution);
            wp.save(filepath)
        }
        Commands::Random {
            filepath,
            resolution,
            seed,
        } => {
            let seed = seed.unwrap_or_else(generate_seed);
            let mut rng = SmallRng::seed_from_u64(seed);

            println!("{seed}");

            match rng.random_range(..=2u8) {
                0 => {
                    let wp = RandomDotsWallpaper::generator(&mut rng, resolution);
                    wp.save(filepath)
                }
                1 => {
                    let wp = XYZWallpaper::generator(&mut rng, resolution);
                    wp.save(filepath)
                }
                2 => {
                    let wp = TilesWallpaper::generator(&mut rng, resolution);
                    wp.save(filepath)
                }
                _ => unreachable!(),
            }
        }
    };

    if let Err(error) = result {
        eprintln!("Erro ao gerar a imagem: {}", error);
        return match error {
            image::ImageError::IoError(_) => ExitCode::from(2),
            image::ImageError::Unsupported(_) => ExitCode::from(3),
            _ => ExitCode::from(1),
        };
    }

    ExitCode::SUCCESS
}
