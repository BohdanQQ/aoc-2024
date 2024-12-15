use std::mem;

advent_of_code::solution!(15);

#[derive(Clone, Copy)]
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
type FieldV = Vec<Vec<MapObj>>;
type FieldS = [Vec<MapObj>];

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
// PART 1 SPECIFICS ------------------------------------------------------------
// looks in 1 direction and searches for 1 free space (allows the shift)
fn has_empty_in_dir(
    row_shift: i32,
    col_shift: i32,
    fld: &mut FieldS,
    from_row: usize,
    from_col: usize,
) -> bool {
    let mut now_row = from_row as i32 + row_shift;
    let mut now_col = from_col as i32 + col_shift;
    let limit = fld.len() as i32;
    let out_of_bounds =
        |x: i32, y: i32| x < 0 || x >= limit || y >= fld[x as usize].len() as i32 || y < 0;
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

fn shift_col(fld: &mut FieldS, (rob_row, rob_col): (usize, usize), shift: i32) {
    let mut prev_val = MapObj::Empty;
    let mut next = rob_row as i32;
    while fld[next as usize][rob_col] != MapObj::Empty {
        mem::swap(&mut prev_val, &mut fld[next as usize][rob_col]);
        next += shift;
    }
    fld[next as usize][rob_col] = prev_val;
}

fn shift_row(fld: &mut FieldS, (rob_row, rob_col): (usize, usize), shift: i32) {
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
    fld: &mut FieldV,
    (rob_row, rob_col): (usize, usize),
    insn: Insn,
) -> Option<(usize, usize)> {
    let (dir_row, dir_col) = match insn {
        Insn::Down => (1, 0),
        Insn::Up => (-1, 0),
        Insn::Left => (0, -1),
        Insn::Right => (0, 1),
    };
    // check if there is space for the very last box in the row
    // (or just the robot)
    if !has_empty_in_dir(dir_row, dir_col, fld, rob_row, rob_col) {
        return None;
    }
    // println!("OK to shift {rob_row} {rob_col} by {dir_row} {dir_col}");
    // perform shifting
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
// PART 1 SPECIFICS END --------------------------------------------------------

fn parser(input: &str) -> (Vec<Vec<MapObj>>, Vec<Insn>) {
    let line_iter = input.split("\n");
    let line_iter2 = line_iter.clone();
    let lines_map = line_iter.take_while(|l| !l.is_empty());
    let n = lines_map.clone().count();
    let insn_lines = line_iter2.skip(n + 1).collect::<String>();
    let instructions = insn_lines.trim().chars().map(Insn::from).collect();

    let field = lines_map
        .map(|l| l.chars().map(MapObj::from).collect())
        .collect();
    (field, instructions)
}

fn part_driver<T>(
    instructions: &[Insn],
    field: &mut FieldV,
    step_fn: &mut T,
    score_match: MapObj,
) -> usize
where
    T: FnMut(&mut FieldV, (usize, usize), Insn) -> Option<(usize, usize)>,
{
    // find the robot
    let mut robot_pos = None;
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == MapObj::Robot {
                assert!(robot_pos.is_none(), "bug: duplicate robot position");
                robot_pos = Some((row, col));
            }
        }
    }
    // print_field(&field);
    // perform steps
    let (mut r, mut c) = robot_pos.unwrap();
    for insn in instructions {
        if let Some((nr, nc)) = step_fn(field, (r, c), *insn) {
            // println!("Shifted to {nr} {nc}");
            r = nr;
            c = nc;
        } else {
            // println!("NOOP");
        }
    }
    // print_field(&field);
    // calculate score
    let mut res = 0;
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == score_match {
                res += row * 100 + col;
            }
        }
    }

    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut field, instructions) = parser(input);
    Some(part_driver(
        &instructions,
        &mut field,
        &mut step,
        MapObj::Obstacle,
    ))
}

fn shift_cols_2(fld: &mut FieldS, (from_r, from_c): (usize, usize), shift: i32) -> bool {
    let nextr = from_r as i32 + shift;
    if nextr < 0 || nextr as usize >= fld.len() {
        return false;
    }
    let nextr = nextr as usize;
    let next_obj = fld[nextr][from_c];
    let moved = match next_obj {
        MapObj::Robot | MapObj::Obstacle => panic!("wtf {:?}", next_obj),
        MapObj::Empty => true,
        MapObj::Wall => false,
        MapObj::ObstacleOpen => {
            shift_cols_2(fld, (nextr, from_c + 1), shift)
                && shift_cols_2(fld, (nextr, from_c), shift)
        }
        MapObj::ObstacleClose => {
            shift_cols_2(fld, (nextr, from_c - 1), shift)
                && shift_cols_2(fld, (nextr, from_c), shift)
        }
    };

    if moved {
        fld[nextr][from_c] = fld[from_r][from_c];
        fld[from_r][from_c] = MapObj::Empty;
    }

    moved
}

fn step_2(
    fld: &mut FieldV,
    (rob_row, rob_col): (usize, usize),
    insn: Insn,
) -> Option<(usize, usize)> {
    let (dir_row, dir_col) = match insn {
        Insn::Down => (1, 0),
        Insn::Up => (-1, 0),
        Insn::Left => (0, -1),
        Insn::Right => (0, 1),
    };

    if dir_row == 0 {
        // check if there is space for the very last box in the row
        // (or just the robot)
        if !has_empty_in_dir(dir_row, dir_col, fld, rob_row, rob_col) {
            return None;
        }
        shift_row(fld, (rob_row, rob_col), dir_col);
    } else {
        let prev = fld.clone(); // oof but makes my life easier
        if !shift_cols_2(fld, (rob_row, rob_col), dir_row) {
            *fld = prev;
            return None;
        }
    }
    Some((
        (rob_row as i32 + dir_row) as usize,
        (rob_col as i32 + dir_col) as usize,
    ))
}

// part 1 input -> part 2 input
fn p2_enlarge(map: &FieldS) -> FieldV {
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
    // part 2 algo had to be changed because the part 1 would get extremely sketchy
    let (field, instructions) = parser(input);
    let mut field = p2_enlarge(&field);
    Some(part_driver(
        &instructions,
        &mut field,
        &mut step_2,
        MapObj::ObstacleOpen,
    ))
}

fn print_field(fld: &FieldS) {
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
        assert_eq!(result, Some(105));
    }
}
