pub type Registers = Vec<usize>;
pub type Instruction = [usize; 4];
pub type Example = (Registers, Instruction, Registers);
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum OpCode { Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti,
                   Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr }
pub type Input = (Vec<Example>, Vec<Instruction>);
use std::collections::BTreeSet;
use std::boxed::Box;

fn to_ins(vec: Vec<usize>) -> Instruction {
    [vec[0], vec[1], vec[2], vec[3]]
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Box<Input> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut iter = lines.chunks(4);

    let mut examples: Vec<Example> = Vec::new();

    while let Some(chunk) = iter.next() {
        if chunk[0].len() > 6 && &chunk[0][0..6] == "Before" {
            let (_, before_s) = chunk[0].split_at(9);
            let before = before_s
                .replace("]", "")
                .split(", ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (_, after_s) = chunk[2].split_at(9);
            let after = after_s
                .replace("]", "")
                .split(", ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let instruction = chunk[1]
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let example = (before, to_ins(instruction), after);
            examples.push(example);
        }
    }

    let parse_line = |line: &str| line.split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let program = lines
        .iter()
        .skip(2 + (4 * examples.len()))
        .map(|l| to_ins(parse_line(l)))
        .collect::<Vec<Instruction>>();

    Box::new((examples, program))
}

pub fn addr(a: usize, b: usize, c: usize, registers: &mut Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] + registers[b];
    new_registers
}

pub fn addi(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] + b;
    new_registers
}

pub fn mulr(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] * registers[b];
    new_registers
}

pub fn muli(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] * b;
    new_registers
}

pub fn banr(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] & registers[b];
    new_registers
}

pub fn bani(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] & b;
    new_registers
}

pub fn borr(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] | registers[b];
    new_registers
}

pub fn bori(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] | b;
    new_registers
}

pub fn setr(a: usize, _b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a];
    new_registers
}

pub fn seti(a: usize, _b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = a;
    new_registers
}

pub fn gtir(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if a > registers[b] { new_registers[c] = 1 };
    new_registers
}

pub fn gtri(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if registers[a] > b { new_registers[c] = 1 };
    new_registers
}

pub fn gtrr(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if registers[a] > registers[b] { new_registers[c] = 1 };
    new_registers
}

pub fn eqir(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if a == registers[b] { new_registers[c] = 1 };
    new_registers
}

pub fn eqri(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if registers[a] == b { new_registers[c] = 1 };
    new_registers
}

pub fn eqrr(a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if registers[a] == registers[b] { new_registers[c] = 1 };
    new_registers
}

fn op_codes() -> [OpCode; 16] {
    [
        OpCode::Addr, OpCode::Addi, OpCode::Mulr, OpCode::Muli, OpCode::Banr,
        OpCode::Bani, OpCode::Borr, OpCode::Bori, OpCode::Setr, OpCode::Seti,
        OpCode::Gtir, OpCode::Gtri, OpCode::Gtrr, OpCode::Eqir, OpCode::Eqri,
        OpCode::Eqrr
    ]
}

pub fn execute(ins: OpCode, a: usize, b: usize, c: usize, registers: Registers) -> Registers {
    match ins {
        OpCode::Addr => addr(a, b, c, registers),
        OpCode::Addi => addi(a, b, c, registers),
        OpCode::Mulr => mulr(a, b, c, registers),
        OpCode::Muli => muli(a, b, c, registers),
        OpCode::Banr => banr(a, b, c, registers),
        OpCode::Bani => bani(a, b, c, registers),
        OpCode::Borr => borr(a, b, c, registers),
        OpCode::Bori => bori(a, b, c, registers),
        OpCode::Setr => setr(a, b, c, registers),
        OpCode::Seti => seti(a, b, c, registers),
        OpCode::Gtir => gtir(a, b, c, registers),
        OpCode::Gtri => gtri(a, b, c, registers),
        OpCode::Gtrr => gtrr(a, b, c, registers),
        OpCode::Eqir => eqir(a, b, c, registers),
        OpCode::Eqri => eqri(a, b, c, registers),
        OpCode::Eqrr => eqrr(a, b, c, registers)
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let (examples, _) = input;
    let mut three_or_more = 0;

    for (before, [_num, a, b, c], after) in examples {
        let mut matches = 0;

        for instr in &op_codes() {
            if execute((*instr).clone(), *a, *b, *c, before.to_vec()) == *after {
                matches += 1;
            }
        }

        if matches >= 3 {
            three_or_more += 1;
        }
    }

    three_or_more
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let (examples, program) = input;
    let prototype: BTreeSet<OpCode> = [OpCode::Addr, OpCode::Addi, OpCode::Mulr, OpCode::Muli, OpCode::Banr, OpCode::Bani, OpCode::Borr, OpCode::Bori, OpCode::Setr, OpCode::Seti,
OpCode::Gtir, OpCode::Gtri, OpCode::Gtrr, OpCode::Eqir, OpCode::Eqri, OpCode::Eqrr].iter().cloned().collect();
    let mut options: Vec<BTreeSet<OpCode>> = vec![prototype.clone(); 16];

    for (before, [num, a, b, c], after) in examples {
        for instr in &op_codes() {
            if options[*num].contains(instr) && execute((*instr).clone(), *a, *b, *c, before.to_vec()) != *after {
                options[*num].remove(instr);
            }
        }
    }

    let mut unassigned = prototype.clone();
    let mut mapping: Vec<OpCode> = vec![OpCode::Addr; 16];

    while unassigned.len() > 0 {
        for i in 0..16 {
            if options[i].len() == 1 {
                let ins = options[i].iter().next().unwrap().clone();
                mapping[i] = ins.clone();
                unassigned.remove(&ins);
                for i in 0..16 {
                    options[i].remove(&ins);
                }
            }
        }
    }

    let mut registers: Registers = vec![0; 4];

    for [op, a, b, c] in program {
        registers = execute(mapping[*op].clone(), *a, *b, *c, registers);
    }

    registers[0]
}

#[cfg(test)]
mod tests {
    use super::{solve_part1};

    #[test]
    fn examples() {
    }
}

