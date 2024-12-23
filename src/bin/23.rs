use std::collections::{HashMap, HashSet};

use rayon::iter::{ParallelBridge, ParallelIterator};

advent_of_code::solution!(23);

type SuccMapVec<'a> = HashMap<&'a str, Vec<&'a str>>;
type SuccMapSet<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn get_successors<'a>(input: &'a str) -> (SuccMapVec<'a>, SuccMapSet<'a>) {
    let mut result = HashMap::new();
    let mut result2 = HashMap::new();
    for line in input.split_ascii_whitespace() {
        let split = line.split('-').collect::<Vec<_>>();
        assert!(split.len() == 2);
        let v1 = split[0];
        let v2 = split[1];
        result
            .entry(v1)
            .and_modify(|vec: &mut Vec<&'a str>| vec.push(v2))
            .or_insert(vec![&v2]);
        result
            .entry(v2)
            .and_modify(|vec: &mut Vec<&'a str>| vec.push(v1))
            .or_insert(vec![&v1]);
        let mut tmp = HashSet::new();
        tmp.insert(v2);
        result2
            .entry(v1)
            .and_modify(|hs: &mut HashSet<&'a str>| {
                hs.insert(v2);
            })
            .or_insert(tmp.clone());
        tmp.remove(v2);
        tmp.insert(v1);
        result2
            .entry(v2)
            .and_modify(|hs: &mut HashSet<&'a str>| {
                hs.insert(v1);
            })
            .or_insert(tmp);
    }

    (result, result2)
}

fn get_sorted_component(things: &mut [&str]) -> String {
    things.sort();
    things.join(",")
}

pub fn part_one(input: &str) -> Option<usize> {
    let cache: (usize, HashSet<String>) = (0, HashSet::new());

    let (successors, successors_hash) = get_successors(input);
    let res = find_clique_count_n(true, 3, &successors, &successors_hash, &cache);
    Some(res.len())
}

fn find_clique_count_n<'a>(
    check_start_t: bool,
    n: usize,
    successors: &SuccMapVec<'a>,
    succ_hash: &SuccMapSet<'a>,
    helper: &(usize, HashSet<String>),
) -> HashSet<String> {
    let mut sorted_components_w_t: HashSet<String> = HashSet::new();
    for node in successors.keys() {
        let mut st = HashSet::new();
        st.insert(node.to_owned());
        find_clique_count(
            check_start_t,
            node,
            successors,
            succ_hash,
            &[node],
            &mut st,
            n - 1,
            &mut sorted_components_w_t,
            helper,
        );
    }
    sorted_components_w_t
}

/**
 * check_start_t = are we checking for the t start (part 1)
 * node = current node we are expanding
 * successors, succ_hash = successor map (values are vectors/hashsets)
 * nodes_sofar = list of nodes that form smaller clique
 * nodes_sofar_hash = the same as above, just a hashset (TODO)
 * n = target size
 * acc = accumulator of final cliques
 * cache = cache of previous results (usize is the size of the cliques in the set)
 * */
fn find_clique_count<'a>(
    check_start_t: bool,
    node: &'a str,
    successors: &SuccMapVec<'a>,
    succ_hash: &SuccMapSet<'a>,
    nodes_sofar: &[&'a str],
    nodes_sofar_hash: &mut HashSet<&'a str>,
    n: usize,
    acc: &mut HashSet<String>,
    cache: &'a (usize, HashSet<String>),
) {
    if cache.0 == n {
        // we rely on cache on the bottom-most n layers
        let rest_of_nodes = nodes_sofar;
        for clique in cache.1.iter() {
            // for each clique, check if rest of nodes makes clique
            if rest_of_nodes.iter().all(|rest_node| {
                !clique.contains(rest_node)
                    && clique
                        .split(',')
                        .all(|part| succ_hash.get(rest_node).unwrap().contains(part))
            }) {
                // if they do, add them into the accumulator
                let mut split = clique.split(',').collect::<Vec<_>>();
                split.extend_from_slice(rest_of_nodes);
                if !check_start_t || split.iter().any(|v| v.starts_with('t')) {
                    let v = get_sorted_component(&mut split);
                    acc.insert(v);
                }
            }
        }
        return;
    }

    let succ = successors.get(node).unwrap();
    // otherwise just iterate ove nodes successors
    for upcoming in succ.iter() {
        // check if the node is unique in the smaller-clique set
        // and that the node is a valid successor of all small-clique nodes
        if nodes_sofar_hash.contains(upcoming)
            || !nodes_sofar
                .iter()
                .all(|part| succ_hash.get(upcoming).unwrap().contains(part))
        {
            continue;
        }
        let mut new_clique = [nodes_sofar, &[upcoming]].concat();

        if n == 1 {
            // ending, clique complete, register it
            if !check_start_t || new_clique.iter().any(|v| v.starts_with('t')) {
                let v = get_sorted_component(&mut new_clique);
                acc.insert(v);
            }
        } else {
            // n > 1 -> we need to enlarge this small-clique (new_sl)
            nodes_sofar_hash.insert(upcoming);
            find_clique_count(
                check_start_t,
                upcoming,
                successors,
                succ_hash,
                &new_clique,
                nodes_sofar_hash,
                n - 1,
                acc,
                cache,
            );
            nodes_sofar_hash.remove(upcoming);
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let (successors, successors_hash) = get_successors(input);
    let mut clique_size = 3;
    let mut previous_cache = (0, HashSet::new());

    let res = loop {
        println!("try size {}", clique_size);
        // BEGIN cutting (ignore this)
        // - this secttion attempts to cut the nodes accoring to the required clique size
        // (it does nothign because the input nodes all have same amount of neighbours)
        let filtered = successors
            .iter()
            .filter(|(_, v)| v.len() >= clique_size - 1);
        let vertices = filtered.clone().map(|v| v.0).collect::<Vec<_>>();
        let mut successors = HashMap::new();
        for v in filtered.clone().collect::<Vec<_>>() {
            successors.insert(
                *v.0,
                v.1.clone()
                    .iter()
                    .cloned()
                    .filter(|v| vertices.contains(&v))
                    .collect::<Vec<_>>(),
            );
        }
        let filtered = successors_hash
            .iter()
            .filter(|(_, v)| v.len() >= clique_size - 1);
        let mut successors_hash = HashMap::new();
        for v in filtered.clone().collect::<Vec<_>>() {
            successors_hash.insert(
                *v.0,
                HashSet::from_iter(
                    v.1.clone()
                        .iter()
                        .cloned()
                        .filter(|v| vertices.contains(&v)),
                ),
            );
        }
        // END cutting

        if successors.keys().count() < clique_size {
            println!(
                "Insufficient sizes {} for {}",
                successors.keys().count(),
                clique_size
            );
            break None;
        }

        let mut map_res: HashSet<String> = HashSet::new();
        for v in successors
            .keys()
            .par_bridge()
            .map(|node| {
                let mut st = HashSet::new();
                st.insert(node.to_owned());
                let mut sorted_components_w_t: HashSet<String> = HashSet::new();
                find_clique_count(
                    false,
                    node,
                    &successors,
                    &successors_hash,
                    &[node],
                    &mut st,
                    clique_size - 1,
                    &mut sorted_components_w_t,
                    &previous_cache,
                );
                sorted_components_w_t
            })
            .collect::<Vec<_>>()
        {
            for val in v {
                map_res.insert(val);
            }
        }
        if map_res.len() == 1 {
            break Some(map_res);
        } else {
            println!("{:?}", map_res.iter().max_by(|a, b| a.len().cmp(&b.len())));
            println!("Count: {}", map_res.len());
            previous_cache = (clique_size, map_res);
        }
        clique_size += 1;
    };
    res.map(|v| {
        if v.len() == 1 {
            v.iter().next().unwrap().clone()
        } else {
            "NONE".to_owned()
        }
    })
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
