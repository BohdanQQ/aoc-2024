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
        println!("{} {:?}", num, secret);
        res += secret;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
