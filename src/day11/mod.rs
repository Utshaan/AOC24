use std::collections::HashMap;

type Cache = HashMap<(u64, u64), u64>;

pub fn solve(input: String) -> (u64, u64) {
    let mut ans: (u64, u64) = (0, 0);

    let mut cache: Cache = HashMap::new();

    let numbers: Vec<u64> = input
        .strip_suffix('\n')
        .expect("No new line")
        .split_whitespace()
        .map(|str| str.parse::<u64>().expect("Not a number"))
        .collect();

    ans.0 = numbers
        .iter()
        .map(|num| blink_n(*num, 25, &mut cache, 1))
        .fold(0, |acc, cur| acc + cur);

    ans.1 = numbers
        .iter()
        .map(|num| blink_n(*num, 75, &mut cache, 1))
        .fold(0, |acc, cur| acc + cur);
    ans
}

fn get_digit_len(mut num: u64) -> u32 {
    let mut len = 0;

    while num > 0 {
        num /= 10;
        len += 1;
    }

    len
}

fn blink_n(num: u64, n: u64, cache: &mut Cache, ans: u64) -> u64 {
    if cache.contains_key(&(num, n)) {
        return cache.get(&(num, n)).unwrap().clone();
    }
    if n == 0 {
        return ans;
    }
    let new_ans = match num {
        0 => blink_n(1, n - 1, cache, ans),
        num => {
            let len = get_digit_len(num);
            if len % 2 == 0 {
                let fir = num / 10_i32.pow(len / 2 as u32) as u64;
                let sec = num - fir * 10_i32.pow(len / 2 as u32) as u64;
                blink_n(fir, n - 1, cache, ans) + blink_n(sec, n - 1, cache, ans)
            } else {
                blink_n(num * 2024, n - 1, cache, ans)
            }
        }
    };

    cache.insert((num, n), new_ans);
    return new_ans;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 11");
        assert_eq!((55312, 0), solve(input));
    }

    #[test]
    fn is_even() {
        assert_eq!(get_digit_len(437562), 6);
    }

    #[test]
    fn is_odd() {
        assert_eq!(get_digit_len(47562), 5);
    }
}
