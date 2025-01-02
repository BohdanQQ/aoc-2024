advent_of_code::solution!(2);

#[derive(Debug)]
enum SafetyStatus {
    Unsafe,
    SafeDecreasing(u32),
    SafeIncreasing(u32),
    Unininit,
    UnininitFirst(u32),
}

fn get_safety(
    seq: impl Iterator<Item = u32> + Clone,
    tolerate: bool,
    skip_idx: Option<usize>,
) -> SafetyStatus {
    let mut idx = 0;
    let mut count = 0;
    let result = seq.clone().fold(SafetyStatus::Unininit, |acc, next| {
        count += 1;
        if let Some(i) = skip_idx {
            if idx == i {
                idx += 1;
                return acc;
            }
        }
        idx += 1;
        match acc {
            // using ops::ControlFlow does not help
            SafetyStatus::Unsafe => acc,
            SafetyStatus::Unininit => SafetyStatus::UnininitFirst(next),
            SafetyStatus::UnininitFirst(last) if last < next && last.abs_diff(next) < 4 => {
                SafetyStatus::SafeIncreasing(next)
            }
            SafetyStatus::UnininitFirst(last) if last > next && last.abs_diff(next) < 4 => {
                SafetyStatus::SafeDecreasing(next)
            }
            SafetyStatus::UnininitFirst(_) => SafetyStatus::Unsafe,
            SafetyStatus::SafeDecreasing(last) if next >= last || last.abs_diff(next) >= 4 => {
                SafetyStatus::Unsafe
            }
            SafetyStatus::SafeDecreasing(_) => SafetyStatus::SafeDecreasing(next),
            SafetyStatus::SafeIncreasing(last) if next <= last || last.abs_diff(next) >= 4 => {
                SafetyStatus::Unsafe
            }
            SafetyStatus::SafeIncreasing(_) => SafetyStatus::SafeIncreasing(next),
        }
    });
    if matches!(result, SafetyStatus::Unsafe) && tolerate {
        // part 2: fuck it
        // I could add extrta state to SafetyStatus but yea, too lazy (plus in my "fold" implementation this would require
        // 2-step backtrack, making the state even bigger...)
        // sequences are very short anyway
        // or you could try skipping idx_of_failure - 1, idx_of_failure, and idx_of_failure +1
        for idx in 0..count {
            let res = get_safety(seq.clone(), false, Some(idx));
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
pub fn sequences(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32> + Clone + '_> + '_ {
    input.lines().map(|l| {
        l.split_ascii_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let correct_count = sequences(input)
        .map(|seq| get_safety(seq, false, None))
        .filter(|s| !matches!(s, SafetyStatus::Unsafe))
        .count();
    Some(correct_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let correct_count = sequences(input)
        // part 2 50us faster with seq.collect (i.e. slice, not an iterator)
        // but p1 became 100us faster => gain of 50us
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
