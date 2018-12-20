pub type Quintuple = [usize; 4];
pub type Example = (Quintuple, Quintuple, Quintuple);
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Instruction { Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti,
                   Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr }
pub type Input = (Vec<Example>, Vec<Quintuple>);
use std::collections::BTreeSet;
use std::boxed::Box;

fn to_quin(vec: Vec<usize>) -> Quintuple {
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

            let example = (to_quin(before), to_quin(instruction), to_quin(after));
            examples.push(example);
        }
    }

    let parse_line = |line: &str| line.split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let program = lines
        .iter()
        .skip(2 + (4 * examples.len()))
        .map(|l| to_quin(parse_line(l)))
        .collect::<Vec<Quintuple>>();

    Box::new((examples, program))
}

fn addr(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] + registers[b];
    new_registers
}

fn addi(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] + b;
    new_registers
}

fn mulr(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] * registers[b];
    new_registers
}

fn muli(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] * b;
    new_registers
}

fn banr(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] & registers[b];
    new_registers
}

fn bani(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] & b;
    new_registers
}

fn borr(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] | registers[b];
    new_registers
}

fn bori(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a] | b;
    new_registers
}

fn setr(a: usize, _b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = registers[a];
    new_registers
}

fn seti(a: usize, _b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = a;
    new_registers
}

fn gtir(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if a > registers[b] { new_registers[c] = 1 };
    new_registers
}

fn gtri(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if registers[a] > b { new_registers[c] = 1 };
    new_registers
}

fn gtrr(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if registers[a] > registers[b] { new_registers[c] = 1 };
    new_registers
}

fn eqir(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if a == registers[b] { new_registers[c] = 1 };
    new_registers
}

fn eqri(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if registers[a] == b { new_registers[c] = 1 };
    new_registers
}

fn eqrr(a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    let mut new_registers = registers.clone();
    new_registers[c] = 0;
    if registers[a] == registers[b] { new_registers[c] = 1 };
    new_registers
}

fn op_codes() -> [Instruction; 16] {
    [
        Instruction::Addr, Instruction::Addi, Instruction::Mulr, Instruction::Muli, Instruction::Banr,
        Instruction::Bani, Instruction::Borr, Instruction::Bori, Instruction::Setr, Instruction::Seti,
        Instruction::Gtir, Instruction::Gtri, Instruction::Gtrr, Instruction::Eqir, Instruction::Eqri,
        Instruction::Eqrr
    ]
}

fn execute(ins: Instruction, a: usize, b: usize, c: usize, registers: Quintuple) -> Quintuple {
    match ins {
        Instruction::Addr => addr(a, b, c, registers),
        Instruction::Addi => addi(a, b, c, registers),
        Instruction::Mulr => mulr(a, b, c, registers),
        Instruction::Muli => muli(a, b, c, registers),
        Instruction::Banr => banr(a, b, c, registers),
        Instruction::Bani => bani(a, b, c, registers),
        Instruction::Borr => borr(a, b, c, registers),
        Instruction::Bori => bori(a, b, c, registers),
        Instruction::Setr => setr(a, b, c, registers),
        Instruction::Seti => seti(a, b, c, registers),
        Instruction::Gtir => gtir(a, b, c, registers),
        Instruction::Gtri => gtri(a, b, c, registers),
        Instruction::Gtrr => gtrr(a, b, c, registers),
        Instruction::Eqir => eqir(a, b, c, registers),
        Instruction::Eqri => eqri(a, b, c, registers),
        Instruction::Eqrr => eqrr(a, b, c, registers)
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let (examples, _) = input;
    let mut three_or_more = 0;

    for (before, [num, a, b, c], after) in examples {
        let mut matches = 0;

        for instr in &op_codes() {
            if execute((*instr).clone(), *a, *b, *c, *before) == *after {
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
    let prototype: BTreeSet<Instruction> = [Instruction::Addr, Instruction::Addi, Instruction::Mulr, Instruction::Muli, Instruction::Banr, Instruction::Bani, Instruction::Borr, Instruction::Bori, Instruction::Setr, Instruction::Seti,
Instruction::Gtir, Instruction::Gtri, Instruction::Gtrr, Instruction::Eqir, Instruction::Eqri, Instruction::Eqrr].iter().cloned().collect();
    let mut options: Vec<BTreeSet<Instruction>> = vec![prototype.clone(); 16];

    for (before, [num, a, b, c], after) in examples {
        let mut matches = 0;

        for instr in &op_codes() {
            if options[*num].contains(instr) && execute((*instr).clone(), *a, *b, *c, *before) != *after {
                options[*num].remove(instr);
            }
        }
    }

    let mut unassigned = prototype.clone();
    let mut mapping: Vec<Instruction> = vec![Instruction::Addr; 16];

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

    let mut registers: Quintuple = [0; 4];

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

