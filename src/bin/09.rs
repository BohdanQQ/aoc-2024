use advent_of_code::parse_field;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut num = -1;
    let mut free_space = 0;
    let mut parsed: Vec<i64> = vec![];
    let _ = parse_field(input, |c, (_, col)| {
        let n = c.to_string().parse::<u32>().unwrap();
        let v = if col % 2 == 1 {
            free_space += n;
            -1
        } else {
            num += 1;
            num
        };
        for _ in 0..n {
            parsed.push(v);
        }
        n
    });
    let mut idx_left = 0;
    let mut idx_right = parsed.len() - 1;
    while parsed[idx_right] == -1 {
        idx_right -= 1;
    }

    while parsed[idx_left] != -1 {
        idx_left += 1;
    }

    while idx_left < parsed.len() - free_space as usize {
        if parsed[idx_left] == -1 && parsed[idx_right] != -1 {
            parsed[idx_left] = parsed[idx_right];
            parsed[idx_right] = -1;
            idx_left += 1;
            idx_right -= 1;
        } else if parsed[idx_right] != -1 {
            idx_left += 1;
        } else if parsed[idx_left] == -1 {
            idx_right -= 1
        } else {
            idx_left += 1;
            idx_right -= 1;
        }
    }

    Some(parsed.iter().enumerate().fold(0, |acc, (i, val)| {
        if *val == -1 {
            acc
        } else {
            let x = (i as i64) * val;
            acc + x
        }
    }))
}

#[derive(Debug, Copy, Clone)]
enum Desc {
    Block(i64, usize),
    Free(usize),
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut num = -1;
    let mut parsed: Vec<Desc> = vec![];
    let _ = parse_field(input, |c, (_, col)| {
        let n = c.to_string().parse::<usize>().unwrap();
        let v = if col % 2 == 1 {
            Desc::Free(n)
        } else {
            num += 1;
            Desc::Block(num, n)
        };
        parsed.push(v);
        n
    });

    let mut block = num;
    while block > 0 {
        // get idx of block
        let (idx, desc) = parsed
            .iter()
            .enumerate()
            .find(|(_, v)| match v {
                Desc::Block(b, _) => *b == block,
                _ => false,
            })
            .unwrap();

        // find first free space from right that fits the block (sz)
        if let Desc::Block(_, sz) = *desc {
            let x = parsed
                .iter()
                .enumerate()
                .find(|(_, v)| match v {
                    Desc::Free(s) => *s >= sz,
                    _ => false,
                })
                .to_owned();
            if let Some((fidx, Desc::Free(fsz))) = x {
                if fidx < idx {
                    let fsz = *fsz;
                    // found block
                    if fsz == sz {
                        // exact, just remove block, and replace free block
                        parsed[fidx] = *desc;
                        parsed[idx] = Desc::Free(sz);
                    } else {
                        // split block
                        parsed[fidx] = *desc;
                        parsed[idx] = Desc::Free(sz);
                        parsed.insert(fidx + 1, Desc::Free(fsz - sz));
                    }
                    // coalesce free blocks (if 2 consecutive free, coalesce into 1)
                    for i in 0..parsed.len() - 1 {
                        if i >= parsed.len() - 1 {
                            break;
                        }
                        let x = match parsed[i] {
                            Desc::Free(f1) => match parsed[i + 1] {
                                Desc::Free(f2) => Some(Desc::Free(f1 + f2)),
                                _ => None,
                            },
                            _ => None,
                        };
                        if let Some(f) = x {
                            parsed[i] = f;
                            parsed.remove(i + 1);
                        }
                    }
                }
            }
        } else {
            panic!("bug");
        }

        block -= 1;
    }

    let mut expanded: Vec<i64> = vec![];
    for v in parsed {
        let (val, n) = match v {
            Desc::Block(a, s) => (a, s),
            Desc::Free(s) => (-1, s),
        };
        for _ in 0..n {
            expanded.push(val);
        }
    }

    Some(expanded.iter().enumerate().fold(0, |acc, (i, val)| {
        if *val == -1 {
            acc
        } else {
            let x = (i as i64) * val;
            acc + x
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
