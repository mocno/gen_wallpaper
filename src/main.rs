mod generator;
mod types;

use clap::Parser;

use crate::types::Resolution;

/// Gerador simples de wallpapers aleatórios e bonitos.
/// A ideia desse projeto é gerar de forma simples e rápida planos de fundos, mas sem perder a beleza e o estilo que seu plano de fundo precisa :P
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Nome da imagem a ser criada
    filepath: String,

    /// Resolução do papel de parede
    #[arg(short, long, default_value = "full-hd")]
    resolution: Resolution,
}

fn main() {
    let args = Args::parse();

    let resolution = args.resolution.size();
    let wp = generator::generator(resolution);
    wp.save(args.filepath).unwrap();
}
