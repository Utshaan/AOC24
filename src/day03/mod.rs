use regex::Regex;

pub fn solve(input: String) -> (u64, u64) {
    (mul_solve(&input), do_mul_solve(&input))
}

fn mul_solve(data: &str) -> u64 {
    let reg = Regex::new(r"mul\((?<n1>\d{1,3}+),(?<n2>\d{1,3}+)\)").unwrap();

    reg.captures_iter(data)
        .map(|caps| {
            let (_, [n1, n2]) = caps.extract();
            n1.parse::<u64>().unwrap() * n2.parse::<u64>().unwrap()
        })
        .sum()
}

fn do_mul_solve(data: &str) -> u64 {
    let reg_end = Regex::new(r"(do\(\)|don't\(\)|mul\((?<n1>\d{1,3}+),(?<n2>\d{1,3}+)\))").unwrap();

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
            n1.parse::<u64>().unwrap() * n2.parse::<u64>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 03");
        assert_eq!((161, 48), solve(input));
    }
}
