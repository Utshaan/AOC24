pub mod day01;
pub mod day02;

static mut WIDTH: u16 = 0;

fn main() {
    unsafe {
        WIDTH = termion::terminal_size().unwrap().0;
    }
    day01::solve();
    day02::solve();
}

fn print_day(day: &str) {
    unsafe {
        println!(
            " {} {} {} ",
            (5..WIDTH / 2).map(|_| '-').collect::<String>(),
            day,
            (5..WIDTH / 2).map(|_| '-').collect::<String>()
        );
    }
}
