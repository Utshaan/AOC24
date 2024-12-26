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

    if args.day == 0 {
        (1..=25).for_each(|day| {
            solve_day(day);
        });
    } else {
        solve_day(args.day);
    }
}
