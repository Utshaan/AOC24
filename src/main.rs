use aoc24::{solve_day, WIDTH};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    unsafe {
        WIDTH = termion::terminal_size().unwrap().0;
    }

    let args = Args::parse();

    solve_day(args.day);
}
