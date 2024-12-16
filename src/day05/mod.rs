use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve(input: String) -> (u32, u32) {
    let (rules, orders) = match input.find("\n\n") {
        Some(i) => input.split_at(i),
        None => ("", ""),
    };

    let rreg = Regex::new(r"(?<n1>[0-9]+)\|(?<n2>[0-9]+)").unwrap();
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

    rules.lines().for_each(|x| {
        rreg.captures_iter(x).for_each(|caps| {
            let (_, [n1, n2]) = caps.extract();
            let n1 = n1.parse::<u32>().unwrap();
            let n2 = n2.parse::<u32>().unwrap();
            map.entry(n1)
                .and_modify(|s| {
                    (*s).insert(n2);
                })
                .or_insert(HashSet::from([n2]));
        });
    });

    let temp_map = map.clone();
    temp_map.keys().for_each(|key| {
        map.entry(*key).and_modify(|s| {
            temp_map
                .get(&key)
                .unwrap_or(&HashSet::new())
                .iter()
                .for_each(|i| {
                    (*s).insert(*i);
                });
        });
    });

    let oreg = Regex::new(r"(?<n1>[0-9]+)").unwrap();
    let ans = orders
        .lines()
        .skip(2)
        .filter_map(|x| {
            let order = oreg
                .captures_iter(x)
                .map(|caps| {
                    let (_, [n1]) = caps.extract();
                    let n1 = n1.parse::<u32>().unwrap();
                    n1
                })
                .collect::<Vec<u32>>();
            let mut temp = order.clone();
            let mut temp_2 = order.clone();
            let a_0 = order
                .iter()
                .all(|n| {
                    let mut set = HashSet::new();
                    order.iter().skip_while(|a| *a != n).skip(1).for_each(|a| {
                        set.insert(*a);
                    });
                    map.get(n).unwrap_or(&HashSet::new()).is_superset(&set)
                })
                .then_some(temp.remove(order.len() / 2))
                .unwrap_or(0);

            let a_1 = order
                .iter()
                .any(|n| {
                    let mut set = HashSet::new();
                    order.iter().skip_while(|a| *a != n).skip(1).for_each(|a| {
                        set.insert(*a);
                    });
                    !map.get(n).unwrap_or(&HashSet::new()).is_superset(&set)
                })
                .then_some({
                    temp_2.sort_by(|a, b| {
                        match map.get(a).unwrap_or(&HashSet::new()).contains(b) {
                            true => std::cmp::Ordering::Less,
                            false => std::cmp::Ordering::Greater,
                        }
                    });
                    temp_2.remove(order.len() / 2)
                })
                .unwrap_or(0);
            Some((a_0, a_1))
        })
        .fold((0, 0), |acc, temp| (acc.0 + temp.0, acc.1 + temp.1));
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_example_input;

    #[test]
    fn example() {
        let input = get_example_input("DAY 05");
        assert_eq!((143, 123), solve(input));
    }
}
