use std::collections::{BinaryHeap, HashMap};
use std::fs;

use crate::print_day;

pub fn solve() {
    print_day("DAY 01");
    let content = fs::read_to_string("src/day01/input.txt").expect(
        "Issue in reading input.txt. Make sure the file exists and the permissions are right",
    );

    let mut left: BinaryHeap<u32> = BinaryHeap::new();
    let mut right: BinaryHeap<u32> = BinaryHeap::new();
    let mut map: HashMap<u32, u32> = HashMap::new();

    content.lines().into_iter().for_each(|x| {
        let pair: Vec<u32> = x
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        left.push(pair[0]);
        right.push(pair[1]);

        map.entry(pair[1])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let mut diff_score = 0;
    let mut sim_score = 0;

    while let (Some(left_val), Some(right_val)) = (left.pop(), right.pop()) {
        diff_score += left_val.abs_diff(right_val);
        sim_score += left_val * map.get(&left_val).unwrap_or(&0);
    }

    println!("Part A : {}", diff_score);
    println!("Part B : {}", sim_score);
}
