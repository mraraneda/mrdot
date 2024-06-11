#![allow(unused)]

use clap::{Args, Parser, Subcommand};
use env_logger::Env;

use mrdot::configurations::get_configuration;
use mrdot::engine::actions;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(
    about = "mrdot is a CLI for dotfiles management",
    long_about = "mrdot is a CLI that automates the installation of dotfiles and other setup tasks"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Capture,
}

#[derive(Args)]
struct Deploy {
    #[arg(short = 'd', long = "with-install")]
    install: bool,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let config = get_configuration().expect("Failed to read config yaml file");
    let cli = Cli::parse();

    // TODO
    //  Cuales son las opciones?:
    //  [✓] Capture
    //      Buscar los archivos originales y los traerá a la base dotfiles.
    //  [] Deploy
    //      Creará los symlinks a los elementos que tenga en la base, de acuerdo a la directiva.
    //  [] Resolución de conflictos: Qué hará la APP cuando encuentre un archivo en lugar de un symlink?
    //  [] Install
    //      Instalará los packages indicados en la directiva, serán de 3 tipos: Hombrew cask, Homebrew formulae y github.
    //

    match &cli.command {
        Commands::Capture => actions::capture_iter(&config.dotfiles).unwrap(),
    }
}
