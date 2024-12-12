use std::{
    sync::{atomic::AtomicU64, Arc},
    thread,
};

advent_of_code::solution!(7);

pub fn try_num(tgt: u64, nums: &[u64], now: u64) -> bool {
    if now > tgt {
        return false;
    }
    if let Some((oldnums, newnums)) = nums.split_at_checked(1) {
        if now == 0 {
            try_num(tgt, newnums, now + oldnums[0])
        } else {
            try_num(tgt, newnums, now * oldnums[0]) || try_num(tgt, newnums, now + oldnums[0])
        }
    } else {
        now == tgt
    }
}

pub fn try_num2(tgt: u64, nums: &[u64], now: u64) -> bool {
    if now > tgt {
        return false;
    }
    if let Some((oldnums, newnums)) = nums.split_at_checked(1) {
        let s = (now.to_string() + &oldnums[0].to_string())
            .parse::<u64>()
            .unwrap();
        if now == 0 {
            try_num2(tgt, newnums, now + oldnums[0]) || try_num2(tgt, newnums, s)
        } else {
            try_num2(tgt, newnums, now * oldnums[0])
                || try_num2(tgt, newnums, now + oldnums[0])
                || try_num2(tgt, newnums, s)
        }
    } else {
        tgt == now
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let specs = input.split('\n').map(|line| {
        line.split_ascii_whitespace()
            .map(|v| v.replace(':', "").parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut res = 0;
    for mut vec in specs {
        let nums = vec.split_off(1);
        if try_num(vec[0], &nums, 0) {
            res += vec[0];
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let specs = input.split('\n').map(|line| {
        line.split_ascii_whitespace()
            .map(|v| v.replace(':', "").parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });

    let res = Arc::new(AtomicU64::new(0));
    thread::scope(|c| {
        for mut vec in specs {
            let vc = vec.clone();
            let rc = res.clone();
            c.spawn(move || {
                let nums = vec.split_off(1);
                if try_num2(vc[0], &nums, 0) {
                    rc.fetch_add(vc[0], std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
    });

    Some(res.load(std::sync::atomic::Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}