use std::mem;

advent_of_code::solution!(15);

enum Insn {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Insn {
    fn from(value: char) -> Self {
        match value {
            'v' => Insn::Down,
            '>' => Insn::Right,
            '<' => Insn::Left,
            '^' => Insn::Up,
            _ => panic!("Unexpected"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum MapObj {
    Robot,
    Empty,
    Obstacle,
    Wall,
    ObstacleOpen,
    ObstacleClose,
}

impl From<char> for MapObj {
    fn from(value: char) -> Self {
        match value {
            '@' => MapObj::Robot,
            '.' => MapObj::Empty,
            'O' => MapObj::Obstacle,
            '#' => MapObj::Wall,
            _ => panic!("Unexpected"),
        }
    }
}

fn has_empty_in_dir(
    row_shift: i32,
    col_shift: i32,
    fld: &mut [Vec<MapObj>],
    from_row: usize,
    from_col: usize,
) -> bool {
    let mut now_row = from_row as i32 + row_shift;
    let mut now_col = from_col as i32 + col_shift;
    let limit = fld.len() as i32;
    let out_of_bounds = |x: i32, y: i32| x < 0 || x >= limit || y >= limit || y < 0;
    // println!("Try shift {row_shift} {col_shift} @ {from_row} {from_col}");
    while !out_of_bounds(now_row, now_col) {
        // println!("{} {}", now_row, now_col);
        let r = now_row as usize;
        let c = now_col as usize;
        let curr = fld[r][c];
        if curr == MapObj::Empty {
            // println!("Found at {r} {c}");
            return true;
        } else if curr == MapObj::Wall {
            return false;
        }

        now_row += row_shift;
        now_col += col_shift;
    }
    false
}

fn shift_col(fld: &mut [Vec<MapObj>], (rob_row, rob_col): (usize, usize), shift: i32) {
    let mut prev_val = MapObj::Empty;
    let mut next = rob_row as i32;
    while fld[next as usize][rob_col] != MapObj::Empty {
        mem::swap(&mut prev_val, &mut fld[next as usize][rob_col]);
        next += shift;
    }
    fld[next as usize][rob_col] = prev_val;
}

fn shift_row(fld: &mut [Vec<MapObj>], (rob_row, rob_col): (usize, usize), shift: i32) {
    let mut prev_val = MapObj::Empty;
    let mut next = rob_col as i32;
    while fld[rob_row][next as usize] != MapObj::Empty {
        // println!("Next: {next} ({}) at {rob_row}", next as usize);
        mem::swap(&mut prev_val, &mut fld[rob_row][next as usize]);
        next += shift;
    }
    fld[rob_row][next as usize] = prev_val;
}

fn step(
    fld: &mut [Vec<MapObj>],
    (rob_row, rob_col): (usize, usize),
    insn: Insn,
) -> Option<(usize, usize)> {
    let (dir_row, dir_col) = match insn {
        Insn::Down => (1, 0),
        Insn::Up => (-1, 0),
        Insn::Left => (0, -1),
        Insn::Right => (0, 1),
    };
    if !has_empty_in_dir(dir_row, dir_col, fld, rob_row, rob_col) {
        return None;
    }

    // println!("OK to shift {rob_row} {rob_col} by {dir_row} {dir_col}");

    if dir_col == 0 {
        shift_col(fld, (rob_row, rob_col), dir_row);
    } else {
        shift_row(fld, (rob_row, rob_col), dir_col);
    }

    Some((
        (rob_row as i32 + dir_row) as usize,
        (rob_col as i32 + dir_col) as usize,
    ))
}

fn pf(fld: &[Vec<MapObj>]) {
    for r in fld {
        for c in r {
            let chr = match c {
                MapObj::Empty => '.',
                MapObj::Robot => '@',
                MapObj::Obstacle => 'O',
                MapObj::Wall => '#',
                MapObj::ObstacleOpen => '[',
                MapObj::ObstacleClose => ']',
            };
            print!("{}", chr);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let line_iter = input.split("\n");
    let line_iter2 = line_iter.clone();
    let lines_map = line_iter.take_while(|l| !l.is_empty());
    let n = lines_map.clone().count();
    let insn_lines = line_iter2.skip(n + 1).collect::<String>();
    let instructions = insn_lines.trim().chars().map(Insn::from);

    let mut robot_pos = None;
    let mut field = lines_map
        .map(|l| l.chars().map(MapObj::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for row in 0..field.len() {
        for col in 0..field.len() {
            if field[row][col] == MapObj::Robot {
                if robot_pos.is_some() {
                    panic!("bug robot search");
                }
                robot_pos = Some((row, col));
            }
        }
    }
    let (mut r, mut c) = robot_pos.unwrap();

    for insn in instructions {
        if let Some((nr, nc)) = step(&mut field, (r, c), insn) {
            // println!("Shifted to {nr} {nc}");
            r = nr;
            c = nc;
        } else {
            // println!("NOOP");
        }
        // pf(&field);
    }

    let mut res = 0;
    for row in 0..field.len() {
        for col in 0..field.len() {
            if field[row][col] == MapObj::Obstacle {
                res += row * 100 + col;
            }
        }
    }

    Some(res)
}

fn p2_enlarge(map: &[Vec<MapObj>]) -> Vec<Vec<MapObj>> {
    let mut res = Vec::with_capacity(map.len());
    for r in map {
        let mut nr = Vec::with_capacity(map.len() * 2);
        for c in r {
            match c {
                MapObj::Robot => {
                    nr.push(*c);
                    nr.push(MapObj::Empty);
                }
                MapObj::Obstacle => {
                    nr.push(MapObj::ObstacleOpen);
                    nr.push(MapObj::ObstacleClose);
                }
                x => {
                    nr.push(*x);
                    nr.push(*x);
                }
            };
        }
        res.push(nr);
    }

    res
}

pub fn part_two(input: &str) -> Option<usize> {
    let line_iter = input.split("\n");
    let line_iter2 = line_iter.clone();
    let lines_map = line_iter.take_while(|l| !l.is_empty());
    let n = lines_map.clone().count();
    let insn_lines = line_iter2.skip(n + 1).collect::<String>();
    let instructions = insn_lines.trim().chars().map(Insn::from);

    let mut robot_pos = None;
    let field = lines_map
        .map(|l| l.chars().map(MapObj::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let field = p2_enlarge(&field);

    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == MapObj::Robot {
                if robot_pos.is_some() {
                    panic!("bug robot search");
                }
                robot_pos = Some((row, col));
            }
        }
    }
    pf(&field);
    let (mut r, mut c) = robot_pos.unwrap();

    // damn, i would need to rewrite a lot...

    Some(r * c * instructions.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(908));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
