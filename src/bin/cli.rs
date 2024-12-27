use clap::Parser;
use uranium_cli::uranium::Cli;

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);
}
