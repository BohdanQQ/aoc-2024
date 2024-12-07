use std::{
    sync::{atomic, Arc},
    thread,
};

advent_of_code::solution!(6);

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pos {
    Air,
    Obstacle,
    Guard,
    Visited(i8, i8),
}

fn turn_right(dir_row: &mut i8, dir_col: &mut i8) {
    let help = *dir_col;
    *dir_col = -(*dir_row);
    *dir_row = help;
}

// part one, does not use visited, uses guard of Pos
// mutate the field, position, return nr of tiles changed
fn do_step(
    field: &mut [Vec<Pos>],
    row: &mut usize,
    col: &mut usize,
    dir_row: &mut i8,
    dir_col: &mut i8,
) -> Option<u32> {
    let next_r = *row as i64 + *dir_row as i64;
    let next_c = *col as i64 + *dir_col as i64;
    if next_r < 0 || next_c < 0 || next_c as usize >= field.len() || next_r as usize >= field.len()
    {
        return None;
    }

    // Next Usize Row / Column
    let nur = next_r as usize;
    let nuc = next_c as usize;
    if field[nur][nuc] == Pos::Obstacle {
        turn_right(dir_row, dir_col);
        Some(0)
    } else {
        let res = if field[nur][nuc] == Pos::Air { 1 } else { 0 };
        field[nur][nuc] = Pos::Guard;
        *row = nur;
        *col = nuc;
        Some(res)
    }
}

// part one, does not use visited, uses guard of Pos
// mutate the field, position, return nr of tiles changed
fn do_step_2(
    field: &mut [Vec<Pos>],
    row: &mut usize,
    col: &mut usize,
    dir_row: &mut i8,
    dir_col: &mut i8,
) -> Option<bool> {
    let next_r = *row as i64 + *dir_row as i64;
    let next_c = *col as i64 + *dir_col as i64;
    if next_r < 0 || next_c < 0 || next_c as usize >= field.len() || next_r as usize >= field.len()
    {
        return Some(false);
    }

    // Next Usize Row / Column
    let nur = next_r as usize;
    let nuc = next_c as usize;
    let cur = &field[nur][nuc];

    if let Pos::Visited(i, j) = cur {
        if i == dir_row && j == dir_col {
            return Some(true);
        }
    }

    if field[nur][nuc] == Pos::Obstacle {
        turn_right(dir_row, dir_col);
        None
    } else {
        field[nur][nuc] = Pos::Visited(*dir_row, *dir_col);
        *row = nur;
        *col = nuc;
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parsed = input
        .split_ascii_whitespace()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == '#' {
                        Pos::Obstacle
                    } else if c == '.' {
                        Pos::Air
                    } else {
                        Pos::Guard
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut guard = (0, 0);
    for i in 0..parsed.len() {
        for j in 0..parsed.len() {
            if parsed[i][j] == Pos::Guard {
                guard = (i, j);
            }
        }
    }
    let (mut x, mut y) = guard;
    let (mut dx, mut dy) = (-1, 0);

    let mut res = 1;
    while let Some(n) = do_step(&mut parsed, &mut x, &mut y, &mut dx, &mut dy) {
        res += n;
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = input
        .split_ascii_whitespace()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == '#' {
                        Pos::Obstacle
                    } else if c == '.' {
                        Pos::Air
                    } else {
                        Pos::Guard
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mtx = Arc::new(parsed);
    let parsed = mtx.clone();
    let mut guard = (0, 0);
    for i in 0..parsed.len() {
        for j in 0..parsed.len() {
            if parsed[i][j] == Pos::Guard {
                guard = (i, j);
            }
        }
    }

    // 4s to 500ms (debug) (release 55ms)
    let total = Arc::from(atomic::AtomicU32::new(0));
    thread::scope(|c| {
        for i in 0..parsed.len() {
            let parsed = mtx.clone();
            let total = total.clone();
            c.spawn(move || {
                let mut subtotal = 0;
                for j in 0..parsed.len() {
                    let (mut x, mut y) = guard;
                    let (mut dx, mut dy) = (-1, 0);

                    let mut working_cpy = Vec::from_iter(parsed.clone().iter().cloned());
                    if i != x || j != y {
                        // one could also replace just the positions from the part one,
                        // which became marked and place an obstacle there
                        // but since this was fast enough and actually fast to implement
                        // i cant be bothered xd
                        working_cpy[i][j] = Pos::Obstacle;
                    }

                    loop {
                        let res = do_step_2(&mut working_cpy, &mut x, &mut y, &mut dx, &mut dy);
                        if let Some(b) = res {
                            subtotal += if b { 1 } else { 0 };
                            break;
                        }
                    }
                }
                // we dont have to care since we're in a thread scope
                total.fetch_add(subtotal, atomic::Ordering::Relaxed);
            });
        }
    });

    Some(total.load(atomic::Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
