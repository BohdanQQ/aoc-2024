use rayon::iter::{ParallelBridge, ParallelIterator};

advent_of_code::solution!(22);

fn mix(sec: u64, n: u64) -> u64 {
    sec ^ n
}

fn prune(sec: u64) -> u64 {
    sec % 16777216
}

fn mix_prune(sec: u64, n: u64) -> u64 {
    prune(mix(sec, n))
}

fn next_secret(n: u64) -> u64 {
    let mut sec = n;
    let x = sec * 64;
    sec = mix_prune(sec, x);
    let x = sec / 32;
    sec = mix_prune(sec, x);
    let x = sec * 2048;
    sec = mix_prune(sec, x);
    sec
}

pub fn part_one(input: &str) -> Option<u64> {
    let nums = input
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap());
    let mut res = 0u64;
    for num in nums {
        let iters = 2000;
        let mut secret = num;
        for _ in 0..iters {
            secret = next_secret(secret);
        }
        res += secret;
    }
    Some(res)
}

fn find_first(window: &[(i8, i8)], stack: &[(i8, i8)]) -> i8 {
    for i in 0..stack.len() - window.len() {
        for j in 0..window.len() {
            if stack[i + j].0 != window[j].0 {
                break;
            } else if j == window.len() - 1 {
                return stack[i + j].1;
            }
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut last_digit_seq = vec![];
    let nums = input
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap());
    for num in nums {
        let mut digit_seq = vec![];
        let iters = 2000;
        let mut secret = num;
        for _ in 0..iters {
            digit_seq.push((secret % 10) as i8);
            secret = next_secret(secret);
        }
        last_digit_seq.push(digit_seq);
    }

    let diffs = last_digit_seq
        .iter()
        .map(|v| {
            v.windows(2)
                .map(|window| (window[1] - window[0], window[1]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let first = diffs[0].clone();

    let best = first
        .windows(4)
        .par_bridge()
        .map(|diff_window| {
            let mut current = diff_window[diff_window.len() - 1].1 as u64;
            for trader_list in diffs.iter().skip(1) {
                let res = find_first(diff_window, trader_list) as u64;
                current += res;
            }
            current
        })
        .max()
        .unwrap();
    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
