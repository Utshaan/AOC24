#[derive(Clone, PartialEq)]
struct Block {
    v: Option<u32>,
    size: u32,
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = match self.v {
            Some(d) => d.to_string(),
            None => '.'.to_string(),
        };

        c = c.repeat(self.size as usize);
        write!(f, "{c}")
    }
}

impl Block {
    fn new(v: Option<u32>, size: u32) -> Self {
        Block { v, size }
    }
}

pub fn solve(input: String) -> (u64, u64) {
    let mut ans = (0, 0);

    let mut q = Vec::new();

    let mut id = 0;
    let mut f = true;
    let mut spaces = 0;

    input
        .strip_suffix("\n")
        .unwrap_or(&input)
        .chars()
        .for_each(|c| {
            let d = c.to_digit(10).expect("Non digit in input\n");
            if f {
                (0..d).for_each(|_| {
                    q.push(id);
                });
                id += 1;
            } else {
                (0..d).for_each(|_| q.push(u32::MAX));
                spaces += d;
            }
            f = !f;
        });

    ans.0 = part_1(q.clone(), spaces);
    ans.1 = part_2(q);

    ans
}

fn part_1(mut q: Vec<u32>, spaces: u32) -> u64 {
    (0..spaces).for_each(|_| {
        let t = q.pop().expect("Ran out?");
        match q.iter().enumerate().filter(|(_, &c)| c == u32::MAX).next() {
            Some((i, _)) => {
                q[i] = t;
            }
            None => {}
        }
    });
    q.iter()
        .enumerate()
        .fold(0, |acc, (i, &id)| acc + id as u64 * i as u64)
}

fn part_2(mut q: Vec<u32>) -> u64 {
    let mut qb: Vec<Block> = Vec::new();

    let mut last = q.pop().expect("Empty?");
    while q.len() != 0 {
        let mut size = 1;
        while q.last().is_some_and(|d| *d == last) {
            q.pop();
            size += 1;
        }
        match last {
            u32::MAX => {
                qb.insert(0, Block::new(None, size));
            }
            n => {
                qb.insert(0, Block::new(Some(n), size));
            }
        }
        match q.pop() {
            Some(d) => {
                last = d;
            }
            None => {}
        }
    }

    let mut temp = qb.len() - 1;
    while temp != 0 {
        match (0..temp)
            .filter(|i| qb[*i].size >= qb[temp].size && qb[*i].v.is_none())
            .next()
        {
            Some(i) => {
                let work = qb.remove(temp);
                qb.insert(temp, Block::new(None, work.size));
                match qb[i].size - work.size {
                    0 => {
                        qb[i] = work.clone();
                    }
                    l => {
                        qb[i].size = l;
                        qb.insert(i, work.clone());
                        temp += 1;
                    }
                }
            }
            None => {}
        }
        temp -= 1;
        while qb[temp].v.is_none() {
            temp -= 1;
        }
    }

    let qs = qb.iter().fold(vec![], |mut acc, cur| match cur.v {
        Some(n) => {
            (0..cur.size).for_each(|_| {
                acc.push(n);
            });
            acc
        }
        None => {
            (0..cur.size).for_each(|_| {
                acc.push(0);
            });
            acc
        }
    });

    qs.iter()
        .enumerate()
        .fold(0, |acc, (i, &id)| acc + id as u64 * i as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 09");
        assert_eq!((1928, 2858), solve(input));
    }
}
