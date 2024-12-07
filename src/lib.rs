use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub static mut WIDTH: u16 = 0;

pub fn solve_day(day: u8) {
    let ans = match day {
        ones if ones < 10 => {
            print_day(&format!("DAY 0{}", ones));
            let input = get_input(&format!("DAY 0{}", ones));
            match ones {
                1 => Some(day01::solve(input)),
                2 => Some(day02::solve(input)),
                3 => Some(day03::solve(input)),
                4 => Some(day04::solve(input)),
                _ => None,
            }
        }
        tens => {
            print_day(&format!("DAY {}", tens));
            None
        }
    };

    match ans {
        Some((part1, part2)) => println!("Part One : {}\nPart Two : {}", part1, part2),
        None => println!("No solution yet!"),
    }
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

fn get_input(day_str: &str) -> String {
    fs::read_to_string(format!(
        "src/{}/input.txt",
        day_str.to_lowercase().replace(" ", "")
    ))
    .expect("Issue in reading input.txt. Make sure the file exists and the permissions are right")
}

#[allow(dead_code)]
fn get_example_input(day_str: &str) -> String {
    fs::read_to_string(format!(
        "src/{}/example.txt",
        day_str.to_lowercase().replace(" ", "")
    ))
    .expect("Issue in reading input.txt. Make sure the file exists and the permissions are right")
}
