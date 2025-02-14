use std::collections::{BinaryHeap, HashMap};

pub fn solve(input: String) -> (u64, u64) {
    let mut left: BinaryHeap<u64> = BinaryHeap::new();
    let mut right: BinaryHeap<u64> = BinaryHeap::new();
    let mut map: HashMap<u64, u64> = HashMap::new();

    input.lines().into_iter().for_each(|x| {
        let pair: Vec<u64> = x
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
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

    (diff_score, sim_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 01");
        assert_eq!((11, 31), solve(input));
    }
}
