advent_of_code::solution!(2);

#[derive(Debug)]
enum SafetyStatus {
    Unsafe,
    SafeDecreasing(u32),
    SafeIncreasing(u32),
    Unininit,
    UnininitFirst(u32),
}

fn get_safety(seq: &Vec<u32>, tolerate: bool, skip_idx: Option<usize>) -> SafetyStatus {
    let mut idx = 0;
    let result = seq.iter().fold(SafetyStatus::Unininit, |acc, next| {
        if let Some(i) = skip_idx {
            if idx == i {
                idx += 1;
                return acc;
            }
        }
        idx += 1;
        match acc {
            SafetyStatus::Unsafe => acc,
            SafetyStatus::Unininit => SafetyStatus::UnininitFirst(*next),
            SafetyStatus::UnininitFirst(last) if last < *next && last.abs_diff(*next) < 4 => {
                SafetyStatus::SafeIncreasing(*next)
            }
            SafetyStatus::UnininitFirst(last) if last > *next && last.abs_diff(*next) < 4 => {
                SafetyStatus::SafeDecreasing(*next)
            }
            SafetyStatus::UnininitFirst(_) => SafetyStatus::Unsafe,
            SafetyStatus::SafeDecreasing(last) if *next >= last || last.abs_diff(*next) >= 4 => {
                SafetyStatus::Unsafe
            }
            SafetyStatus::SafeDecreasing(_) => SafetyStatus::SafeDecreasing(*next),
            SafetyStatus::SafeIncreasing(last) if *next <= last || last.abs_diff(*next) >= 4 => {
                SafetyStatus::Unsafe
            }
            SafetyStatus::SafeIncreasing(_) => SafetyStatus::SafeIncreasing(*next),
        }
    });
    if matches!(result, SafetyStatus::Unsafe) && tolerate {
        // part 2: fuck it
        // I could add extrta state to SafetyStatus but yea, too lazy (plus in my "fold" implementation this would require
        // 2-step backtrack, making the state even bigger...)
        // sequences are very short anyway
        // or you could try skipping idx_of_failure - 1, idx_of_failure, and idx_of_failure +1
        for idx in 0..seq.len() {
            let res = get_safety(seq, false, Some(idx));
            if matches!(res, SafetyStatus::SafeDecreasing(_))
                || matches!(res, SafetyStatus::SafeIncreasing(_))
            {
                return res;
            }
        }
    }
    result
}

// could be like 10% faster if I was able to write the iterator return time
// (avoiding the final collect call)
pub fn sequences(input: &str) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let correct_count = sequences(input)
        .iter()
        .map(|seq| get_safety(seq, false, None))
        .filter(|s| !matches!(s, SafetyStatus::Unsafe))
        .count();
    Some(correct_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let correct_count = sequences(input)
        .iter()
        .map(|seq| get_safety(seq, true, None))
        .filter(|s| !matches!(s, SafetyStatus::Unsafe))
        .count();
    Some(correct_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
