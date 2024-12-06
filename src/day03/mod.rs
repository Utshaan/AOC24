use regex::Regex;

#[allow(unused_imports)]
use crate::{get_example_input, get_input, print_day};

pub fn solve() {
    print_day("DAY 03");
    let content = get_input("DAY 03");

    println!(
        "Part A : {}\nPart B : {}",
        mul_solve(&content),
        do_mul_solve(&content)
    );
}

fn mul_solve(data: &str) -> i32 {
    let reg = Regex::new(r"mul\((?<n1>[0-9]+),(?<n2>[0-9]+)\)").unwrap();

    reg.captures_iter(data)
        .map(|caps| {
            let (_, [n1, n2]) = caps.extract();
            n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()
        })
        .sum()
}

fn do_mul_solve(data: &str) -> i32 {
    let reg_end = Regex::new(r"(do\(\)|don't\(\)|mul\((?<n1>[0-9]+),(?<n2>[0-9]+)\))").unwrap();

    let mut todo = true;
    reg_end
        .captures_iter(data)
        .filter(|cap| {
            let cmd = cap.get(0).unwrap().as_str();
            match cmd {
                "do()" => {
                    todo = true;
                    false
                }
                "don't()" => {
                    todo = false;
                    false
                }
                _ => todo,
            }
        })
        .map(|cap| {
            let n1 = cap.get(2).unwrap().as_str();
            let n2 = cap.get(3).unwrap().as_str();
            n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()
        })
        .sum()
}
