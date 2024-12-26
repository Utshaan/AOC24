use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn change(&mut self) {
        *self = match *self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

pub fn solve(input: String) -> (u64, u64) {
    let mut ans = (0, 0);
    let mut gpos = (0, 0);

    let mut map: Vec<Vec<u16>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => {
                        gpos = (i, j);
                        2
                    }
                })
                .collect()
        })
        .collect();

    ans.0 = part_1(gpos, &mut map);
    ans.1 = part_2(gpos, map);

    ans
}

fn part_1(mut gpos: (usize, usize), map: &mut Vec<Vec<u16>>) -> u64 {
    let mut dir = Dir::Up;
    let mut ans = 1;
    let rows = map.len();
    let cols = map[0].len();

    while !is_edge(gpos.0, gpos.1, rows, cols) {
        match dir {
            Dir::Up => (0..gpos.0)
                .rev()
                .take_while(|i| map[*i][gpos.1] != 1)
                .collect::<Vec<usize>>()
                .iter()
                .for_each(|i| {
                    if map[*i][gpos.1] != 2 {
                        map[*i][gpos.1] = 2;
                        ans += 1;
                    }
                    gpos.0 = *i;
                }),
            Dir::Right => ((gpos.1 + 1)..cols)
                .take_while(|j| map[gpos.0][*j] != 1)
                .collect::<Vec<usize>>()
                .iter()
                .for_each(|i| {
                    if map[gpos.0][*i] != 2 {
                        map[gpos.0][*i] = 2;
                        ans += 1;
                    }
                    gpos.1 = *i;
                }),
            Dir::Down => ((gpos.0 + 1)..rows)
                .take_while(|i| map[*i][gpos.1] != 1)
                .collect::<Vec<usize>>()
                .iter()
                .for_each(|i| {
                    if map[*i][gpos.1] != 2 {
                        map[*i][gpos.1] = 2;
                        ans += 1;
                    }
                    gpos.0 = *i;
                }),
            Dir::Left => (0..gpos.1)
                .rev()
                .take_while(|j| map[gpos.0][*j] != 1)
                .collect::<Vec<usize>>()
                .iter()
                .for_each(|i| {
                    if map[gpos.0][*i] != 2 {
                        map[gpos.0][*i] = 2;
                        ans += 1;
                    }
                    gpos.1 = *i;
                }),
        }

        dir.change();
    }
    ans
}

fn part_2(gpos: (usize, usize), map: Vec<Vec<u16>>) -> u64 {
    let mut ans = 0;

    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if (i, j) != gpos && *c == 2 {
                if is_loop(gpos, &map, (i, j)) {
                    ans += 1;
                }
            }
        })
    });

    ans
}

fn is_loop(mut gpos: (usize, usize), map: &Vec<Vec<u16>>, opos: (usize, usize)) -> bool {
    let mut map = map.clone();
    let mut dir = Dir::Up;
    map[opos.0][opos.1] = 1;
    let rows = map.len();
    let cols = map[0].len();

    let mut visited = HashSet::new();

    while !is_edge(gpos.0, gpos.1, rows, cols) {
        match dir {
            Dir::Up => {
                gpos.0 = *(0..gpos.0)
                    .rev()
                    .take_while(|i| map[*i][gpos.1] != 1)
                    .collect::<Vec<usize>>()
                    .iter()
                    .last()
                    .unwrap_or(&gpos.0);
            }
            Dir::Right => {
                gpos.1 = *((gpos.1 + 1)..cols)
                    .take_while(|j| map[gpos.0][*j] != 1)
                    .collect::<Vec<usize>>()
                    .iter()
                    .last()
                    .unwrap_or(&gpos.1)
            }
            Dir::Down => {
                gpos.0 = *((gpos.0 + 1)..rows)
                    .take_while(|i| map[*i][gpos.1] != 1)
                    .collect::<Vec<usize>>()
                    .iter()
                    .last()
                    .unwrap_or(&gpos.0)
            }
            Dir::Left => {
                gpos.1 = *(0..gpos.1)
                    .rev()
                    .take_while(|j| map[gpos.0][*j] != 1)
                    .collect::<Vec<usize>>()
                    .iter()
                    .last()
                    .unwrap_or(&gpos.1)
            }
        }

        dir.change();
        if !visited.insert((gpos, dir.clone())) {
            return true;
        }
    }
    false
}

fn is_edge(x: usize, y: usize, rows: usize, cols: usize) -> bool {
    (x == 0) | (x == rows - 1) | (y == 0) | (y == cols - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 06");
        assert_eq!((41, 6), solve(input));
    }
}
