use std::collections::{HashSet, VecDeque};

pub fn solve(input: String) -> (u64, u64) {
    let mut starts = Vec::new();

    let map: Vec<Vec<u32>> = input
        .lines()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '0' {
                        starts.push((i, j));
                    }
                    c.to_digit(10).expect("Not 0?")
                })
                .collect()
        })
        .collect();

    let rows = map.len();
    let cols = map[0].len();

    let mut visited = VecDeque::new();

    let ans: (u64, u64) = starts
        .iter()
        .map(|(si, sj)| {
            let mut fin = HashSet::new();
            let mut count: u32 = 0;
            let cur_pos = (*si, *sj).clone();
            get_n(&cur_pos, rows as usize, cols as usize)
                .into_iter()
                .for_each(|p| {
                    visited.push_back((p.0, p.1, 0));
                });
            while let Some(pos) = visited.pop_front() {
                if map[pos.0][pos.1] > pos.2 && map[pos.0][pos.1] - pos.2 == 1 {
                    if map[pos.0][pos.1] == 9 {
                        fin.insert((pos.0, pos.1));
                        count += 1;
                        continue;
                    }
                    get_n(&(pos.0, pos.1), rows as usize, cols as usize)
                        .into_iter()
                        .for_each(|p| {
                            visited.push_back((p.0, p.1, map[pos.0][pos.1]));
                        });
                }
            }
            (fin.len(), count)
        })
        .fold((0, 0), |mut tot, curcounts| {
            tot.0 += curcounts.0 as u64;
            tot.1 += curcounts.1 as u64;
            tot
        });
    ans
}

fn get_n(pos: &(usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    if pos.0 == 0 {
        if pos.1 == 0 {
            vec![(1, 0), (0, 1)]
        } else if pos.1 == cols - 1 {
            vec![(1, cols - 1), (0, cols - 2)]
        } else {
            vec![(0, pos.1 - 1), (0, pos.1 + 1), (1, pos.1)]
        }
    } else if pos.0 == rows - 1 {
        if pos.1 == 0 {
            vec![(rows - 2, 0), (rows - 1, 1)]
        } else if pos.1 == cols - 1 {
            vec![(rows - 2, cols - 1), (rows - 1, cols - 2)]
        } else {
            vec![
                (rows - 1, pos.1 - 1),
                (rows - 1, pos.1 + 1),
                (rows - 2, pos.1),
            ]
        }
    } else {
        if pos.1 == 0 {
            vec![(pos.0 - 1, 0), (pos.0, 1), (pos.0 + 1, 0)]
        } else if pos.1 == cols - 1 {
            vec![
                (pos.0 - 1, cols - 1),
                (pos.0, cols - 2),
                (pos.0 + 1, cols - 1),
            ]
        } else {
            vec![
                (pos.0 - 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
                (pos.0 + 1, pos.1),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 10");
        assert_eq!((36, 81), solve(input));
    }
}
