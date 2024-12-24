use std::collections::{HashMap, HashSet};

use pathfinding::num_traits::pow;
use rayon::iter::{ParallelBridge, ParallelIterator};

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy)]
enum Operator<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

trait Calculate {
    fn get_dependencies(&self) -> Vec<&str>;

    fn calculate(&self, dependencies: &[u8]) -> u8;
}

impl<'a> Calculate for Operator<'a> {
    fn get_dependencies(&self) -> Vec<&'a str> {
        match self {
            Operator::And(a, b) => vec![a, b],
            Operator::Or(a, b) => vec![a, b],
            Operator::Xor(a, b) => vec![a, b],
        }
    }

    // could be done better -> hashmap str -> operator with
    // Constant(u8) as an operator
    fn calculate(&self, dependencies: &[u8]) -> u8 {
        if dependencies.len() != 2 {
            panic!("invalid operand count");
        }
        let left_op = dependencies[0];
        let right_op = dependencies[1];

        match self {
            Operator::And(_, _) => left_op & right_op,
            Operator::Or(_, _) => left_op | right_op,
            Operator::Xor(_, _) => left_op ^ right_op,
        }
    }
}

impl<'a> From<&'a str> for Operator<'a> {
    fn from(value: &'a str) -> Self {
        let v = value.trim().split(" ").collect::<Vec<_>>();
        assert!(v.len() == 3);
        let operand_l = v[0];
        let operand_r = v[2];
        let operator = v[1];
        match operator {
            "XOR" => Self::Xor(operand_l, operand_r),
            "OR" => Self::Or(operand_l, operand_r),
            "AND" => Self::And(operand_l, operand_r),
            _ => panic!("unknown operator {}", operator),
        }
    }
}

fn parse(input: &str) -> (HashMap<&str, u8>, Vec<(Operator, &str)>) {
    let mut initial_value_map = HashMap::new();
    let lines = input.split("\n").collect::<Vec<_>>();
    for l in lines.iter().take_while(|v| !v.is_empty()) {
        let split = l.split(": ").collect::<Vec<_>>();
        assert!(split.len() == 2);
        initial_value_map.insert(split[0], split[1].parse::<u8>().unwrap());
    }

    let mut operators = vec![];
    for l in lines
        .iter()
        .skip_while(|v| !v.is_empty())
        .skip_while(|v| v.is_empty())
    {
        let split = l.split(" -> ").collect::<Vec<_>>();
        assert!(split.len() == 2);
        operators.push((Operator::from(split[0]), split[1]));
    }

    (initial_value_map, operators)
}

fn calculate<'a, 'b>(
    target: &'a str,
    formulae: &'b HashMap<&str, Operator<'_>>,
    values: &mut HashMap<&'a str, u8>,
    set: &mut HashSet<String>,
) where
    'b: 'a,
{
    if set.contains(target) || values.contains_key(target) {
        return;
    }
    set.insert(target.to_owned());

    if let Some(formula) = formulae.get(target) {
        let deps = formula.get_dependencies();
        for dep in deps.iter() {
            if values.contains_key(dep) {
                continue;
            }
            calculate(dep, formulae, values, set);
        }
        if deps.iter().all(|v| values.get(v).is_some()) {
            let vals = deps
                .iter()
                .map(|v| *values.get(v).unwrap())
                .collect::<Vec<_>>();
            values.insert(target, formula.calculate(&vals));
        }
    }
    set.remove(&target.to_owned());
}

pub fn part_one<'a>(input: &'a str) -> Option<u64> {
    let (mut values, formulae) = parse(input);
    let formula_map: HashMap<&'a str, Operator<'a>> =
        HashMap::from_iter(formulae.iter().map(|(form, res_name)| (*res_name, *form)));

    // println!("Values: {:?}\nForms: {:?}", values, formula_map);
    let mut z_vals = formula_map
        .keys()
        .cloned()
        .filter(|k| k.starts_with('z'))
        .collect::<Vec<_>>();
    for z in &z_vals {
        let mut x = HashSet::new();
        calculate(z, &formula_map, &mut values, &mut x);
    }
    z_vals.sort();
    Some(
        z_vals
            .iter()
            .map(|z| values.get(z).unwrap())
            .rev()
            .fold(0u64, |prev, next| prev * 2 + *next as u64),
    )
}

fn pairs<'a>(uni: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    let mut res = vec![];
    for i in 0..uni.len() {
        for j in i + 1..uni.len() {
            res.push((uni[i], uni[j]));
        }
    }
    res
}

fn swapped_wires<'a>(pairs: &[(&'a str, &'a str)], formulae: &mut HashMap<&'a str, Operator<'a>>) {
    for (w1, w2) in pairs {
        let w2val = formulae.remove(w2).unwrap();
        let w1val = formulae.remove(w1).unwrap();
        formulae.insert(w2, w1val);
        formulae.insert(w1, w2val);
    }
}

pub fn part_two<'a>(input: &'a str) -> Option<u32> {
    let (values, formulae) = parse(input);
    let formula_map: HashMap<&'a str, Operator<'a>> =
        HashMap::from_iter(formulae.iter().map(|(form, res_name)| (*res_name, *form)));
    let pairs = pairs(&formula_map.keys().cloned().collect::<Vec<_>>());

    fn get_vals_starting_with<'b>(map: &HashMap<&'b str, Operator<'b>>, c: &str) -> Vec<&'b str> {
        map.keys()
            .cloned()
            .filter(|k| k.starts_with(c))
            .collect::<Vec<_>>()
    }

    fn get_vals_startin_with_v<'b>(map: &HashMap<&'b str, u8>, c: &str) -> Vec<&'b str> {
        map.keys()
            .cloned()
            .filter(|k| k.starts_with(c))
            .collect::<Vec<_>>()
    }
    fn get_num(vals: &mut [&str], vals_map: &HashMap<&str, u8>) -> Option<u64> {
        vals.sort();
        if vals.iter().all(|z| vals_map.get(z).is_some()) {
            Some(
                vals.iter()
                    .map(|z| vals_map.get(z).unwrap())
                    .rev()
                    .fold(0u64, |prev, next| prev * 2 + *next as u64),
            )
        } else {
            None
        }
    }
    let target = get_num(&mut get_vals_startin_with_v(&values, "x"), &values).unwrap()
        + get_num(&mut get_vals_startin_with_v(&values, "y"), &values).unwrap();
    println!("Target: {target}");
    // yea this wont work... still worth a try? xdd
    pairs.iter().enumerate().par_bridge().for_each(|(i, _)| {
        let mut my_formulae = formula_map.clone();
        for j in i + 1..pairs.len() {
            if j % 10 == 0 {
                println!("{i}: {j}/{}", pairs.len());
            }
            for k in j + 1..pairs.len() {
                'reset: for l in k + 1..pairs.len() {
                    let col = [pairs[i], pairs[j], pairs[k], pairs[l]];
                    swapped_wires(&col, &mut my_formulae);
                    let mut my_values = values.clone();
                    let mut z_vals = get_vals_starting_with(&my_formulae, "z");
                    z_vals.sort();
                    for (pow, z) in z_vals.iter().enumerate() {
                        let mut set = HashSet::new();
                        calculate(z, &my_formulae, &mut my_values, &mut set);
                        // checks if the corresponding bit is correct, if not, we end iteration early
                        if !my_values.contains_key(z)
                            || *my_values.get(z).unwrap()
                                != if target & (1 << pow) == 0 { 0 } else { 1 }
                        {
                            break 'reset;
                        }
                    }
                    if let Some(v) = get_num(&mut z_vals, &my_values) {
                        if v == target {
                            println!("RESULT {:?}", pairs);
                        }
                    }
                    swapped_wires(&col, &mut my_formulae);
                }
            }
        }
    });
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
