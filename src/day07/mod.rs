pub fn solve(input: String) -> (u64, u64) {
    let mut ans = (0, 0);

    input.lines().for_each(|line| {
        let line: Vec<&str> = line.split(':').collect();
        let a = line[0].parse::<u64>().expect("KYS");
        let mut v: Vec<u64> = line[1]
            .trim()
            .split_whitespace()
            .map(|i| i.parse::<u64>().expect("KYS"))
            .collect();
        let f = v.remove(0);
        if part_1(f, v.clone(), a) {
            ans.0 += a;
        }
        if part_2(f, v, a) {
            ans.1 += a;
        }
    });

    ans
}

fn part_1(res: u64, mut v: Vec<u64>, ans: u64) -> bool {
    if v.len() == 0 {
        return res == ans;
    }
    let temp = v.remove(0);

    return part_1(res + temp, v.clone(), ans) | part_1(res * temp, v.clone(), ans);
}

fn part_2(res: u64, mut v: Vec<u64>, ans: u64) -> bool {
    if v.len() == 0 {
        return res == ans;
    }
    let temp = v.remove(0);

    return part_2(res + temp, v.clone(), ans)
        | part_2(res * temp, v.clone(), ans)
        | part_2(
            format!("{res}{temp}").parse::<u64>().expect("Too big man"),
            v.clone(),
            ans,
        );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 07");
        assert_eq!((3749, 11387), solve(input));
    }
}
