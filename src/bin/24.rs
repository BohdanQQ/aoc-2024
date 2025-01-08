use std::collections::{HashMap, HashSet};

advent_of_code::solution!(24, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

impl<'a> Operator<'a> {
    fn is_over(&self, op: &'a str) -> bool {
        match self {
            Operator::And(a, b) => &op == a || &op == b,
            Operator::Or(a, b) => &op == a || &op == b,
            Operator::Xor(a, b) => &op == a || &op == b,
        }
    }
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
    let lines = input.lines().collect::<Vec<_>>();
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

pub fn part_two<'a>(input: &'a str) -> Option<u32> {
    // solution mps z25 vcv z13 vwp  z19 vjv cqm

    /* Structure-based solution:

            Used binary adder structure:

             ┌─────────────────────────────────────────────────────────┐
             │         ┌────────────┐                                  │
             │         │            │                                  │   zi
             │         │            │                                  │
             │ ┌───────►   Z XOR    ┼──────────────────────────────────┼────►
             │ │     c │            │                                  │
             │ │       │            │                                  │
             │ │       └─────▲──────┘                                  │
             │ │           xi│                                         │
             │ │             │                                         │
             │ │             │      ┌────────────┐     ┌────────────┐  │
             │ │             │      │            │     │            │  │
             │ │             │    c │            ┼─────►            │  │next carry
    Carry  ──┼─┼─────────────┼──────►   MID AND  │     │  FINAL OR  ┼──┼───►
             │               │      │            │  ┌──►            │  │
             │               │  ┌───►            │  │  │            │  │
             │               │  │ xi└────────────┘  │  └────────────┘  │
             │               │  │                   │                  │
             │               │  │                   │                  │
             │         ┌─────┴──┴───┐             ┌─┴─────────┐        │
             │         │            │             │           │        │
             │         │            │             │           │        │
             │         │            │             │           │        │
             │         │    XOR     │             │    AND    │        │
             │         │            │             │           │        │
             │         │            │             │           │        │
             │         └─────▲───▲──┘             └───▲───▲───┘        │
             │            xi │   │yi                yi│   │xi          │
             │               │   │                    │   │            │
             │               ┼───┼────────────────────┼───┘            │
             │               │   └────────────────────┼                │
             └─────────────────────────────────────────────────────────┘
                             │                        │
                            Xi                       Yi

            This layout and names should help in analysis using the algorithm below:

         */
    let (values, formulae) = parse(input);
    // generate needed AND and XOR gates:
    let mut needed = values
        .iter()
        .take(values.len() / 2)
        .enumerate()
        .map(|(i, _)| (format!("x{:02}", i), format!("y{:02}", i)))
        .collect::<Vec<(String, String)>>();
    needed.sort_by(|a, b| a.0.cmp(&b.0));
    let mut prev_carry_gate: Option<String> = None;
    let mut first = true;
    let is_xor = |x: &Operator<'a>| match x {
        Operator::And(_, _) => false,
        Operator::Or(_, _) => false,
        Operator::Xor(_, _) => true,
    };

    // note: ignore the first layer's warnings
    // for x00, y00 ; x01, y01...
    // check initial AND and XOR gates as well as the zXOR gate (a preliminary analysis that helps further)
    for (x, y) in needed.clone() {
        let and = formulae
            .iter()
            .filter(|(op, _)| op == &Operator::And(&x, &y) || op == &Operator::And(&y, &x))
            .collect::<Vec<_>>();
        if and.is_empty() {
            println!("Formula X AND Y missing: {:?} {:?}", x, y);
            continue;
        } else if and.len() > 1 {
            println!(
                "Suspicious, only one AND formula should be present, meanwhile got more: {:?}",
                and
            );
        }
        let xor = formulae
            .iter()
            .filter(|(op, _)| op == &Operator::Xor(&x, &y) || op == &Operator::Xor(&y, &x))
            .collect::<Vec<_>>();
        if xor.is_empty() {
            println!("Formula X XOR Y missing: {:?} {:?}", x, y);
            continue;
        } else if xor.len() > 1 {
            println!(
                "Suspicious, only one XOR formula should be present, meanwhile got more: {:?}",
                and
            );
        }

        let and = and[0].1;
        let xor = xor[0].1;
        let z_xor = formulae
            .iter()
            .filter(|(op, _)| op.is_over(xor))
            .collect::<Vec<_>>();
        if z_xor.is_empty() {
            println!("Formula ZXOR missing: xor {:?}, and {and}, x {:?}, y {:?}, should result in z\n{:?}", xor, x, y, z_xor);
        } else if z_xor.iter().filter(|x| is_xor(&x.0)).count() != 1 {
            println!("count of ZXOR suspicious: xor {:?}, and {and}, x {:?}, y {:?}, should result in z\n{:?}", xor, x, y, z_xor);
        }
        let z_xor2 = z_xor
            .iter()
            .filter(|x| x.1.starts_with('z') && is_xor(&x.0))
            .collect::<Vec<_>>();
        if z_xor2.len() != 1 {
            println!("Suspicious, only one XOR formula should be present, ending in zXX for {:?} {:?}, xor {xor}, and {and} meanwhile got: {:?}", x, y, z_xor2);
            println!("Previous result: {:?}", z_xor);
        }
    }

    for (x, y) in needed {
        let and = formulae
            .iter()
            .filter(|(op, _)| op == &Operator::And(&x, &y) || op == &Operator::And(&y, &x))
            .collect::<Vec<_>>();
        let xor = formulae
            .iter()
            .filter(|(op, _)| op == &Operator::Xor(&x, &y) || op == &Operator::Xor(&y, &x))
            .collect::<Vec<_>>();

        let and = and[0].1;
        let xor = xor[0].1;

        if let Some(ref carry) = prev_carry_gate {
            // find prevcarry xor "xor variable"
            let z_xor = formulae
                .iter()
                .filter(|(op, _)| {
                    op == &Operator::Xor(xor, carry) || op == &Operator::Xor(carry, xor)
                })
                .collect::<Vec<_>>();
            if z_xor.is_empty() {
                println!("Formula ZXOR missing: xor {xor} carry {carry} for x, y, should result in z {:?}, {:?}", x, y);
            } else if z_xor.len() > 1 {
                println!("Suspicious, only one XOR formula should be present, ending in zXX for {:?} {:?}, meanwhile got more: {:?}", x, y, z_xor);
            }
        } else if !first {
            panic!(
                "Zxor not found for x, y, and, xor {x} {y} {and} {xor} prev carry: {:?}",
                prev_carry_gate
            );
        };

        let mid_and = if let Some(ref carry) = prev_carry_gate {
            // find prevcarry xor "xor variable"
            let mid_and = formulae
                .iter()
                .filter(|(op, _)| {
                    op == &Operator::And(xor, carry) || op == &Operator::And(carry, xor)
                })
                .collect::<Vec<_>>();
            if mid_and.is_empty() {
                println!(
                    "Formula midAND missing: {:?} {:?} {:?} {:?} for xor, carry, x, y",
                    xor, carry, x, y
                );
            } else if mid_and.len() > 1 {
                println!("Suspicious, only one midAND formula should be present, for x y and xor {x} {y} {and} {xor} prev carry: {:?},  meanwhile got more: {:?}", prev_carry_gate, mid_and);
            }
            // this line may panic, because mid_and was not found => ok since the analysis cannot continue - you must fix the mistake
            mid_and[0].1
        } else {
            if !first {
                panic!(
                    "Mid And not found for x, y, and, xor {x} {y} {and} {xor} prev carry: {:?}",
                    prev_carry_gate
                );
            }
            xor
        };

        let final_or = formulae
            .iter()
            .filter(|(op, _)| {
                op == &Operator::Or(mid_and, and) || op == &Operator::Or(and, mid_and)
            })
            .collect::<Vec<_>>();
        if final_or.is_empty() {
            println!("Formula final OR missing: and {and} midand {mid_and} x {x} y {y}");
            // continue
        } else if final_or.len() > 1 {
            println!("Suspicious, only one final OR formula should be present, for x y and xor midand {x} {y} {and} {xor} {mid_and} prev carry: {:?},  meanwhile got more: {:?}", prev_carry_gate, final_or);
            // continue
        }
        prev_carry_gate = Some(if first {
            and.to_owned()
        } else {
            // this line may panic, because final_or was not found => ok since the analysis cannot continue - you must fix the mistake
            final_or[0].1.to_owned()
        });
        first = false;
    }
    Some(0)
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
