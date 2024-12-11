use std::collections::HashSet;

use advent_of_code::parse_field;

advent_of_code::solution!(10);

fn get_suitable_next_steps((r, c): (usize, usize), field: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let v = field[r][c];
    let sl: Vec<i32> = vec![-1, 0, 1];
    let mut res = vec![];
    for i in sl.clone() {
        for j in sl.clone() {
            if i == 0 && j == 0 || (r == 0 && i == -1) || (c == 0 && j == -1) || i == j || i == -j {
                continue;
            }
            let nr = (r as i32 + i) as usize;
            let nc = (c as i32 + j) as usize;
            if nr < field.len() && nc < field.len() && field[nr][nc] == v + 1 {
                res.push((nr, nc));
            }
        }
    }
    res
}

fn explore_path(
    (r, c): (usize, usize),
    field: &[Vec<u32>],
    acc: &mut HashSet<(usize, usize)>,
) -> usize {
    if field[r][c] == 9 {
        acc.insert((r, c));
        return 1;
    }

    let next = get_suitable_next_steps((r, c), field);
    let mut res = 0;
    for pos in next.iter() {
        res += explore_path((pos.0, pos.1), field, acc);
    }
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut zeroes: HashSet<(usize, usize)> = HashSet::new();
    let field = parse_field(input, |v, pos| {
        let val = v.to_string().parse::<u32>().unwrap();
        if val == 0 {
            zeroes.insert(pos);
        }
        val
    });

    let mut res = 0;
    for zero_pos in zeroes {
        let mut nines = HashSet::new();
        explore_path(zero_pos, &field, &mut nines);
        res += nines.len();
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut zeroes: HashSet<(usize, usize)> = HashSet::new();
    let field = parse_field(input, |v, pos| {
        let val = v.to_string().parse::<u32>().unwrap();
        if val == 0 {
            zeroes.insert(pos);
        }
        val
    });

    let mut res = 0;
    for zero_pos in zeroes {
        let mut nines = HashSet::new();
        res += explore_path(zero_pos, &field, &mut nines);
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
