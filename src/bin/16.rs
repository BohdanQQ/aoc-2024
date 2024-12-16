use std::collections::{HashMap, HashSet};

use advent_of_code::parse_field;
use pathfinding::directed::{dijkstra, yen};

advent_of_code::solution!(16);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum State {
    Up,
    Down,
    Left,
    Right,
}

fn dir_vec(state: &State) -> (i32, i32) {
    match state {
        State::Up => (-1, 0),
        State::Down => (1, 0),
        State::Left => (0, -1),
        State::Right => (0, 1),
    }
}

fn get_rotations(state: &State) -> [State; 2] {
    match state {
        State::Up | State::Down => [State::Left, State::Right],
        State::Left | State::Right => [State::Up, State::Down],
    }
}

fn new_pos_forward((r, c): &Position, state: State, field: &[Vec<char>]) -> Option<Position> {
    let (ir, ic) = (*r as i32, *c as i32);
    let (dr, dc) = dir_vec(&state);
    let (resr, resc) = (ir + dr, ic + dc);
    if resr < 0
        || resr as usize >= field.len()
        || resc < 0
        || field[resr as usize].len() <= resc as usize
        || field[resr as usize][resc as usize] == '#'
    {
        None
    } else {
        Some((resr as usize, resc as usize))
    }
}
type Position = (usize, usize);
type PositionAndDir = (Position, State);
type SuccessorMap = HashMap<PositionAndDir, Vec<(PositionAndDir, u32)>>;

fn construct_successors(input: &str, field: Vec<Vec<char>>) -> SuccessorMap {
    let mut successors: HashMap<PositionAndDir, Vec<(PositionAndDir, u32)>> = HashMap::new();
    let _ = parse_field(input, |v, field_pos| {
        if v == '#' {
            return;
        }
        // println!("{:?}", field_pos);
        for dir in [State::Up, State::Down, State::Right, State::Left] {
            let key = &(field_pos, dir);
            if let Some(pos) = new_pos_forward(&field_pos, dir, &field) {
                let to_insert = ((pos, dir), 1u32);
                if let Some(vec) = successors.get_mut(key) {
                    vec.push(to_insert);
                } else {
                    successors.insert(*key, vec![to_insert]);
                }
            }

            for rot in get_rotations(&dir) {
                if let Some(pos) = new_pos_forward(&field_pos, rot, &field) {
                    let to_insert = ((pos, rot), 1u32 + 1000);
                    if let Some(vec) = successors.get_mut(key) {
                        vec.push(to_insert);
                    } else {
                        successors.insert(*key, vec![to_insert]);
                    }
                }
            }

            if let Some(v) = successors.get(key) {
                if v.is_empty() {
                    successors.remove(key);
                }
            }
        }
        // println!("{:?}", successors);
        // let mut buf = String::new();
        // println!("s {:?}", new_pos_forward(&(4, 1), State::Down, &field));
        // std::io::stdin().read_line(&mut buf);
    });
    successors
}

pub fn part_one(input: &str) -> Option<u32> {
    let (field, start, end) = construct_field(input);
    let successors = construct_successors(input, field);

    assert!(start.is_some() && end.is_some());
    let res = dijkstra::dijkstra(
        &(start.unwrap(), State::Right),
        |&v| (*(successors.get(&v).unwrap_or(&vec![]))).to_vec(),
        |&(r, _)| end.unwrap().0 == r.0 && end.unwrap().1 == r.1,
    );
    if let Some((_, price)) = res {
        Some(price)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (field, start, end) = construct_field(input);
    let successors = construct_successors(input, field);

    let p1res = 85396;
    assert!(start.is_some() && end.is_some());
    let res = yen::yen(
        &(start.unwrap(), State::Right),
        |&v| (*(successors.get(&v).unwrap_or(&vec![]))).to_vec(),
        |&(r, _)| end.unwrap().0 == r.0 && end.unwrap().1 == r.1,
        100,
    );
    let set: HashSet<(usize, usize)> = HashSet::from_iter(
        res.iter()
            .filter(|v| v.1 == p1res)
            .flat_map(|v| v.0.iter().map(|x| x.0)),
    );
    Some(set.len())
}

fn construct_field(input: &str) -> (Vec<Vec<char>>, Option<Position>, Option<Position>) {
    let mut start = None;
    let mut end = None;
    let fld = parse_field(input, |v, (r, c)| {
        if v == 'S' {
            start = Some((r, c));
            'S'
        } else if v == 'E' {
            end = Some((r, c));
            'E'
        } else {
            v
        }
    });
    (fld, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
