use std::char;

pub fn solve(input: String) -> (u64, u64) {
    let mut x_positions = Vec::new();
    let mut a_positions = Vec::new();
    let mut ans = (0, 0);

    let ws: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| {
                    if ch == 'X' {
                        x_positions.push((i as i16, j as i16));
                    } else if ch == 'A' {
                        a_positions.push((i as i16, j as i16));
                    }
                    ch
                })
                .collect()
        })
        .collect();

    let rows = ws.len();
    let cols = ws[0].len();

    x_positions.iter().for_each(|&(x, y)| {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .for_each(|&(dx, dy)| {
            let m_pos = (x + dx, y + dy);
            let a_pos = (x + 2 * dx, y + 2 * dy);
            let s_pos = (x + 3 * dx, y + 3 * dy);

            // Check if positions are within bounds
            if in_matrix(m_pos.0, m_pos.1, rows, cols)
                && in_matrix(a_pos.0, a_pos.1, rows, cols)
                && in_matrix(s_pos.0, s_pos.1, rows, cols)
            {
                // Check if characters match
                if ws[m_pos.0 as usize][m_pos.1 as usize] == 'M'
                    && ws[a_pos.0 as usize][a_pos.1 as usize] == 'A'
                    && ws[s_pos.0 as usize][s_pos.1 as usize] == 'S'
                {
                    ans.0 += 1;
                }
            }
        })
    });

    a_positions
        .iter()
        .filter(|&(x, y)| *x != 0 && *x != rows as i16 - 1 && *y != 0 && *y != cols as i16 - 1)
        .for_each(|&(x, y)| {
            let tl_pos = (x - 1, y - 1);
            let tr_pos = (x - 1, y + 1);
            let bl_pos = (x + 1, y - 1);
            let br_pos = (x + 1, y + 1);
            if ws[tl_pos.0 as usize][tl_pos.1 as usize] == 'M' {
                if ws[tr_pos.0 as usize][tr_pos.1 as usize] == 'M' {
                    if ws[bl_pos.0 as usize][bl_pos.1 as usize] == 'S' {
                        if ws[br_pos.0 as usize][br_pos.1 as usize] == 'S' {
                            ans.1 += 1;
                        }
                    }
                } else if ws[bl_pos.0 as usize][bl_pos.1 as usize] == 'M' {
                    if ws[tr_pos.0 as usize][tr_pos.1 as usize] == 'S' {
                        if ws[br_pos.0 as usize][br_pos.1 as usize] == 'S' {
                            ans.1 += 1;
                        }
                    }
                }
            } else if ws[br_pos.0 as usize][br_pos.1 as usize] == 'M' {
                if ws[tr_pos.0 as usize][tr_pos.1 as usize] == 'M' {
                    if ws[bl_pos.0 as usize][bl_pos.1 as usize] == 'S' {
                        if ws[tl_pos.0 as usize][tl_pos.1 as usize] == 'S' {
                            ans.1 += 1;
                        }
                    }
                } else if ws[bl_pos.0 as usize][bl_pos.1 as usize] == 'M' {
                    if ws[tr_pos.0 as usize][tr_pos.1 as usize] == 'S' {
                        if ws[tl_pos.0 as usize][tl_pos.1 as usize] == 'S' {
                            ans.1 += 1;
                        }
                    }
                }
            }
        });

    ans
}

fn in_matrix(x: i16, y: i16, rows: usize, cols: usize) -> bool {
    x >= 0 && x < rows as i16 && y >= 0 && y < cols as i16
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 04");
        assert_eq!((18, 9), solve(input));
    }
}
