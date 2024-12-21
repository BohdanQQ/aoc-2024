use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

advent_of_code::solution!(21, 2);

type Position = (u64, u64);

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
enum NumericKey {
    Number(u8),
    Activate,
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
enum DirectionalKey {
    ArrowL,
    ArrowR,
    ArrowU,
    ArrowD,
    Activate,
}

trait KeypadKey {
    fn get_coordinates(&self) -> Position;
}

impl KeypadKey for NumericKey {
    fn get_coordinates(&self) -> Position {
        match self {
            NumericKey::Number(0) => (0, 1),
            NumericKey::Number(1) => (1, 0),
            NumericKey::Number(2) => (1, 1),
            NumericKey::Number(3) => (1, 2),
            NumericKey::Number(4) => (2, 0),
            NumericKey::Number(5) => (2, 1),
            NumericKey::Number(6) => (2, 2),
            NumericKey::Number(7) => (3, 0),
            NumericKey::Number(8) => (3, 1),
            NumericKey::Number(9) => (3, 2),
            NumericKey::Activate => (0, 2),
            _ => panic!("unknown key"),
        }
    }
}

impl KeypadKey for DirectionalKey {
    fn get_coordinates(&self) -> Position {
        match self {
            DirectionalKey::ArrowL => (0, 0),
            DirectionalKey::ArrowR => (0, 2),
            DirectionalKey::ArrowU => (1, 1),
            DirectionalKey::ArrowD => (0, 1),
            DirectionalKey::Activate => (1, 2),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum PushInsn {
    L,
    R,
    U,
    D,
    A,
}

impl TryFrom<(i32, i32)> for PushInsn {
    type Error = String;
    fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
        match value {
            (1, 0) => Ok(Self::U),
            (-1, 0) => Ok(Self::D),
            (0, 1) => Ok(Self::R),
            (0, -1) => Ok(Self::L),
            _ => Err(format!("{:?}", value)),
        }
    }
}

impl EnumAll for PushInsn {
    fn get_all() -> Vec<Self>
    where
        Self: std::marker::Sized,
    {
        vec![Self::L, Self::R, Self::U, Self::D, Self::A]
    }
}

trait EnumAll {
    fn get_all() -> Vec<Self>
    where
        Self: std::marker::Sized;
}

impl EnumAll for DirectionalKey {
    fn get_all() -> Vec<Self> {
        vec![
            Self::Activate,
            Self::ArrowD,
            Self::ArrowL,
            Self::ArrowR,
            Self::ArrowU,
        ]
    }
}

impl EnumAll for NumericKey {
    fn get_all() -> Vec<Self> {
        vec![
            Self::Number(0),
            Self::Number(1),
            Self::Number(2),
            Self::Number(3),
            Self::Number(4),
            Self::Number(5),
            Self::Number(6),
            Self::Number(7),
            Self::Number(8),
            Self::Number(9),
            Self::Activate,
        ]
    }
}

fn get_all_moves<T: EnumAll + Hash + Eq + PartialEq + Sized + KeypadKey + Copy>(
) -> HashMap<T, Vec<(T, PushInsn)>> {
    let mut res = HashMap::new();
    let get_dirs = |from: Position| {
        T::get_all()
            .iter()
            .map(|k| (k, k.get_coordinates()))
            .map(|(k, (r, c))| {
                (
                    *k,
                    PushInsn::try_from((r as i32 - from.0 as i32, c as i32 - from.1 as i32)),
                )
            })
            .filter(|(_, x)| x.is_ok())
            .map(|(k, x)| (k, x.unwrap()))
            .collect::<Vec<_>>()
    };
    for k in T::get_all() {
        res.insert(k, get_dirs(k.get_coordinates()));
    }
    res
}

fn get_all_paths_impl<T: Eq + PartialEq + Hash + Clone + Copy>(
    from: T,
    to: T,
    visited: &mut HashSet<T>,
    moves: &HashMap<T, Vec<(T, PushInsn)>>,
    acc_now: &mut PushSeq,
    acc_all: &mut Vec<PushSeq>,
) {
    if from == to {
        let mut to_add = acc_now.clone();
        to_add.push(PushInsn::A);
        acc_all.push(to_add);
        return;
    } else if visited.contains(&from) {
        return;
    }
    visited.insert(from);

    for (next, dir) in moves.get(&from).unwrap() {
        acc_now.push(*dir);
        get_all_paths_impl(*next, to, visited, moves, acc_now, acc_all);
        assert!(acc_now.pop().unwrap() == *dir);
    }
    visited.remove(&from);
}

type PushSeq = Vec<PushInsn>;

fn get_all_paths<
    T: EnumAll + Eq + PartialEq + Hash + std::fmt::Debug + Clone + Copy + KeypadKey,
>() -> HashMap<(T, T), Vec<PushSeq>> {
    let mut res = HashMap::new();
    let succ = get_all_moves();
    for source in T::get_all() {
        for target in T::get_all() {
            let mut acc = vec![];
            get_all_paths_impl(
                source,
                target,
                &mut HashSet::new(),
                &succ,
                &mut vec![],
                &mut acc,
            );
            res.insert((source, target), acc);
        }
    }

    res
}

impl From<&PushInsn> for DirectionalKey {
    fn from(val: &PushInsn) -> Self {
        match val {
            PushInsn::L => DirectionalKey::ArrowL,
            PushInsn::R => DirectionalKey::ArrowR,
            PushInsn::U => DirectionalKey::ArrowU,
            PushInsn::D => DirectionalKey::ArrowD,
            PushInsn::A => DirectionalKey::Activate,
        }
    }
}

type NumPair = (NumericKey, NumericKey);
type DirPair = (DirectionalKey, DirectionalKey);

fn expand_paths(
    cand: &Vec<PushInsn>,
    paths_dir: &HashMap<DirPair, Vec<PushSeq>>,
    current_min: Option<usize>,
) -> Vec<Vec<PushInsn>> {
    let mut res: Vec<Vec<PushInsn>> = vec![vec![]];
    let mut from_loc = DirectionalKey::Activate;
    for x in cand {
        let mut new_res = Vec::with_capacity(256);
        for expansion in paths_dir.get(&(from_loc, x.into())).unwrap() {
            for r in res.iter_mut() {
                if let Some(min) = current_min {
                    if r.len() + expansion.len() >= min {
                        continue;
                    }
                }
                let mut c = r.clone();
                c.reserve(expansion.len());
                c.extend(expansion.iter());
                new_res.push(c);
            }
        }
        std::mem::swap(&mut res, &mut new_res);
        from_loc = x.into();
    }
    res
}

fn get_shortest_paths_source_target(
    from: NumericKey,
    target: NumericKey,
    paths_num: &HashMap<NumPair, Vec<PushSeq>>,
    paths_dir: &HashMap<DirPair, Vec<PushSeq>>,
) -> HashMap<NumPair, PushSeq> {
    let mut res = HashMap::new();

    let mut set = false;
    let mut min_path = vec![];
    // cand consists of all possible paths (from, target)
    for (ci, cand) in paths_num.get(&(from, target)).unwrap().iter().enumerate() {
        let min = if set { Some(min_path.len()) } else { None };
        // we try to expand each move by each move on a keypad
        for (i, cand2) in expand_paths(cand, paths_dir, min).iter().enumerate() {
            // for those moves we expand again
            let min = if set { Some(min_path.len()) } else { None };
            let tmp = expand_paths(cand2, paths_dir, min);
            // those are the final paths, get nonempty shortest paths
            if let Some(p) = tmp
                .iter()
                .filter(|v| !v.is_empty())
                .min_by(|a, b| a.len().cmp(&b.len()))
            {
                // and register it
                if !set || p.len() < min_path.len() {
                    min_path = p.clone();
                    set = true;
                    // println!("2 {:?}", p);
                }
            }
        }
    }
    if !min_path.is_empty() {
        res.insert((from, target), min_path);
    }
    res
}

fn get_shortest_paths_for_steps(
    steps: Vec<NumericKey>,
    paths_num: &HashMap<NumPair, Vec<PushSeq>>,
    paths_dir: &HashMap<DirPair, Vec<PushSeq>>,
) -> HashMap<NumPair, PushSeq> {
    let mut res = HashMap::new();

    let mut start = NumericKey::Activate;
    let maps = steps
        .iter()
        .map(|v| {
            let rv = (start, v);
            start = *v;
            rv
        })
        .par_bridge()
        .map(|(start, target)| {
            get_shortest_paths_source_target(start, *target, paths_num, paths_dir)
        })
        .collect::<Vec<_>>();
    for map in maps {
        for (k, v) in map {
            res.insert(k, v);
        }
    }

    res
}

// const TARGETS : [(u64, [NumericKey; 4]); 5] = [
// (29, [NumericKey::Number(0), NumericKey::Number(2), NumericKey::Number(9), NumericKey::Activate]),
// (980, [NumericKey::Number(9), NumericKey::Number(8), NumericKey::Number(0), NumericKey::Activate]),
// (179, [NumericKey::Number(1), NumericKey::Number(7), NumericKey::Number(9), NumericKey::Activate]),
// (456, [NumericKey::Number(4), NumericKey::Number(5), NumericKey::Number(6), NumericKey::Activate]),
// (379, [NumericKey::Number(3), NumericKey::Number(7), NumericKey::Number(9), NumericKey::Activate]),
// ];

const TARGETS: [(u64, [NumericKey; 4]); 5] = [
    (
        382,
        [
            NumericKey::Number(3),
            NumericKey::Number(8),
            NumericKey::Number(2),
            NumericKey::Activate,
        ],
    ),
    (
        463,
        [
            NumericKey::Number(4),
            NumericKey::Number(6),
            NumericKey::Number(3),
            NumericKey::Activate,
        ],
    ),
    (
        935,
        [
            NumericKey::Number(9),
            NumericKey::Number(3),
            NumericKey::Number(5),
            NumericKey::Activate,
        ],
    ),
    (
        279,
        [
            NumericKey::Number(2),
            NumericKey::Number(7),
            NumericKey::Number(9),
            NumericKey::Activate,
        ],
    ),
    (
        480,
        [
            NumericKey::Number(4),
            NumericKey::Number(8),
            NumericKey::Number(0),
            NumericKey::Activate,
        ],
    ),
];

pub fn part_one(_: &str) -> Option<usize> {
    // not bothered with parsing xd
    let results = TARGETS
        .par_iter()
        .map(|(val, target)| {
            let dir_par_insns = get_all_paths::<DirectionalKey>();
            let num_pad_insns = get_all_paths::<NumericKey>();
            let paths =
                get_shortest_paths_for_steps(target.to_vec(), &num_pad_insns, &dir_par_insns);
            println!("\nAct - Num: {:?}", paths);
            let mut res = vec![];
            let mut start = NumericKey::Activate;
            for t in target {
                for v in paths.get(&(start, *t)).unwrap() {
                    res.push(v);
                }
                start = *t;
            }
            (val, res.iter().map(|v| **v).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (val, res) in results {
        sum += res.len() * *val as usize;
        println!(
            "Num {val}, len {} = {}",
            res.len(),
            res.len() * *val as usize
        );
    }

    Some(sum)
}

type InsnPair = (PushInsn, PushInsn);

fn get_shortest_paths_for_steps_in_depth(
    steps: Vec<NumericKey>,
    paths_num: &HashMap<NumPair, Vec<PushSeq>>,
    paths_dir: &HashMap<DirPair, Vec<PushSeq>>,
    cache: &mut HashMap<(InsnPair, u64), u64>,
    robots: u64,
) -> u64 {
    let mut start = NumericKey::Activate;
    let paths = steps
        .iter()
        .map(|v| {
            let rv = paths_num.get(&(start, *v)).unwrap();
            start = *v;
            rv
        })
        .collect::<Vec<_>>();

    let mut res = 0;

    for number_paths in paths {
        let mut best_path = 0;
        for path in number_paths {
            let mut start = PushInsn::A;
            let mut path_score = 0;
            for target in path {
                path_score += get_shortest_paths_for_steps_in_depth_robot(
                    &(start, *target),
                    paths_dir,
                    cache,
                    robots - 1,
                );
                start = *target;
            }
            if best_path == 0 || best_path > path_score {
                best_path = path_score;
            }
        }
        res += best_path;
    }

    res
}

// works by counting nr of characters each pair of transitions (L, R, U, D, A)
// expands to in lower layers (using cache because some pairs expand to the same
// sequences)
fn get_shortest_paths_for_steps_in_depth_robot(
    (from, to): &InsnPair,
    paths_dir: &HashMap<DirPair, Vec<PushSeq>>,
    cache: &mut HashMap<(InsnPair, u64), u64>,
    robots: u64,
) -> u64 {
    let mut best_path = 0;
    for path in paths_dir.get(&(from.into(), to.into())).unwrap() {
        let mut start = PushInsn::A;
        let mut path_score = 0u64;
        if robots == 0 {
            // skip path score calculation (I am pushign buttons)
            path_score = path.len() as u64;
        } else {
            for target in path {
                let cache_key = ((start, *target), robots);
                if cache.contains_key(&cache_key) {
                    path_score += cache.get(&cache_key).unwrap();
                } else {
                    let res = get_shortest_paths_for_steps_in_depth_robot(
                        &(start, *target),
                        paths_dir,
                        cache,
                        robots - 1,
                    );
                    path_score += res;
                    cache.insert(cache_key, res);
                }
                start = *target;
            }
        }
        if best_path == 0 || best_path > path_score {
            best_path = path_score;
        }
    }
    best_path
}

pub fn part_two(_: &str) -> Option<u64> {
    // needs something more clever :)
    // - haha turns out i made correct observations but skipped and
    //   forgot about them
    // - also, my representations in p1 also didnt help

    let mut cache = HashMap::new();
    let results = TARGETS
        .iter()
        .map(|(val, target)| {
            let dir_par_insns = get_all_paths::<DirectionalKey>();
            let num_pad_insns = get_all_paths::<NumericKey>();
            (
                val,
                get_shortest_paths_for_steps_in_depth(
                    target.to_vec(),
                    &num_pad_insns,
                    &dir_par_insns,
                    &mut cache,
                    25,
                ),
            )
        })
        .collect::<Vec<_>>();
    let sum = results.iter().map(|a| a.0 * a.1).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
