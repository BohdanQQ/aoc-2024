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
            0..=3 => ComboOperand::SmallLit(value.into()),
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
        let a_div_op =
            |combo_operand: ComboOperand| self.a >> self.get_combo_op_val(combo_operand) as usize;
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
                    self.ip = op as usize >> 1;
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
    // used for bruteforce attempt xd
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

pub fn part_one(input: &str) -> Option<usize> {
    let (reg_a, reg_b, reg_c, instrs, _) = parse_init(input);
    // println!("inputs {} {} {} {:?}", reg_a, reg_b, reg_c, instrs);

    let mut m = Machine::new(0, reg_a, reg_b, reg_c, instrs);
    while m.execute() {}
    println!("{:?}", m.results);
    None
}

// helper that creates the bits of an octet
fn val_to_sl(v: u8) -> [u8; 3] {
    if v > 7 {
        panic!("bad")
    } else {
        let fst = (v << 7) >> 7;
        let snd = (v << 6) >> 7;
        let trd = (v << 5) >> 7;
        [trd, snd, fst]
    }
}

// a lookup table for xor values
// my version of the input program first flips bottom 3 bits of the accumulator (A)
// and indexes via those flipped bits (IDX_XOR further) into A - takes 3 bits from there
// and xors the result with the bottom 3 bits of A and outputs the value

// the table here has lines that correspond to bit sequences in the bottom 3 bits
// the bit flip is (7 - number) so it is not present in the table
// each row contains the values that need to be at IDX_XOR in order to produce desired number (column)

// so row 4 corresponding to 3 (011), column 2 corresponding to 1 (001) has value (010) (011^001 = 010)
// this says that if you want to output 1, when the lowest bits are 011, bits (7 - 4 the IDX_XOR) 3, 4, 5 must be set to 010
// this table is crucial to the construction of the final number
fn get_table() -> Table {
    let mut result = [[[EMPTY; 3]; 8]; 8];
    for (start_val, row) in result.iter_mut().enumerate() {
        for (target_val, cell) in row.iter_mut().enumerate() {
            *cell = val_to_sl(start_val as u8 ^ target_val as u8);
        }
    }

    result
}

const EMPTY: u8 = 8;
type NumField = [u8; 64];
type Bits = [u8; 3];
type Table = [[Bits; 8]; 8];

// checks if bits fit
// [ ... E, E, E, 0, 1] query with [0, 1, 0] on index 1 is TRUE
// [ ... E, E, E, 0, 1] query with [0, 1, 0] on index 0 is FALSE
// [ ... E, E, 1, 0, 1] query with [1, 0, 1] on index 0 is TRUE
// for the following functions, read them thus:
//      real_idx_field - index into field from the back
//      real_idx_bits  - index into the 3-element slice of bits
fn fits(field: &NumField, idx: usize, bits: &Bits) -> bool {
    for i in idx..(idx + 3).min(field.len()) {
        let real_idx_field = field.len() - i - 1;
        let real_idx_bits = 3 + idx - i - 1;
        // println!("{i}, {idx} {real_idx_bits}");
        if field[real_idx_field] != bits[real_idx_bits] && field[real_idx_field] != EMPTY {
            return false;
        }
    }
    true
}

// usless but created as a preparation (gets all possibilies that fit)
fn possibilities(field: &NumField, idx: usize) -> Vec<Bits> {
    let mut result: Vec<Bits> = vec![[EMPTY; 3]];
    for i in idx..(idx + 3).min(field.len()) {
        let real_idx_field = field.len() - i - 1;
        let real_idx_bits = 3 + idx - i - 1;
        if field[real_idx_field] == EMPTY {
            let mut other = vec![];
            for x in &mut result {
                let mut cpy = *x;
                x[real_idx_bits] = 0;
                cpy[real_idx_bits] = 1;
                other.push(cpy);
            }
            result.append(&mut other);
        } else {
            for x in &mut result {
                x[real_idx_bits] = field[real_idx_field];
            }
        }
    }
    result
}

fn possibilities_all(field: &NumField) -> Vec<NumField> {
    let mut result: Vec<NumField> = vec![[EMPTY; 64]];
    for i in 0..field.len() {
        let real_idx_field = field.len() - i - 1;
        if field[real_idx_field] == EMPTY {
            let mut other = vec![];
            for x in &mut result {
                let mut cpy = *x;
                x[real_idx_field] = 0;
                cpy[real_idx_field] = 1;
                other.push(cpy);
            }
            result.append(&mut other);
        } else {
            for x in &mut result {
                x[real_idx_field] = field[real_idx_field];
            }
        }
    }
    result
}

// merges bits
// [ ... E, E, E, 0, 1] merge with [0, 1, 1] on index 2
// vvvvvvvvvvvvvvvvvvvv
// [ ... 0, 1, 1, 0, 1]
fn merge(field: &mut NumField, idx: usize, bits: Bits) {
    for i in idx..(idx + 3).min(field.len()) {
        let real_idx_field = field.len() - i - 1;
        let real_idx_bits = 3 + idx - i - 1;
        if field[real_idx_field] == EMPTY {
            field[real_idx_field] = bits[real_idx_bits];
        } else if field[real_idx_field] != bits[real_idx_bits] {
            panic!("Cannot merge {:?} {:?} at {idx}", field, bits);
        }
    }
}
// outputs all solutions
// also outputs "spurious" solutions (ones that give a sequence that contain some more
// elements than the original one - but strictly more (for original 0 2 3 4 the spurious
// output could be 0 2 3 4 1))

fn solve(tbl: &Table, field: &mut NumField, idx: usize, targets: &Vec<u8>) {
    if idx == targets.len() {
        // println!("Original: \n{:?}", field);
        for f in field.iter_mut() {
            if *f != EMPTY {
                break;
            }
            if *f == EMPTY {
                *f = 0;
            }
        }
        let mut all_seen = vec![];

        for pos in possibilities_all(field) {
            if !all_seen.contains(&pos) {
                all_seen.push(pos);
            }
        }

        if all_seen.is_empty() {
            all_seen.push(*field);
        }

        for f in all_seen {
            // println!("{:?}", &f);
            println!("{:?}", get_num(&f));
        }
        return;
    }

    let field_idx = idx * 3;
    let target = targets[idx];
    for i in 0..8 {
        let bits = val_to_sl(i as u8);
        let line = tbl[i];
        if !fits(field, field_idx, &bits) {
            continue;
        }
        let prev = *field;
        merge(field, field_idx, bits);

        let candidate = line[target as usize];
        let candidate_idx = field_idx + (7 - i);
        // this check below and the "fits" check above cannot be merged because some
        // overlaps between bits and candidate (eg. when candidate_idx == 0)
        // are invalid (and become apparent only after the first merge is complete)
        // this could be tackled in the table itself (having an empty entry)
        // or directly here by comparing candidate and field_idx and should they overlap
        // the candidate and bits bitfields
        if !fits(field, candidate_idx, &candidate) {
            *field = prev;
            continue;
        }
        merge(field, candidate_idx, candidate);

        solve(tbl, field, idx + 1, targets);
        *field = prev;
    }
}

fn get_num(field: &NumField) -> u64 {
    let mut res = 0;
    let mut num = *field;
    num.reverse();
    for (i, n) in num.iter().enumerate() {
        if *n == EMPTY {
            break;
        } else if *n == 1 {
            res += pow::pow(2, i);
        } else if *n != 0 {
            panic!("invalid format!");
        }
    }
    res
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, _, _, _, orig) = parse_init(input);
    let tbl = get_table();

    // for (i, line) in tbl.iter().enumerate() {
    //     if i == 0 {
    //         println!("at 0/ output | {:?} | {:?} | {:?} | {:?} | {:?} | {:?} | {:?} | {:?} |", val_to_sl(0),  val_to_sl(1),  val_to_sl(2),  val_to_sl(3),  val_to_sl(4),  val_to_sl(5),  val_to_sl(6),  val_to_sl(7));
    //         println!("--------------------------------------------------------------------------------------------------------------");
    //     }
    //     print!("  {:?}  | ", val_to_sl(i as u8));
    //     for v in line {
    //         print!("{:?} | ", v);
    //     }
    //     println!()
    // }
    /*
    The above generates the following:
      at 0/ output | [0, 0, 0] | [0, 0, 1] | [0, 1, 0] | [0, 1, 1] | [1, 0, 0] | [1, 0, 1] | [1, 1, 0] | [1, 1, 1] |
      --------------------------------------------------------------------------------------------------------------
        [0, 0, 0]  | [0, 0, 0] | [0, 0, 1] | [0, 1, 0] | [0, 1, 1] | [1, 0, 0] | [1, 0, 1] | [1, 1, 0] | [1, 1, 1] |
        [0, 0, 1]  | [0, 0, 1] | [0, 0, 0] | [0, 1, 1] | [0, 1, 0] | [1, 0, 1] | [1, 0, 0] | [1, 1, 1] | [1, 1, 0] |
        [0, 1, 0]  | [0, 1, 0] | [0, 1, 1] | [0, 0, 0] | [0, 0, 1] | [1, 1, 0] | [1, 1, 1] | [1, 0, 0] | [1, 0, 1] |
        [0, 1, 1]  | [0, 1, 1] | [0, 1, 0] | [0, 0, 1] | [0, 0, 0] | [1, 1, 1] | [1, 1, 0] | [1, 0, 1] | [1, 0, 0] |
        [1, 0, 0]  | [1, 0, 0] | [1, 0, 1] | [1, 1, 0] | [1, 1, 1] | [0, 0, 0] | [0, 0, 1] | [0, 1, 0] | [0, 1, 1] |
        [1, 0, 1]  | [1, 0, 1] | [1, 0, 0] | [1, 1, 1] | [1, 1, 0] | [0, 0, 1] | [0, 0, 0] | [0, 1, 1] | [0, 1, 0] |
        [1, 1, 0]  | [1, 1, 0] | [1, 1, 1] | [1, 0, 0] | [1, 0, 1] | [0, 1, 0] | [0, 1, 1] | [0, 0, 0] | [0, 0, 1] |
        [1, 1, 1]  | [1, 1, 1] | [1, 1, 0] | [1, 0, 1] | [1, 0, 0] | [0, 1, 1] | [0, 1, 0] | [0, 0, 1] | [0, 0, 0] |
     */

    let mut value = [EMPTY; 64];
    // demo for playing with the functions
    // {
    //     value[63] = 1;
    //     value[62] = EMPTY;
    //     value[61] = 1;
    //     println!("{}", fits(&value, 2, &[0, 1, 1]));
    //     println!("Pos:\n{:?}", possibilities(&value, 2));
    //     merge(&mut value, 50, [0, 1, 1]);
    //     println!("{:?}", value);
    // }
    solve(&tbl, &mut value, 0, &orig);

    // solutions
    // 265652340990875
    // 265652340990877
    Some(0)
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
        assert_eq!(result, Some(265652340990875));
    }
}
