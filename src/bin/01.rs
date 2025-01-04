use std::collections::HashSet;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut res = (vec![], vec![]);
    // this gets another few % faster if rewritten in a for loop
    input
        .lines()
        .for_each(|line| {
            let mut it = line.split_ascii_whitespace();
            res.0.push(it.next().unwrap().parse().unwrap());
            res.1.push(it.next().unwrap().parse().unwrap());
        });
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut v1, mut v2) = parse(input);
    v1.sort();
    v2.sort();
    // keeping the zip and rewriting the rest with a loop results in a miniscule
    // but present uplift
    v1.iter()
        .zip(v2)
        .fold(0, |acc, (one, two)| acc + one.abs_diff(two))
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (v1, v2) = parse(input);
    let right_set: HashSet<u32> = std::collections::HashSet::from_iter(v2.clone());
    let mut right_map = std::collections::HashMap::new();
    v2.iter().filter(|v| right_set.contains(v)).for_each(|v| {
        right_map.entry(v).and_modify(|v| *v += 1).or_insert(1);
    });
    let mut res = 0u64;
    // curiously, writing this as a fold slows down part 2 by >10%
    // using filter better than unwrap or
    for v in v1.iter().filter(|v| right_map.contains_key(v)) {
        res += *v as u64 * *right_map.get(&v).unwrap();
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
