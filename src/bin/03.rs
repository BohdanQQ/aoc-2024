use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
    for mat in re.find_iter(input) {
        let start_idx = mat.start();
        let mut inner_res: u32 = 1;
        for g in re.captures_at(input, start_idx).unwrap().iter().skip(1) {
            inner_res *= g.unwrap().as_str().parse::<u32>().unwrap();
        }
        res += inner_res;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;
    let mut enabled = true;
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do\\(\\)|don't\\(\\)").unwrap();
    for mat in re.find_iter(input) {
        let start_idx = mat.start();
        let mut inner_res: u32 = 1;
        let unwrapped = re.captures_at(input, start_idx).unwrap();
        let iter = unwrapped.iter();
        let mut is_mul = false;
        for g in iter.take(1) {
            let s = g.unwrap().as_str();
            if s.starts_with("don\'t") {
                enabled = false;
            } else if s.starts_with("do") {
                enabled = true;
            } else {
                is_mul = true;
            }
        }
        if !is_mul || !enabled {
            continue;
        }
        for g in re.captures_at(input, start_idx).unwrap().iter().skip(1) {
            inner_res *= g.unwrap().as_str().parse::<u32>().unwrap();
        }
        res += inner_res;
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
