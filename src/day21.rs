use day16::{execute};
use day19::{input_generator, Input};

#[aoc_generator(day21)]
pub fn input_generator_wrapper(input: &str) -> Box<Input> {
    input_generator(input)
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let ip: usize = input.0.clone();
    let program = &input.1;
    let mut registers = vec![0, 0, 0, 0, 0, 0];
    let mut num_instructions = 0;

    while registers[ip] < program.len() {
        let pc = registers[ip];

        let (ref opcode, opa, opb, opc) = program[registers[ip]];
        // Instruction at pc=28 is the exit condition
        // It compares reg0 (which is never touched otherwise) to reg1
        // As such, the answer is in reg1, since reg0 == reg1 causes
        // the earliest exist at this point
        if pc == 28 {
            return registers[opa];
        }
        execute(opcode.clone(), opa, opb, opc, &mut registers);
        registers[ip] += 1;
        num_instructions += 1;
    }

    num_instructions
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let ip: usize = input.0.clone();
    let program = &input.1;
    let mut registers = vec![0, 0, 0, 0, 0, 0];
    let mut num_instructions = 0;

    let mut seen: Vec<bool> = Vec::new();
    let mut previous = 0;
    let mut times = 0;

    while registers[ip] < program.len() {
        let pc = registers[ip];

        let (ref opcode, opa, opb, opc) = program[registers[ip]];
        // Instruction at pc=28 is the exit condition
        // It compares reg0 (which is never touched otherwise) to reg1
        // As such, the answer is whatever last value of reg1 before they
        // repeat
        if pc == 28 {
            times += 1;

            if (times % 1000) == 0 {
                println!("{}", times);
            }

            let value = registers[opa];

            if seen.len() <= value {
                seen.resize(value + 1, false);
            }

            if seen[value] == true {
                return previous;
            } else {
                seen[value] = true;
                previous = value;
            }
        }
        execute(opcode.clone(), opa, opb, opc, &mut registers);
        registers[ip] += 1;
        num_instructions += 1;
    }

    num_instructions
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
    }
}

