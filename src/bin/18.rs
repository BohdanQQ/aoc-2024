use std::{
    collections::{HashMap, HashSet},
    iter::{successors, Successors},
};

use advent_of_code::parse_field;
use pathfinding::directed::dijkstra;
use rayon::{
    iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};

advent_of_code::solution!(18);

type NodeType = (usize, usize);

fn construct_successors(
    obstacles: &Vec<NodeType>,
    bound_x: usize,
    bound_y: usize,
    obst_limit: usize,
) -> HashMap<NodeType, Vec<(NodeType, usize)>> {
    let obstacles = obstacles.iter().take(obst_limit).collect::<HashSet<_>>();
    let mut successors = HashMap::new();

    for y in 0..bound_y {
        for x in 0..bound_x {
            let v = if let std::collections::hash_map::Entry::Vacant(e) = successors.entry((x, y)) {
                let ety: Vec<(NodeType, usize)> = vec![];
                e.insert(ety);
                successors.get_mut(&(x, y)).unwrap()
            } else {
                successors.get_mut(&(x, y)).unwrap()
            };
            for candidate in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let cx = x as i32 + candidate.0;
                let cy = y as i32 + candidate.1;
                if cx < 0 || cy < 0 || cx >= bound_x as i32 || cy >= bound_y as i32 {
                    continue;
                }
                let usize_cand = (cx as usize, cy as usize);
                if obstacles.contains(&usize_cand) {
                    continue;
                }

                v.push(((cx as usize, cy as usize), 1));
            }
        }
    }

    successors
}

pub fn part_one(input: &str) -> Option<usize> {
    let positions = input
        .split("\n")
        .map(|ln| ln.split(",").map(|v| v.parse::<usize>().unwrap()))
        .map(|mut v| (v.next().unwrap(), v.next().unwrap()))
        .collect::<Vec<_>>();
    let bound = 7;
    let end = (bound - 1, bound - 1);
    let succ = construct_successors(&positions, bound, bound, 12);
    let res = dijkstra::dijkstra(
        &(0, 0),
        |&v| (*(succ.get(&v).unwrap_or(&vec![]))).to_vec(),
        |&(x, y)| end.0 == x && end.1 == y,
    );
    if let Some((_, price)) = res {
        Some(price)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let positions = input
        .split("\n")
        .map(|ln| ln.split(",").map(|v| v.parse::<usize>().unwrap()))
        .map(|mut v| (v.next().unwrap(), v.next().unwrap()))
        .collect::<Vec<_>>();
    let bound = 71;
    let end = (bound - 1, bound - 1);
    let bytes = 1024;
    let res = (bytes..positions.len())
        .into_par_iter()
        .find_map_first(|idx| {
            let succ = construct_successors(&positions, bound, bound, idx);
            let res = dijkstra::dijkstra(
                &(0, 0),
                |&v| (*(succ.get(&v).unwrap_or(&vec![]))).to_vec(),
                |&(x, y)| end.0 == x && end.1 == y,
            );
            if res.is_none() {
                Some(positions[idx - 1])
            } else {
                None
            }
        });
    println!("{:?}", res);
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
