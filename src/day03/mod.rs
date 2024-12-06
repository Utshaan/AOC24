use regex::Regex;

#[allow(unused_imports)]
use crate::{get_example_input, get_input};

pub fn solve(input: String) -> (u32, u32) {
    (mul_solve(&input), do_mul_solve(&input))
}

fn mul_solve(data: &str) -> u32 {
    let reg = Regex::new(r"mul\((?<n1>[0-9]+),(?<n2>[0-9]+)\)").unwrap();

    reg.captures_iter(data)
        .map(|caps| {
            let (_, [n1, n2]) = caps.extract();
            n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap()
        })
        .sum()
}

fn do_mul_solve(data: &str) -> u32 {
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
            n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = get_example_input("DAY 03");
        assert_eq!((161, 48), solve(input));
    }
}
