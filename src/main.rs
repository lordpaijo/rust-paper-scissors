mod game;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None, 
    name = "Rust Paper Scissors")]
struct Args {
    #[arg(short, long)]
    rounds: Option<u64>,
    #[arg(long)]
    auto: bool,
}

fn main() {
    let args = Args::parse();
    game::run(1, args.rounds, args.auto); }
