use std::{cmp::Ordering, collections::HashSet};

use advent_of_code::parse_field;

advent_of_code::solution!(12);

pub fn dfs_same_chars(
    tgt: char,
    (start_r, start_c): (usize, usize),
    field: &mut [Vec<(char, bool)>],
    acc: &mut HashSet<(usize, usize)>,
) {
    let r: &mut (char, bool) = &mut field[start_r][start_c];
    if r.1 {
        return;
    }

    if r.0 == tgt {
        acc.insert((start_r, start_c));
        r.1 = true;
    } else {
        return;
    }
    let offsets = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dr, dc) in offsets {
        let next_r = start_r as i32 + dr;
        let next_c = start_c as i32 + dc;

        if next_r < 0 || next_c < 0 || next_r >= field.len() as i32 || next_c >= field.len() as i32
        {
            continue;
        }

        dfs_same_chars(tgt, (next_r as usize, next_c as usize), field, acc);
    }
}
// just try every side of a field in an area - if not in the area -> got a piece of perimeter
pub fn perimeter(area: &HashSet<(usize, usize)>, acc: &mut Vec<(i32, i32)>, field_limit: usize) {
    let offsets = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (start_r, start_c) in area.iter() {
        for (dr, dc) in offsets.iter() {
            let next_r = *start_r as i32 + dr;
            let next_c = *start_c as i32 + dc;

            if next_r < -1
                || next_c < -1
                || next_r > field_limit as i32
                || next_c > field_limit as i32
                || area.contains(&(next_r as usize, next_c as usize))
            {
                continue;
            }

            acc.push((next_r, next_c));
        }
    }
}

// 3.5 11.1

fn filter_into_vec<
    'a,
    T: Fn(&&(usize, usize)) -> bool,
    S: Fn(&&(usize, usize), &&(usize, usize)) -> Ordering,
>(
    in_seq: &'a [&(usize, usize)],
    filter_fn: T,
    sorter: S,
) -> Vec<&'a (usize, usize)> {
    let mut rv = in_seq.iter().cloned().filter(filter_fn).collect::<Vec<_>>();
    rv.sort_by(sorter);
    rv
}

// further abstractions over scanx and scany would make them incomprehensible
pub fn scan_x(r_coord: usize, area: &HashSet<(usize, usize)>) -> usize {
    let on_row = area
        .iter()
        .filter(|(r, _)| *r == r_coord)
        .collect::<Vec<_>>();

    let sorter = |a: &&(usize, usize), b: &&(usize, usize)| a.1.cmp(&b.1);
    // this trick allows us to count the leftmost and topmost sides (0-th row implies the element MUST be filtered IN)
    let on_row_over = filter_into_vec(
        &on_row,
        |(r, c)| (*r == 0 || !area.contains(&(r - 1, *c))),
        sorter,
    );
    let on_row_below = filter_into_vec(&on_row, |(r, c)| !area.contains(&(r + 1, *c)), sorter);
    let mut res = 0;
    let mut add_consecutive = |row: Vec<&(usize, usize)>| {
        if !row.is_empty() {
            res += 1;
            let mut last = row[0].1;
            for (_, c) in row {
                if *c > last + 1 {
                    res += 1;
                }
                last = *c;
            }
        }
    };
    add_consecutive(on_row_over);
    add_consecutive(on_row_below);
    res
}

pub fn scan_y(c_coord: usize, area: &HashSet<(usize, usize)>) -> usize {
    let on_col = area
        .iter()
        .filter(|(_, c)| *c == c_coord)
        .collect::<Vec<_>>();
    let sorter = |a: &&(usize, usize), b: &&(usize, usize)| a.0.cmp(&b.0);
    let on_col_left = filter_into_vec(
        &on_col,
        |(r, c)| (*c == 0 || !area.contains(&(*r, c - 1))),
        sorter,
    );
    let on_col_right = filter_into_vec(&on_col, |(r, c)| !area.contains(&(*r, c + 1)), sorter);
    let mut res = 0;
    let mut add_consecutive = |col: Vec<&(usize, usize)>| {
        if !col.is_empty() {
            res += 1;
            let mut last = col[0].0;
            for (r, _) in col {
                if *r > last + 1 {
                    res += 1;
                }
                last = *r;
            }
        }
    };
    add_consecutive(on_col_left);
    add_consecutive(on_col_right);
    res
}

fn get_areas(field: &mut [Vec<(char, bool)>]) -> Vec<HashSet<(usize, usize)>> {
    let mut areas: Vec<HashSet<(usize, usize)>> = vec![];
    for row in 0..field.len() {
        for col in 0..field.len() {
            if field[row][col].1 {
                continue;
            }
            let mut res = HashSet::new();
            dfs_same_chars(field[row][col].0, (row, col), field, &mut res);
            areas.push(res);
        }
    }
    areas
}

pub fn part_one(input: &str) -> Option<usize> {
    // parsing field for area lookup -> inserts a visited flag next to each character
    let mut field = parse_field(input, |v, _| (v, false));
    let areas = get_areas(&mut field);

    let mut result = 0;
    let mut perimeters: Vec<Vec<(i32, i32)>> = vec![];
    for area in areas.iter() {
        let mut res = Vec::new();
        perimeter(area, &mut res, field.len());
        result += area.len() * res.len();
        perimeters.push(res);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut field = parse_field(input, |v, _| (v, false));
    let areas = get_areas(&mut field);

    let mut result = 0;
    for area in areas.iter() {
        let mut sides = 0;
        for x in 0..field.len() {
            // the idea is to scan the area horizontally and vertically
            // (needs sorting and detection of [non]consecutive ranges)
            sides += scan_x(x, area);
            sides += scan_y(x, area);
        }
        result += sides * area.len();
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(140));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }
}
