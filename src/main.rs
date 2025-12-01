mod generator;
mod types;

use clap::Parser;

use crate::types::{Resolution, Wallpaper};

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
    },

    /// Gera um wallpaper baseado numa função que mapeia cada pixel da tela a uma cor
    Xyz {
        /// Nome da imagem a ser criada
        filepath: String,

        /// Resolução do papel de parede
        #[arg(short, long, default_value = "full-hd")]
        resolution: Resolution,
    },

    /// Gera um wallpaper de qualquer tipo
    Random {
        /// Nome da imagem a ser criada
        filepath: String,

        /// Resolução do papel de parede
        #[arg(short, long, default_value = "full-hd")]
        resolution: Resolution,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Dots {
            filepath,
            resolution,
        } => {
            let resolution = resolution.size();
            let wp = generator::dots_generator(resolution);
            wp.save(filepath).unwrap();
        }
        Commands::Xyz {
            filepath,
            resolution,
        } => {
            let resolution = resolution.size();
            let wp = generator::xyz_generator(resolution);
            wp.save(filepath).unwrap();
        }
        Commands::Random {
            filepath,
            resolution,
        } => {
            let resolution = resolution.size();
            generator::save_wallpaper_random(resolution, filepath).unwrap();
        }
    }
}
