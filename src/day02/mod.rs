use std::fs;

use crate::print_day;

pub fn solve() {
    print_day("DAY 02");
    let content = fs::read_to_string("src/day02/input.txt").expect(
        "Issue in reading input.txt. Make sure the file exists and the permissions are right",
    );

    let ans = content
        .lines()
        .map(|l| {
            let nums: Vec<i32> = l
                .split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect();

            let part_a = is_safe(&nums);
            let mut part_b = part_a;
            if part_a != 1 {
                part_b = match (0..nums.len())
                    .map(|i| {
                        let mut copy_nums = nums.clone();
                        copy_nums.remove(i);
                        is_safe(&copy_nums)
                    })
                    .any(|res| res == 1)
                {
                    true => 1,
                    false => 0,
                }
            }

            (part_a, part_b)
        })
        .fold((0, 0), |ans, x| (ans.0 + x.0, ans.1 + x.1));

    println!("Part A : {}\nPart B : {}", ans.0, ans.1);
}

fn is_safe(nums: &Vec<i32>) -> u32 {
    let mut ret = 1;
    let mut first = true;
    let mut last = (nums[1] - nums[0]).signum();
    nums.windows(2).for_each(|pair| {
        let temp = pair[1] - pair[0];
        let temp_sign = temp.signum();

        if temp.abs() >= 1 && temp.abs() <= 3 {
            if temp_sign != last {
                first = false;
                ret = 0;
            }
            last = temp_sign;
        } else {
            first = false;
            ret = 0;
        }
    });
    return ret;
}
