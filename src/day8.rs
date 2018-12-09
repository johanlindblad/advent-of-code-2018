use std::boxed::Box;
use std::collections::VecDeque;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Box<VecDeque<usize>> {
    Box::new(input
        .trim()
        .split(" ")
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>())
}

fn metadata_sum(input: &mut VecDeque<usize>) -> usize {
    let children = input.pop_front().unwrap();
    let entries = input.pop_front().unwrap();

    let child_sum = (0..children).fold(0, |sum, _i| sum + metadata_sum(input));
    let entry_sum = (0..entries).fold(0, |sum, _i| sum + input.pop_front().unwrap());

    child_sum + entry_sum
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &VecDeque<usize>) -> usize {
    let mut cloned: VecDeque<usize> = input.clone();
    metadata_sum(&mut cloned)
}

fn root_value(input: &mut VecDeque<usize>) -> usize {
    let num_children = input.pop_front().unwrap();
    let num_entries = input.pop_front().unwrap();
    let child_values: Vec<usize> = (0..num_children).map(|_i| root_value(input)).collect();
    let entries = (0..num_entries).map(|_i| input.pop_front().unwrap()).collect::<Vec<usize>>();

    if num_children == 0 {
        entries.iter().sum()
    } else {
        let child_sum = entries.iter()
            .filter(|&&e| e > 0 && e <= num_children)
            .map(|e| child_values[e - 1])
            .sum();
        child_sum
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &VecDeque<usize>) -> usize {
    let mut cloned: VecDeque<usize> = input.clone();
    root_value(&mut cloned)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};
    #[test]
    fn examples() {
        let raw = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let input = input_generator(raw);
        assert_eq!(solve_part1(&input), 138);
        assert_eq!(solve_part2(&input), 66);
    }
}

