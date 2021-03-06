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
            .map(|line| char_counts(line))
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
            .find(|(str1, str2)| differ_by_one(str1, str2));

    match words {
        Some((word1, word2)) => {
            common_characters(word1, word2)
        },
        None => panic!("Expected to find correct words")
    }
}

fn differ_by_one(str1: &str, str2: &str) -> bool {
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

fn common_characters(str1: &str, str2: &str) -> String {
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

fn char_counts(str: &str) -> BTreeMap<char, isize> {
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
    use super::{solve_part1, input_generator, solve_part2};

    #[test]
    fn examples() {
        let raw = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
        assert_eq!(solve_part1(&input_generator(raw)), 12);

        let raw2 = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        assert_eq!(solve_part2(&input_generator(raw2)), "fgij");
    }
}

