use std::boxed::Box;
use std::cmp::min;

pub fn react_once(polymer: &str) -> Option<String> {
    let mut after = String::new();

    let mut it = polymer.chars().peekable();

    while let Some(c) = it.next() {
        match it.peek() {
            None => after.push(c),
            Some(&other_c) => {
                let react =
                    c != other_c && c.to_ascii_uppercase() == other_c.to_ascii_uppercase();

                if react {
                    it.next();
                } else {
                    after.push(c);
                }
            }
        }
    }

    if after.len() == polymer.len() {
        None
    } else {
        Some(after)
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut out = input.trim().to_string();

    while let Some(after) = react_once(&out) {
        out = after;
    }

    out.len()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut uniq_chars = input
        .trim()
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();

    uniq_chars.sort();
    uniq_chars.dedup();

    let mut shortest = input.len();

    let out = react_once(&input.trim()).unwrap();

    for c in uniq_chars {
        let mut polymer = out
            .replace(|x| x == c, "")
            .replace(|x| x == c.to_ascii_lowercase(), "")
            .to_string();

        while let Some(after) = react_once(&polymer) {
            polymer = after;
        }

        shortest = min(shortest, polymer.len());
    }

    shortest
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, react_once};
    #[test]
    fn examples() {
        assert_eq!(react_once("bAaB"), Some("bB".to_string()));
        assert_eq!(react_once("bB"), Some("".to_string()));
        assert_eq!(solve_part1("dabAcCaCBAcCcaDA"), 10);
        assert_eq!(solve_part1("abBA"), 0);
        assert_eq!(solve_part1("aaA"), 1);
    }
}

