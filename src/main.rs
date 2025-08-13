mod cats;

use rand::rng;
use rand::seq::IndexedRandom;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rats")]
#[command(version = "1.0")]
#[command(about = "Print a random ASCII art")]
struct Cli {
    #[arg(long)]
    install: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.install {
        install_binary();
        return;
    }

    show_random_cat();
}

fn show_random_cat() {
    let arts = cats::load_arts();
    let mut rng = rng();
    if let Some(art) = arts.choose(&mut rng) {
        println!("{}", art);
    }
}

fn install_binary() {
    use std::path::Path;

    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let local_bin = home_dir.join(".local/bin");
    let target_path = local_bin.join("rats");

    if Path::new(&target_path).exists() {
        println!("rats is already installed at {}", target_path.display());
        return;
    }

    let exe_path = std::env::current_exe().expect("Failed to get current executable path");

    if !local_bin.exists() {
        std::fs::create_dir_all(&local_bin).expect("Failed to create ~/.local/bin");
    }

    std::fs::copy(&exe_path, &target_path).expect("Failed to copy binary");

    println!("successfully installed to {}", target_path.display());
}