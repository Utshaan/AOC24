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
            let mut nums: Vec<u32> = l
                .split_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect();

            let nums2 = nums.clone();
            let asc = nums.last() >= nums.first();
            let mut last = nums.pop().unwrap();
            let mut line = 1;
            let mut line2 = 1;
            while let Some(this) = nums.pop() {
                if (asc == (this <= last))
                    && (1 <= this.abs_diff(last))
                    && (this.abs_diff(last) <= 3)
                {
                    last = this;
                } else {
                    line = 0;
                    if last == *nums2.last().unwrap() || last == *nums2.first().unwrap() {
                        last = this;
                    }
                    while let Some(this) = nums.pop() {
                        if (asc == (this <= last))
                            && (1 <= this.abs_diff(last))
                            && (this.abs_diff(last) <= 3)
                        {
                            last = this;
                        } else {
                            line2 = 0;
                            break;
                        }
                    }
                    break;
                }
            }

            println!("{} | {} | {:?}", line, line2, nums2);

            (line, line2)
        })
        .fold((0, 0), |ans, x| (ans.0 + x.0, ans.1 + x.1));

    println!("{:?}", ans);
}
