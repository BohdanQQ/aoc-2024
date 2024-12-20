advent_of_code::solution!(20);

use std::{
    collections::HashMap,
    sync::{atomic, Arc},
};

use advent_of_code::parse_field;
use pathfinding::directed::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Position = (usize, usize);
type SuccessorMap = HashMap<Position, Vec<(Position, u32)>>;

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

fn in_field((r, c): &(i32, i32), field: &[Vec<char>]) -> bool {
    !(*r < 0 || *r as usize >= field.len() || *c < 0 || *c as usize >= field[*r as usize].len())
}

// normal SuccessorMap construction without cheating
fn construct_successors(input: &str, field: &[Vec<char>]) -> SuccessorMap {
    let mut successors: HashMap<Position, Vec<(Position, u32)>> = HashMap::new();
    let _ = parse_field(input, |_, field_pos| {
        if field[field_pos.0][field_pos.1] == '#' {
            return;
        }
        let mut successor_list = vec![];
        let from_node = field_pos;
        for dir in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let upcoming_pos = (field_pos.0 as i32 + dir.0, field_pos.1 as i32 + dir.1);
            if !in_field(&upcoming_pos, field) {
                continue;
            }
            // I forgot about this skip and it cost me additional 80% time in part 2
            // and ~33% in part 1
            if field[upcoming_pos.0 as usize][upcoming_pos.1 as usize] == '#' {
                continue;
            }
            successor_list.push(((upcoming_pos.0 as usize, upcoming_pos.1 as usize), 1u32));
        }
        successors.insert(from_node, successor_list);
    });
    successors
}

// adds a manhattan circle around the manhattan_target as the list of its successors
// otherwise the returned SuccessorMap is normal
fn construct_successors_manhattan(
    manhattan_target: (usize, usize),
    input: &str,
    field: &[Vec<char>],
    manhattan_budget: u32,
) -> SuccessorMap {
    let mut options = HashMap::new();
    get_manhattan_options(&manhattan_target, field, manhattan_budget, &mut options);
    let options = options.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
    let mut successors: HashMap<Position, Vec<(Position, u32)>> =
        construct_successors(input, field);
    let mut successor_map: HashMap<(usize, usize), u32> = HashMap::new();
    for (pos, cost) in options {
        if pos.0 != manhattan_target.0 || pos.1 != manhattan_target.1 {
            successor_map
                .entry(pos)
                .and_modify(|v| {
                    *v = (*v).min(cost);
                })
                .or_insert(cost);
        }
    }
    successors.remove(&manhattan_target);
    successors.insert(
        manhattan_target,
        Vec::from_iter(successor_map.iter().map(|x| (*x.0, *x.1))),
    );
    successors
}

fn get_manhattan_options(
    (or, oc): &Position,
    field: &[Vec<char>],
    budget: u32,
    accumulator: &mut HashMap<Position, u32>,
) {
    if budget == 0 {
        return;
    }
    let range = -(budget as i32)..=(budget as i32);
    for dr in range.clone() {
        for dc in -(budget as i32 - dr.abs())..=(budget as i32 - dr.abs()) {
            if dc == 0 && dr == 0 {
                continue;
            }
            let (nr, nc) = (*or as i32 + dr, *oc as i32 + dc);
            if !in_field(&(nr, nc), field) {
                continue;
            }
            let (cand_r, cand_c) = (nr as usize, nc as usize);
            if field[cand_r][cand_c] != '#' {
                let cost = (dr.abs() + dc.abs()) as u32;
                accumulator
                    .entry((cand_r, cand_c))
                    .and_modify(|v: &mut u32| {
                        *v = (*v).min(cost);
                    })
                    .or_insert(cost);
            }
        }
    }
}

fn get_used_cheat_from_path(path: &[Position]) -> Option<(Position, Position)> {
    for (p1, p2) in path.windows(2).map(|s| (s[0], s[1])) {
        if p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) > 1 {
            return Some((p1, p2));
        }
    }
    None
}

// construct successors with cheats
// run dijkstra until you don't have any suitable paths
// if you have a suitable path, find the cheat used in the path and remove it from succesor list (and register this path/cheat)
// return number of discovered cheats/paths

// could be optimized by retaining partial searching state in the point of a used cheat
// after removing a cheat, you restore the searching state and continue with dijkstra
// this of course is not supported by the library im using and i cannot be bothered to go that low level
// algorithmically-wise
// on my 16-core laptop this runs ~210s for part 2
pub fn solver<T: Fn(&u32, &u32) -> bool + Send + Sync>(
    input: &str,
    picos: u32,
    filter_fn: T,
) -> Option<usize> {
    let (field, start, end) = construct_field(input);
    let successors = construct_successors(input, &field);

    assert!(start.is_some() && end.is_some());
    let (nocheat_path, nocheat_cost) = dijkstra::dijkstra(
        &start.unwrap(),
        |&v| (*(successors.get(&v).unwrap_or(&vec![]))).to_vec(),
        |&(r, c)| end.unwrap().0 == r && end.unwrap().1 == c,
    )
    .unwrap();

    // uncomment for progress counting (also the println in map)
    //      - expect ~10% perf impact depending on core count
    // let counter = Arc::new(atomic::AtomicU32::new(nocheat_path.len() as u32));
    let res: usize = nocheat_path
        .par_iter()
        .map(|manhattan_target| {
            // println!(
            //     "Remaining: {:?}",
            //     counter.clone().fetch_sub(1, atomic::Ordering::Relaxed)
            // );
            let loc_field = field.clone();
            let mut loc_successors =
                construct_successors_manhattan(*manhattan_target, input, &loc_field, picos);
            let mut acc_res = 0;

            while let Some((path, cost)) = dijkstra::dijkstra(
                &start.unwrap(),
                |&v| (*(loc_successors.get(&v).unwrap_or(&vec![]))).to_vec(),
                |&(r, c)| end.unwrap().0 == r && end.unwrap().1 == c,
            ) {
                if !filter_fn(&cost, &nocheat_cost) {
                    break;
                }
                if let Some((cheat_start, cheat_end)) = get_used_cheat_from_path(&path) {
                    acc_res += 1;
                    let su = loc_successors.get_mut(&cheat_start).unwrap();
                    let idx = su.iter().position(|(p, _)| *p == cheat_end).unwrap();
                    su.swap_remove(idx);
                }
            }
            acc_res
        })
        .sum();
    Some(res)
}

pub fn part_one(input: &str) -> Option<usize> {
    let filter_fn = |cost: &u32, nocheat_cost: &u32| *cost < *nocheat_cost;
    // comment out the line below for EXAMPLE input
    let filter_fn = |cost: &u32, nocheat_cost: &u32| *cost + 100 <= *nocheat_cost;
    solver(input, 2, filter_fn)
}

pub fn part_two(input: &str) -> Option<usize> {
    let filter_fn = |cost: &u32, nocheat_cost: &u32| *cost + 50 <= *nocheat_cost;
    // comment out the line below for EXAMPLE input
    let filter_fn = |cost: &u32, nocheat_cost: &u32| *cost + 100 <= *nocheat_cost;
    solver(input, 20, filter_fn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
