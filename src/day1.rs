use std::collections::BTreeSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[isize]) -> isize {
    input
        .iter()
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[isize]) -> isize {
    let mut visited = BTreeSet::<isize>::new();
    let mut sum = 0;
    visited.insert(0);

    for i in input.iter().cycle() {
        sum += i;

        if visited.contains(&sum) {
            return sum;
        }

        visited.insert(sum);
    }

    panic!("{}", "Should not get here");
}

#[cfg(test)]
mod tests {
    use super::{solve_part1};
    #[test]
    fn examples() {
        assert_eq!(solve_part1(&[1,1,1]), 3);
        assert_eq!(solve_part1(&[1,1,-2]), 0);
        assert_eq!(solve_part1(&[-1, -2, -3]), -6);
    }
}
