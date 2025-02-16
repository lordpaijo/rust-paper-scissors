mod game;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None, 
    name = "Rust Paper Scissors")]

/* test */

struct Args {
    #[arg(short, long)] rounds: Option<u64>, 
    #[arg(short, long)] auto: bool,
    #[arg(short, long)] skip: Option<u64>, 
    #[arg(long, value_delimiter = ',')] skip_rounds: Option<Vec<u64>>,
    #[arg(long)] skip_even: bool, 
    #[arg(long)] skip_odd: bool,
    #[arg(long)] skip_prime: bool,
    #[arg(long)] skip_all: bool,
    #[arg(long)] set_start_round: Option<u64>,
    #[arg(long)] set_even_round: bool, 
    #[arg(long)] set_odd_round: bool,
    #[arg(long)] set_prime_round: bool,
}

fn main() {
    let args = Args::parse();
    let start_round = args.set_start_round.unwrap_or(1);
    game::run(
        start_round, args.rounds, args.auto,
        args.skip, args.skip_rounds, args.skip_even,
        args.skip_odd, args.skip_prime, args.skip_all, 
        args.set_even_round, args.set_odd_round, args.set_prime_round,
    );
}
