use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;

static mut WIDTH: u16 = 0;

fn main() {
    unsafe {
        WIDTH = termion::terminal_size().unwrap().0;
    }
    day01::solve();
    day02::solve();
    day03::solve();
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

fn get_input(day: &str) -> String {
    fs::read_to_string(format!(
        "src/{}/input.txt",
        day.to_lowercase().replace(" ", "")
    ))
    .expect("Issue in reading input.txt. Make sure the file exists and the permissions are right")
}

#[allow(dead_code)]
fn get_example_input(day: &str) -> String {
    fs::read_to_string(format!(
        "src/{}/example.txt",
        day.to_lowercase().replace(" ", "")
    ))
    .expect("Issue in reading input.txt. Make sure the file exists and the permissions are right")
}
