use std::collections::HashMap;

advent_of_code::solution!(11);

fn step(vals: &HashMap<u64, u64>, max: u64) -> u64 {
    let mut map = vals.clone();
    for _ in 0..max {
        let mut new_map: HashMap<u64, u64> = HashMap::with_capacity(map.len());
        for (k, v) in &map {
            if *k == 0 {
                let ones_n = new_map.get(&1).unwrap_or(&0);
                new_map.insert(1, *ones_n + v);
                continue;
            }

            let s = k.to_string();
            if s.len() % 2 == 0 {
                let n = s[0..(s.len() / 2)].parse().unwrap();
                new_map.insert(n, (new_map.get(&n).unwrap_or(&0)) + (v));
                let n = s[(s.len() / 2)..].parse().unwrap();
                new_map.insert(n, (new_map.get(&n).unwrap_or(&0)) + (v));
            } else {
                let n = k * 2024;
                new_map.insert(n, (new_map.get(&n).unwrap_or(&0)) + (v));
            }
        }
        map = new_map.clone();
    }

    map.values().sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let nums = input
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut map = HashMap::new();
    for n in nums {
        if let Some(x) = map.get(&n) {
            map.insert(n, x + 1);
        } else {
            map.insert(n, 1);
        }
    }
    Some(step(&map, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = input
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut map = HashMap::new();
    for n in nums {
        if let Some(x) = map.get(&n) {
            map.insert(n, x + 1);
        } else {
            map.insert(n, 1);
        }
    }
    Some(step(&map, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
