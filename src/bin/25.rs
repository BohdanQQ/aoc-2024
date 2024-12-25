advent_of_code::solution!(25);

type KeyVals = [u8; 5];
type PinVals = KeyVals;

fn parse_keys_pins(input: &str) -> (Vec<KeyVals>, Vec<PinVals>) {
    let mut keys = vec![];
    let mut pins = vec![];

    let mut value = [0; 5];

    let mut is_key = None;

    let reset = |value: &mut [u8; 5], is_key: &mut Option<bool>| {
        for v in value.iter_mut() {
            *v = 0;
        }
        *is_key = None;
    };

    for line in input.split('\n') {
        if line.is_empty() {
            if is_key.unwrap() {
                keys.push(value);
            } else {
                value.iter_mut().for_each(|v| *v -= 1);
                pins.push(value);
            }
            reset(&mut value, &mut is_key);
        } else {
            for (i, c) in line.chars().enumerate() {
                if is_key.is_none() {
                    if c == '#' {
                        is_key = Some(true);
                    } else {
                        is_key = Some(false);
                    }
                    break;
                }

                value[i] += if c == '.' { 0 } else { 1 };
            }
        }
    }

    (keys, pins)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (keys, pins) = parse_keys_pins(input);
    let mut res = 0;
    for k in &keys {
        for lock in &pins {
            if k.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5) {
                res += 1;
            }
        }
    }
    Some(res)
}

pub fn part_two(_: &str) -> Option<String> {
    Some("Stellar delivery ;'*.*:".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("Stellar delivery ;'*.*:".to_owned()));
    }
}
