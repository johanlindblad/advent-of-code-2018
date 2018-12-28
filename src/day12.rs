use std::boxed::Box;

pub type Pattern = (u8, bool);
pub struct Input {
    initial: Vec<u128>,
    patterns: Vec<bool>
}

fn parse_pattern(line: &str) -> Pattern {
    let (lhs_s, rest) = line.split_at(5);
    let (_, rhs_s) = rest.split_at(4);
    let rhs = rhs_s.chars().next().unwrap();

    let lhs = lhs_s
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '#')
        .map(|(i, _c)| 1u8 << (4 - i))
        .sum();

    (lhs, rhs == '#')
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Box<Input> {
    let mut lines = input.lines();

    let mut initial: Vec<u128> = vec![0; 1];

    lines.next().unwrap().chars().skip(15).enumerate().for_each(|(i, c)| {
        if c == '#' {
            initial[0] |= 1u128 << (127 - i)
        }
    });

    let mut patterns: Vec<bool> = vec![false; 32];

    lines
        .skip(1)
        .map(|l| parse_pattern(l))
        .for_each(|(lhs, rhs)| { patterns[lhs as usize] = rhs; });

    Box::new(Input { initial, patterns })
}

fn run(input: &Input, generations: usize) -> isize {
    let num_pots = 100;
    let mut state = input.initial.clone();
    let patterns = &input.patterns;

    let mut n_minus_2;
    let mut n_minus_1 = 0;
    let mut n = 0;

    let indices = |index: isize, pad: usize| -> (usize, usize) {
        let extra = pad * 128;
        let adjusted = extra as isize + index;
        let position = (adjusted / 128) as usize;

        let bitoffset = 127 - ((128 + index) % 128) as usize;
        (position, bitoffset)
    };

    let val = |ref state: &Vec<u128>, index: isize, pad: usize| -> u128 {
        let (pos, off) = indices(index, pad);
        (state[pos] & (1u128 << off)) >> off
    };

    let set = |ref mut state: &mut Vec<u128>, index: isize, pad: usize, val: bool| {
        let (pos, off) = indices(index, pad);

        if val {
            state[pos] |= (1 as u128) << off
        } else {
            state[pos] &= !((1 as u128) << off)
        }
    };

    for gen in 1..=generations {
        n_minus_2 = n_minus_1;
        n_minus_1 = n;

        let left: isize = 0 - gen as isize - 1;
        let right: isize = num_pots as isize + gen as isize + 2;

        let pad = 1 + (gen / 128);

        let expected_len = 1 + (2 * pad as usize);
        if state.len() < expected_len {
            state.insert(0, 0);
            state.push(0);
        }

        let mut next_state = state.clone();

        let mut acc: u128 =
            (val(&state, left - 2, pad as usize) << 4) |
            (val(&state, left - 1, pad as usize) << 3) |
            (val(&state, left + 0, pad as usize) << 2) |
            (val(&state, left + 1, pad as usize) << 1) |
            (val(&state, left + 2, pad as usize));

        for i in left..right {
            let new_val = patterns[acc as usize];
            set(&mut next_state, i, pad as usize, new_val);
            acc = ((acc << 1) & (0b11111)) | val(&state, i + 3, pad as usize);
        }

        state.swap_with_slice(&mut next_state);

        n = (left..right).filter(|&i| val(&state, i, pad as usize) == 1).sum();

        // It eventually reaches a steady state where sum grows by 23 per generation
        if n - n_minus_1 == n_minus_1 - n_minus_2 {
            let remaining = generations as isize - gen as isize;
            let difference = n - n_minus_1;
            let additional = remaining * difference;
            return n + additional;
        }
    }

    let left: isize = 0 - generations as isize - 1;
    let right: isize = num_pots as isize + generations as isize + 2;
    let pad = 1 + (generations / 128);

    (left..right).filter(|&i| val(&state, i, pad as usize) == 1).sum()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> isize {
    run(input, 20)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Input) -> isize {
    run(input, 50000000000)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1};

    #[test]
    fn examples() {
        let raw = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

        assert_eq!(solve_part1(&input_generator(raw)), 325);
    }
}

