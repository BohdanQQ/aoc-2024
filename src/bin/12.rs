use std::collections::HashSet;

use advent_of_code::parse_field;

advent_of_code::solution!(12);

pub fn dfs_same_chars(
    tgt: char,
    (start_r, start_c): (usize, usize),
    field: &mut Vec<Vec<(char, bool)>>,
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

// i hate this but i already spent so much time on this shit lol
pub fn scan_x(r_coord: usize, area: &HashSet<(usize, usize)>) -> usize {
    let mut on_row = area.iter().filter(|(r, c)| *r == r_coord).collect::<Vec<_>>();

    on_row.sort_by(|a, b| a.1.cmp(&b.1));
    let on_row_over = on_row.iter().cloned().filter(|(r, c)| (*r == 0 || !area.contains(&(r-1, *c)))).collect::<Vec<_>>();
    let on_row_below = on_row.iter().cloned().filter(|(r, c)| !area.contains(&(r+1, *c))).collect::<Vec<_>>();
    // println!("pre row : {:?}", on_row);
    // println!("row ov: {:?}", on_row_over);
    // println!("row be: {:?}", on_row_below);

    let mut res = 0;
    if !on_row_over.is_empty() {
        res += 1;
        let mut last = on_row_over[0].1;
        for (_, c) in on_row_over {
            if *c > last + 1 {
                res += 1;
                // println!("New start {}", c);
            }
            last = *c;
        }
    }

    if !on_row_below.is_empty() {
        res += 1;
        let mut last = on_row_below[0].1;
        for (_, c) in on_row_below {
            if *c > last + 1 {
                res += 1;
                // println!("New start {}", c);
            }
            last = *c;
        }
    }
    res
}

pub fn scan_y(c_coord: usize, area: &HashSet<(usize, usize)>) -> usize {
    let mut on_col = area.iter().filter(|(r, c)| *c == c_coord).collect::<Vec<_>>();

    on_col.sort_by(|a, b| a.0.cmp(&b.0));
    let on_col_right = on_col.iter().cloned().filter(|(r, c)| !area.contains(&(*r, c + 1))).collect::<Vec<_>>();
    let on_col_left = on_col.iter().cloned().filter(|(r, c)| (*c == 0 || !area.contains(&(*r, c - 1)))).collect::<Vec<_>>();
    // println!("pre col : {:?}", on_col);
    // println!("col left: {:?}", on_col_left);
    // println!("col right: {:?}", on_col_right);
    let mut res = 0;
    if !on_col_left.is_empty() {
        res += 1;
        let mut last = on_col_left[0].0;
        for (r, _) in on_col_left {
            if *r > last + 1{
                res += 1;
            }
            last = *r;
        }
    }

    if !on_col_right.is_empty() {
        res += 1;
        let mut last = on_col_right[0].0;
        for (r, _) in on_col_right {
            if *r > last + 1 {
                res += 1;
            }
            last = *r;
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    // explored flag
    let mut field = parse_field(input, |v, pos| (v, false));
    let mut areas: Vec<HashSet<(usize, usize)>> = vec![];

    for row in 0..field.len() {
        for col in 0..field.len() {
            if field[row][col].1 {
                continue;
            }
            let mut res = HashSet::new();
            dfs_same_chars(field[row][col].0, (row, col), &mut field, &mut res);
            areas.push(res);
        }
    }
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
    // explored flag
    let mut field = parse_field(input, |v, pos| (v, false));
    let mut areas: Vec<HashSet<(usize, usize)>> = vec![];

    for row in 0..field.len() {
        for col in 0..field.len() {
            if field[row][col].1 {
                continue;
            }
            let mut res = HashSet::new();
            dfs_same_chars(field[row][col].0, (row, col), &mut field, &mut res);
            areas.push(res);
        }
    }
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
