use day16::{OpCode, execute};

pub type Instruction = (OpCode, usize, usize, usize);
pub type Input = (usize, Vec<Instruction>);

pub fn to_opcode(input: &str) -> OpCode {
    match input {
        "addr" => OpCode::Addr,
        "addi" => OpCode::Addi,
        "mulr" => OpCode::Mulr,
        "muli" => OpCode::Muli,
        "banr" => OpCode::Banr,
        "bani" => OpCode::Bani,
        "borr" => OpCode::Borr,
        "bori" => OpCode::Bori,
        "setr" => OpCode::Setr,
        "seti" => OpCode::Seti,
        "gtir" => OpCode::Gtir,
        "gtri" => OpCode::Gtri,
        "gtrr" => OpCode::Gtrr,
        "eqir" => OpCode::Eqir,
        "eqri" => OpCode::Eqri,
        "eqrr" => OpCode::Eqrr,
        _ => panic!("Unknown instruction {}", input)
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Box<Input> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let ip = first_line[4..5].parse::<usize>().unwrap();
    let mut program: Vec<Instruction> = Vec::new();

    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        let op = to_opcode(split[0]);
        let operands: Vec<usize> = split[1..4].iter().map(|o| o.parse::<usize>().unwrap()).collect();
        program.push((op, operands[0], operands[1], operands[2]));
    }

    Box::new((ip, program))
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let ip: usize = input.0.clone();
    let program = &input.1;
    let mut registers = vec![0, 0, 0, 0, 0, 0];

    while registers[ip] < program.len() {
        let (ref opcode, opa, opb, opc) = program[registers[ip]];
        let new_registers = execute(opcode.clone(), opa, opb, opc, registers);
        registers = new_registers;
        registers[ip] += 1;
    }

    registers[0]
}

#[aoc(day19, part2)]
pub fn solve_part2(_input: &Input) -> usize {
    // Setting register 0 to 1 adds new initialization code that makes the loop
    // go on for longer
    // The program boils down to this, which is the sum of the divisors

    let number = 10551424;
    let mut reg0 = 0;

    for reg1 in 1..=number {
        if (number % reg1) == 0 {
            reg0 += reg1
        }
    }

    reg0
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, input_generator};

    #[test]
    fn examples() {
        let raw = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

        assert_eq!(solve_part1(&input_generator(raw)), 7);
    }
}

