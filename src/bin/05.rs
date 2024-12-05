use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let lines = input.split_ascii_whitespace().filter(|v| !v.is_empty());

    let mut r1 = vec![];
    let mut r2 = vec![];
    for l in lines {
        if l.contains("|") {
            let parsed = l
                .split("|")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            r1.push((parsed[0], parsed[1]));
        } else if l.contains(",") {
            let parsed = l
                .split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            r2.push(parsed);
        }
    }
    (r1, r2)
}
fn get_position_map(v: &[u32]) -> HashMap<u32, usize> {
    let mut m = HashMap::new();
    v.iter().enumerate().for_each(|(i, v)| {
        m.insert(*v, i);
    });
    m
}

pub fn get_position_maps(inp: &[Vec<u32>]) -> Vec<HashMap<u32, usize>> {
    inp.iter()
        .map(|v| get_position_map(v))
        .collect::<Vec<HashMap<u32, usize>>>()
}

// key must be before value
pub fn get_requirement_map(inp: &[(u32, u32)]) -> HashMap<u32, Vec<u32>> {
    let mut m: HashMap<u32, Vec<u32>> = HashMap::new();
    for (u, v) in inp {
        if let Some(vec) = m.get_mut(u) {
            vec.push(*v);
        } else {
            m.insert(*u, vec![*v]);
        }
    }
    m
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pairs, list) = parse(input);
    let position_maps = get_position_maps(&list);
    let requirements = get_requirement_map(&pairs);
    let mut mid_sum = 0;

    for (updates, position_map) in list.iter().zip(position_maps.iter()) {
        let correct = satisfied(updates, &requirements, position_map);
        if correct {
            mid_sum += updates[(updates.len() - 1) / 2];
        }
    }

    Some(mid_sum)
}

pub fn satisfied(
    updates: &[u32],
    requirements: &HashMap<u32, Vec<u32>>,
    position_map: &HashMap<u32, usize>,
) -> bool {
    let mut correct = true;
    for elem in updates {
        let elem_idx = position_map.get(elem).unwrap();
        if !requirements.contains_key(elem) {
            continue;
        }

        for requirement in requirements.get(elem).unwrap() {
            if let Some(req_idx) = position_map.get(requirement) {
                if elem_idx >= req_idx {
                    correct = false;
                    break;
                }
            }
        }

        if !correct {
            break;
        }
    }
    correct
}

fn is_pure_satisfied(
    candidate: u32,
    update: &[u32],
    requirements: &HashMap<u32, Vec<u32>>,
) -> bool {
    let reqs = requirements.get(&candidate);
    if let Some(v) = reqs {
        v.iter().all(|n| !update.contains(n))
    } else {
        true
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (pairs, list) = parse(input);
    let position_maps = get_position_maps(&list);
    let requirements = get_requirement_map(&pairs);
    let mut reorders = vec![];
    // use the algo from p1 to get incorrectly-ordered stuff
    for (updates, position_map) in list.iter().zip(position_maps.iter()) {
        let correct = satisfied(updates, &requirements, position_map);
        if !correct {
            reorders.push((updates.clone(), position_map.clone()));
        }
    }

    let mut mid_sum = 0;
    // construct the correct order by going from the least constrained
    // to the most
    // due to the proeprties of the ordering, there should always be
    // a "purely satisfiable value" to add to the update sequence
    //  - number that has none of its "requirements" in the "update" candidates
    //  - e.g. in s = [75,97,47,61,53], 53 is satisfiable because its
    //    requirements are [13, 29], none of which are in the list s
    //  - then remove that satisfiable number (53) from all requirement lists of
    //    all of the remaining numbers (75,97,47,61) and repeat until there is
    //    just one left (trivial)
    for (updates, _) in reorders.iter_mut() {
        let mut updates_new: Vec<u32> = vec![];
        let mut requirements_glob = requirements.clone();
        loop {
            if updates.len() == 1 {
                break;
            }
            let reqs_clone = requirements_glob.clone();
            let updates_clone = updates.clone();
            let mut progress = false;
            for ok_val in updates_clone
                .iter()
                .filter(|v| is_pure_satisfied(**v, &updates_clone, &reqs_clone))
            {
                progress = true;
                updates_new.insert(0, *ok_val);
                for (_, v) in requirements_glob.iter_mut() {
                    if let Some(idx) = v.iter().position(|x| x == ok_val) {
                        v.remove(idx);

                        if let Some(idx2) = updates.iter().position(|x| x == ok_val) {
                            updates.remove(idx2);
                        }
                    }
                }
            }
            if !progress {
                panic!("infinite loop!")
            }
        }
        for newup in updates_new {
            updates.push(newup);
        }
        mid_sum += updates[(updates.len() - 1) / 2];
    }

    Some(mid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
