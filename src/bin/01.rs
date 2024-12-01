use std::collections::HashSet;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut res = (vec![], vec![]);
    input.trim().split('\n').map(|v| {
        v.split_ascii_whitespace().collect::<Vec<_>>()
    })
    .map(|a| {
        (a[0].parse().unwrap(), a[1].parse().unwrap())
    })
    .for_each(|(o, t)| { res.0.push(o); res.1.push(t); } );
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut v1, mut v2) = parse(input);
    v1.sort();
    v2.sort();
    v1.iter().zip(v2).fold(0, |acc, (one, two)| acc + one.abs_diff(two)).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (v1, v2) = parse(input);
    let right_set: HashSet<u32> = std::collections::HashSet::from_iter(v2.clone());
    let mut right_map = std::collections::HashMap::new();
    v2.iter().for_each(|v| {
        if !right_map.contains_key(v) {
            right_map.insert(v, 0);
        }
        if right_set.contains(v) {
            right_map.insert(v, right_map.get(v).unwrap() + 1);
        }
    });
    let mut res = 0u64;
    for v in v1.iter() {
        res += *v as u64 * *right_map.get(&v).unwrap_or(&0u64) as u64;
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
