advent_of_code::solution!(19);
use std::collections::HashMap;

fn possible(target: &str, parts: &Vec<&str>) -> bool {
    parts.iter().any(|part| {
        *part == target
            || (target.starts_with(part) && possible(target.strip_prefix(part).unwrap(), parts))
    })
}

fn count_possible<'a>(
    target: &'a str,
    parts: &Vec<&str>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if !cache.contains_key(target) {
        let res = parts
            .iter()
            .map(|part| {
                if target == *part {
                    1
                } else if target.starts_with(part) {
                    count_possible(target.strip_prefix(part).unwrap(), parts, cache)
                } else {
                    0
                }
            })
            .sum();
        cache.insert(target, res);
    }
    *cache.get(target).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (parts, _) = input.split_once('\n').unwrap();
    let towels = input.lines();

    let parts = parts.split(',').map(|v| v.trim()).collect::<Vec<_>>();

    let result = towels
        .map(|t| if possible(t, &parts) { 1 } else { 0 })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (parts, _) = input.split_once('\n').unwrap();
    let towels = input.lines();

    let parts = parts.split(',').map(|v| v.trim()).collect::<Vec<_>>();
    let mut cache: HashMap<&str, u64> = HashMap::new();
    let result = towels
        // .filter(|t| possible(t, &parts)) - this makes it slower (31 -> 39ms) - there are not many impossible ones (or they are at least detected early)
        .map(|t| count_possible(t, &parts, &mut cache))
        .sum();
    // ofc you could also solve p1 using this, just insert if cnt > 0 { 1 } else { 0 }
    // into the map closure - this could shave off ~5ms (runtime of p1)
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
