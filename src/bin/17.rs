use std::{thread, u64};

use pathfinding::num_traits::pow;

advent_of_code::solution!(17);

type RegVal = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComboOperand {
    SmallLit(RegVal),
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Insn {
    Adv(ComboOperand),
    Bxl(u8),
    Bst(ComboOperand),
    Jnz(u8),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl From<u8> for ComboOperand {
    fn from(value: u8) -> Self {
        match value {
            0 | 1 | 2 | 3 => ComboOperand::SmallLit(value.into()),
            4 => ComboOperand::A,
            5 => ComboOperand::B,
            6 => ComboOperand::C,
            _ => panic!("unexpected"),
        }
    }
}

impl From<(u8, u8)> for Insn {
    fn from((op_code, operand): (u8, u8)) -> Self {
        match op_code {
            0 => Insn::Adv(operand.into()),
            1 => Insn::Bxl(operand),
            2 => Insn::Bst(operand.into()),
            3 => Insn::Jnz(operand),
            4 => Insn::Bxc,
            5 => Insn::Out(operand.into()),
            6 => Insn::Bdv(operand.into()),
            7 => Insn::Cdv(operand.into()),
            _ => panic!("unknown opcode"),
        }
    }
}

fn parse_init(input: &str) -> (RegVal, RegVal, RegVal, Vec<Insn>, Vec<u8>) {
    if let [l1, l2, l3, _, lp] = input.split('\n').collect::<Vec<_>>().as_slice() {
        let get_reg_num = |i: &str| i.split(": ").nth(1).unwrap().parse::<RegVal>().unwrap();

        let numbers = lp
            .split(": ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|v| v.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let mut insn = vec![];
        for i in 0..numbers.len() {
            if i % 2 == 0 {
                insn.push(Insn::from((numbers[i], numbers[i + 1])));
            }
        }

        (
            get_reg_num(l1),
            get_reg_num(l2),
            get_reg_num(l3),
            insn,
            numbers,
        )
    } else {
        panic!("xd {:?}", input.split('\n').collect::<Vec<_>>().as_slice());
    }
}

struct Machine {
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
    insn: Vec<Insn>,
    results: Vec<u8>,
}

impl Machine {
    pub fn new(ip: usize, a: u64, b: u64, c: u64, insn: Vec<Insn>) -> Self {
        Self {
            ip,
            a,
            b,
            c,
            insn,
            results: vec![],
        }
    }

    fn get_combo_op_val(&self, op: ComboOperand) -> u64 {
        match op {
            ComboOperand::SmallLit(x) => x,
            ComboOperand::A => self.a,
            ComboOperand::B => self.b,
            ComboOperand::C => self.c,
        }
    }

    pub fn execute(&mut self) -> bool {
        if self.ip >= self.insn.len() {
            return false;
        }
        let insn = self.insn[self.ip];
        let a_div_op = |combo_operand: ComboOperand| {
            self.a / pow::pow(2u64, self.get_combo_op_val(combo_operand) as usize)
        };
        let mut skip_ip_incr = false;
        match insn {
            Insn::Adv(combo_operand) => {
                self.a = a_div_op(combo_operand);
            }
            Insn::Bxl(op) => self.b ^= op as u64,
            Insn::Bst(combo_operand) => self.b = self.get_combo_op_val(combo_operand) % 8,
            Insn::Jnz(op) => {
                if self.a != 0 {
                    skip_ip_incr = true;
                    self.ip = op as usize / 2;
                }
            }
            Insn::Bxc => self.b ^= self.c,
            Insn::Out(combo_operand) => self
                .results
                .push((self.get_combo_op_val(combo_operand) % 8) as u8),
            Insn::Bdv(combo_operand) => self.b = a_div_op(combo_operand),
            Insn::Cdv(combo_operand) => self.c = a_div_op(combo_operand),
        }

        if !skip_ip_incr {
            self.ip += 1;
        }
        true
    }

    pub fn inspect(&self, expected: &[u8]) -> bool {
        for (i, v) in self.results.iter().enumerate() {
            if i < expected.len() && *v != expected[i] {
                return false;
            }
        }
        true
    }

    pub fn reset(&mut self, ip: usize, a: u64, b: u64, c: u64) {
        self.ip = ip;
        self.a = a;
        self.b = b;
        self.c = c;
        self.results.clear();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (regA, regB, regC, instrs, _) = parse_init(input);
    println!("inputs {} {} {} {:?}", regA, regB, regC, instrs);

    let mut m = Machine::new(0, regA, regB, regC, instrs);
    while m.execute() {}
    println!("{:?}", m.results);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut a: u64 = 1000000000000;
    let (_, reg_b, reg_c, instrs, orig) = parse_init(input);
    let mut m = Machine::new(0, a, reg_b, reg_c, instrs);
    loop {
        m.reset(0, a, reg_b, reg_c);
        if a % 100000000 == 0 {
            println!("try {}", a);
        }
        while m.execute() {
            if !m.inspect(&orig) {
                break;
            }
        }
        if m.results.len() == orig.len() && m.inspect(&orig) {
            println!("A = {:?}", a);
            return None;
        }
        a += 1;
    }
    return None;
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
