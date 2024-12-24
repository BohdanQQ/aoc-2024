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

fn parse(input: &str) -> (HashMap<String, u8>, Vec<(Operator, &str)>) {
    let mut initial_value_map = HashMap::new();
    let lines = input.split("\n").collect::<Vec<_>>();
    for l in lines.iter().take_while(|v| !v.is_empty()) {
        let split = l.split(": ").collect::<Vec<_>>();
        assert!(split.len() == 2);
        initial_value_map.insert(split[0].to_owned(), split[1].parse::<u8>().unwrap());
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
    values: &mut HashMap<String, u8>,
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
            if values.contains_key(dep.to_owned()) {
                continue;
            }
            calculate(dep, formulae, values, set);
        }
        if deps.iter().all(|v| values.get(v.to_owned()).is_some()) {
            let vals = deps
                .iter()
                .map(|v| *values.get(v.to_owned()).unwrap())
                .collect::<Vec<_>>();
            values.insert(target.to_owned(), formula.calculate(&vals));
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
            .map(|z| values.get(z.to_owned()).unwrap())
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

fn swapped_wires<'a>(
    pairs: &[(&'a str, &'a str)],
    formulae: &HashMap<&'a str, Operator<'a>>,
) -> HashMap<&'a str, Operator<'a>> {
    let mut res = formulae.clone();
    for (w1, w2) in pairs {
        let w2val = formulae.get(w2).unwrap();
        let w1val = formulae.get(w1).unwrap();
        res.insert(w1, *w2val);
        res.insert(w2, *w1val);
    }
    res
}

pub fn part_two<'a>(input: &'a str) -> Option<u32> {
    // solution mps z25 vcv z13 vwp  z19 vjv cqm
    // the first two are mismatch on XOR gates for x y -> z
    // the other two are mismatches in the combination gates
    // i found this by visualizing and sorting the chart (mermaid tool)
    //
    // to help me find them properly I compared results for A + 1 = R
    // A = 2**i i = 0 ..46 and compared the results with R
    // if they didnt match, there was a problem in the neighbourhood of i-1, i, i+1
    // which concentrates you towards the answer, here lies my battleground
    // for this task which does not even work :) XD (for the last, correctly replaced pair, one)
    // instance overflows i32 which gets casted badly i guess)
    // for non +1 test cases you can also jus try R - (result) and look at the
    // near bits from this subtraction

    /*
    maermaid chart i created (could be better, representing the gates as nodes instead):
    flowchart
    x03 -->|AND| fkm
    y03 -->|AND| fkm
    x03 -->|XOR| htb
    y03 -->|XOR| htb
    x04 -->|AND| btd
    y04 -->|AND| btd
    ...
    x44...
    mqs -->|XOR| z01
    mpf -->|XOR| z01

    overall this was very mechanical and fits with the proposed solution based on pattern matching
    (all of those however rely on the structure of the bit-adder without any redundant gates, etc.)

    another solution outlined in the
    // checks if the corresponding bit is correct, if not, we end iteration early
    comment is to check evaluation of a single bit and only proceed if that bit is correct

    another solution technique would construct a graph and look for cycles
    or irregularities in the evaluation of individual bits z02 - z44.

    also, one of the previous commits had an optimized swapping function (swapped_wires)
    which didn't clone the formula map, it just replaced the wires since
    the assignment says they can't overlap.

    I rolled it back because something broke down elsewhere and I found it easier to go back to the cloning
    - more pure - implementation (it made the searching much faster tho)
     */

    return None;
    let (values, formulae) = parse(input);
    let formula_map: HashMap<&'a str, Operator<'a>> =
        HashMap::from_iter(formulae.iter().map(|(form, res_name)| (*res_name, *form)));
    let pairs = pairs(&formula_map.keys().cloned().collect::<Vec<_>>());

    fn get_vals_starting_with<'b>(map: &HashMap<&'b str, Operator<'b>>, c: &str) -> Vec<String> {
        map.keys()
            .cloned()
            .filter(|k| k.starts_with(c))
            .map(|v| v.to_owned())
            .collect::<Vec<_>>()
    }

    fn get_vals_startin_with_v(map: &HashMap<String, u8>, c: &str) -> Vec<String> {
        map.keys()
            .cloned()
            .filter(|k| k.starts_with(c))
            .collect::<Vec<_>>()
    }
    fn get_num(vals: &mut [String], vals_map: &HashMap<String, u8>) -> Option<u64> {
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

    fn check_pair(pair: &(&str, &str), pairs: &[(&str, &str)]) -> bool {
        for p in pairs {
            if p.0 == pair.0 || p.0 == pair.1 || p.1 == pair.0 || p.1 == pair.1 {
                return false;
            }
        }
        true
    }

    let in1 = get_num(&mut get_vals_startin_with_v(&values, "x"), &values).unwrap();
    let in2 = get_num(&mut get_vals_startin_with_v(&values, "y"), &values).unwrap();
    let target = in1 + in2;
    println!("1: {in1}");
    println!("2: {in2}");
    println!("Target: {target}");
    // yea this wont work... still worth a try? xdd
    // still not working xdd
    //
    // there is a clear solution - pattern matching of a bit-adder from the bottom up
    //
    // we match z00 = x00 XOR y00
    // then z01 = CARRY XOR BOTTOM
    // and look up CARRY as x00 AND y00 and BOTTOM as x01 XOR y01
    // then compare what matched with CARRY and BOTTOM with what z01 matched
    // no match -> found swap
    // match -> no swap, go onto next (z02 - except the AND lookup will look
    // for A OR B indicating the carry from previous steps (matched vars)) -> induction -> done
    // it could also be done by hand lol
    // it's gonna be a pain though, so yea, fuck this part :)
    pairs.iter().enumerate().par_bridge().for_each(|(i, _)| {
        // 'reset: for j in i + 1..pairs.len() {
        //     if !check_pair(&pairs[i], &[pairs[j]]) {
        //         // println!("check {:?}", col);
        //         continue;
        //     }
        let col = &[];
        let my_formulae = swapped_wires(col, &formula_map);
        let mut z_vals = get_vals_starting_with(&my_formulae, "z");
        z_vals.sort();
        let mut all = true;
        let mut tries = (0..45)
            .map(|i| {
                (
                    pow::pow(2, i) as u64,
                    1 as u64,
                    pow::pow(2, i) as u64 + 1 as u64,
                )
            })
            .collect::<Vec<_>>();
        tries.push((in1, in2, target));
        if i % 100 == 0 {
            println!("{i}");
        }
        for (one, two, target) in tries {
            let mut my_values: HashMap<String, u8> = HashMap::new();
            for i in (0..46) {
                my_values.insert(
                    ('x'.to_string() + &format!("{:02}", i)),
                    if one & (1 << i) == 0 { 0u8 } else { 1u8 },
                );
                my_values.insert(
                    ('y'.to_string() + &format!("{:02}", i)),
                    if two & (1 << i) == 0 { 0u8 } else { 1u8 },
                );
            }
            for (pow, z) in z_vals.iter().enumerate() {
                let mut set = HashSet::new();
                calculate(z, &my_formulae, &mut my_values, &mut set);
                // checks if the corresponding bit is correct, if not, we end iteration early
                // if !my_values.contains_key(z)
                //     || *my_values.get(z).unwrap()
                //         != if target & (1 << pow) == 0 { 0 } else { 1 }
                // {
                //     break 'reset;
                // }
            }
            if let Some(v) = get_num(&mut z_vals, &my_values) {
                if v == target {
                    // println!("RESULT {:?}", col);
                } else {
                    all = false;
                    // println!("fail: {:?} {one} {two} {target}", v)
                }
            } else {
                all = false;
            }
        }
        if all {
            println!("RESULT {:?}", col);
        }
        // }
    });
    // for testing example input
    // println!("AND Target: {}", in1 & in2);
    // let target = in1 & in2;
    // pairs.iter().enumerate().par_bridge().for_each(|(i, _)| {
    //     let mut my_formulae = formula_map.clone();
    //     'reset: for j in i + 1..pairs.len() {
    //         if pairs[i].0 == pairs[j].0 || pairs[i].0 == pairs[j].1 || pairs[i].1 == pairs[j].0 || pairs[i].1 == pairs[j].1 {
    //             continue;
    //         }
    //         let col = [pairs[i], pairs[j]];
    //         // println!("C: {:?}", col);
    //         swapped_wires(&col, &mut my_formulae);
    //         let mut my_values = values.clone();
    //         let mut z_vals = get_vals_starting_with(&my_formulae, "z");
    //         z_vals.sort();
    //         for (pow, z) in z_vals.iter().enumerate() {
    //             let mut set = HashSet::new();
    //             calculate(z, &my_formulae, &mut my_values, &mut set);
    //             // checks if the corresponding bit is correct, if not, we end iteration early
    //             if !my_values.contains_key(z)
    //                 || *my_values.get(z).unwrap()
    //                     != if target & (1 << pow) == 0 { 0 } else { 1 }
    //             {
    //                 break 'reset;
    //             }
    //         }
    //         if let Some(v) = get_num(&mut z_vals, &my_values) {
    //             if v == target {
    //                 println!("RESULT {:?}", col);
    //             }
    //             // println!("{v}");
    //         }
    //         swapped_wires(&col, &mut my_formulae);
    //     }
    // });
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
