use std::collections::BTreeMap;
use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s: &str| s.to_string())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let (twos, threes) =
        input
            .iter()
            .map(|line| char_counts(line.to_string()))
            .map(|counts| (
                 has_with_count(&counts, 2),
                 has_with_count(&counts, 3)
            ))
            .fold((0, 0), |(twos, threes), (two, three)|
                  match (two, three) {
                      (true, true) => (twos + 1, threes + 1),
                      (true, false) => (twos + 1, threes),
                      (false, true) => (twos, threes + 1),
                      (false, false) => (twos, threes)
                  }
            );

    twos * threes

}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> String {
    let words =
        input
            .iter()
            .cartesian_product(input)
            .find(|(str1, str2)| differ_by_one(str1.to_string(), str2.to_string()));

    match words {
        Some((word1, word2)) => {
            common_characters(word1.to_string(), word2.to_string())
        },
        None => panic!("Expected to find correct words")
    }
}

fn differ_by_one(str1: String, str2: String) -> bool {
    let differences =
        str1
            .chars()
            .zip(str2.chars())
            .filter(|(char1, char2)|
                 char1 != char2
            )
            .count();

    differences == 1
}

fn common_characters(str1: String, str2: String) -> String {
    str1
        .chars()
        .zip(str2.chars())
        .filter(|(char1, char2)|
             char1 == char2
        )
        .map(|(char1, _char2)| char1)
        .collect::<String>()
}

fn has_with_count(counts: &BTreeMap<char, isize>, count: isize) -> bool {
    counts
        .values()
        .any(|&c| c == count)
}

fn char_counts(str: String) -> BTreeMap<char, isize> {
    let mut counts = BTreeMap::new();

    str
        .chars()
        .for_each(|char| {
            counts
                .entry(char)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        });

    counts
}

#[cfg(test)]
mod tests {
    use super::{solve_part1};
    #[test]
    fn examples() {
        //assert_eq!(solve_part1(&[1,1,1]), 3);
    }
}

