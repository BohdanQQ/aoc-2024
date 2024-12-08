use std::collections::{HashMap, HashSet};

use advent_of_code::parse_field;

advent_of_code::solution!(8);

pub fn get_antinodes(
    one: (usize, usize),
    two: (usize, usize),
    size: usize,
    iters: usize,
) -> Vec<(usize, usize)> {
    let (r1, c1) = (one.0 as i32, one.1 as i32);
    let (r2, c2) = (two.0 as i32, two.1 as i32);

    let (dr, dc) = (r2 - r1, c2 - c1);

    let mut res = vec![];
    for mul in 1..iters + 1 {
        let candidate1 = (r1 - dr * mul as i32, c1 - dc * mul as i32);
        let candidate2 = (r2 + dr * mul as i32, c2 + dc * mul as i32);
        for cand in [candidate1, candidate2] {
            if cand.0 >= 0 && cand.1 >= 0 && cand.0 < size as i32 && cand.1 < size as i32 {
                res.push((cand.0 as usize, cand.1 as usize));
            }
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut antena_map = HashMap::<char, Vec<(usize, usize)>>::new();
    let size = parse_field(input, |c, position| {
        if c.is_alphanumeric() {
            let entry = antena_map.entry(c).or_default();
            entry.push(position);
        }
        c
    })
    .len();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for v in antena_map.values() {
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                for an in get_antinodes(v[i], v[j], size, 1) {
                    antinodes.insert(an);
                }
            }
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut antena_map = HashMap::<char, Vec<(usize, usize)>>::new();
    let size = parse_field(input, |c, position| {
        if c.is_alphanumeric() {
            let entry = antena_map.entry(c).or_default();
            entry.push(position);
        }
        c
    })
    .len();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for v in antena_map.values() {
        for i in 0..v.len() {
            antinodes.insert(v[i]);
            for j in i + 1..v.len() {
                for an in get_antinodes(v[i], v[j], size, 100) {
                    antinodes.insert(an);
                }
            }
        }
    }

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
