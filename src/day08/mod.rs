use itertools::{self, Itertools};
use std::collections::{HashMap, HashSet};

pub fn solve(input: String) -> (u64, u64) {
    let mut ans = (0, 0);

    let mut ant_hash: HashMap<char, Vec<(u8, u8)>> = HashMap::new();

    let ant_map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '.' {
                        return c;
                    }
                    ant_hash
                        .entry(c)
                        .and_modify(|v| {
                            v.push((i as u8, j as u8));
                        })
                        .or_insert(vec![(i as u8, j as u8)]);
                    c
                })
                .collect()
        })
        .collect();

    let rows = ant_map.len() as i8;
    let cols = ant_map[0].len() as i8;
    ans.0 = part_1(&ant_hash, rows, cols);
    ans.1 = part_2(&ant_hash, rows, cols);

    ans
}

fn part_1(hash: &HashMap<char, Vec<(u8, u8)>>, rows: i8, cols: i8) -> u64 {
    let mut s = HashSet::new();
    hash.values().for_each(|v| {
        v.iter().combinations(2).for_each(|p| {
            let p1 = (
                (2 * p[0].0) as i8 - (p[1].0) as i8,
                (2 * p[0].1) as i8 - (p[1].1) as i8,
            );
            let p2 = (
                (2 * p[1].0) as i8 - (p[0].0) as i8,
                (2 * p[1].1) as i8 - (p[0].1) as i8,
            );
            if in_map(p1, rows, cols) {
                s.insert(p1);
            }
            if in_map(p2, rows, cols) {
                s.insert(p2);
            }
        });
    });

    s.len() as u64
}

fn part_2(hash: &HashMap<char, Vec<(u8, u8)>>, rows: i8, cols: i8) -> u64 {
    let mut s = HashSet::new();
    hash.values().for_each(|v| {
        v.iter().combinations(2).for_each(|p| {
            let d = (
                (p[1].0) as i8 - (p[0].0) as i8,
                (p[1].1) as i8 - (p[0].1) as i8,
            );

            let mut p1 = (p[0].0 as i8, p[0].1 as i8);
            let mut p2 = (p[1].0 as i8, p[1].1 as i8);

            while in_map(p1, rows, cols) {
                s.insert(p1);
                p1 = (p1.0 - d.0, p1.1 - d.1);
            }

            while in_map(p2, rows, cols) {
                s.insert(p2);
                p2 = (p2.0 + d.0, p2.1 + d.1);
            }
        });
    });

    s.len() as u64
}

fn in_map(p: (i8, i8), rows: i8, cols: i8) -> bool {
    (p.0 >= 0) & (p.0 <= rows - 1) & (p.1 >= 0) & (p.1 <= cols - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 08");
        assert_eq!((14, 34), solve(input));
    }
}
