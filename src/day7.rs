use std::boxed::Box;
use std::collections::BinaryHeap;

pub struct Requirements {
    exists: Vec<bool>,
    children: Vec<Vec<char>>,
    parents: Vec<Vec<char>>
}

fn parse_line(input: &str) -> (char, char) {
    let mut chars = input.chars();
    let parent = chars.nth(5).unwrap();
    let child = chars.nth(30).unwrap();
    (parent, child)
}

fn to_key(c: char) -> usize {
    ((c as isize) - 65) as usize
}

fn to_neg_key(c: char) -> isize {
    -(to_key(c) as isize)
}

fn from_key(k: usize) -> char {
    ((k as u8) + 65) as char
}

fn from_neg_key(k: isize) -> char {
    from_key(-(k as isize) as usize)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Box<Requirements> {
    let children = input
        .lines()
        .map(|l| parse_line(l))
        .fold(vec![Vec::new(); 26], |mut vec, (parent, child)| {
            vec[to_key(parent)].push(child);
            vec
        });

    let parents = input
        .lines()
        .map(|l| parse_line(l))
        .fold(vec![Vec::new(); 26], |mut vec, (parent, child)| {
            vec[to_key(child)].push(parent);
            vec
        });

    let exists = input
        .lines()
        .map(|l| parse_line(l))
        .fold(vec![false; 26], |mut exists, (parent, child)| {
            exists[to_key(parent)] = true; exists[to_key(child)] = true;
            exists
        });

    Box::new(Requirements { exists, children, parents })
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Requirements) -> String {
    let mut path: Vec<char> = Vec::new();
    let children = &input.children;
    let mut parents: Vec<Vec<char>> = input.parents.clone();
    let mut discovered = vec![false; 26];
    let exists = &input.exists;

    let mut frontier = BinaryHeap::new();

    for i in 0..26 {
        if parents[i].len() == 0 && exists[i] {
            frontier.push(-(i as isize));
        }
    }

    while let Some(neg_index) = frontier.pop() {
        let c: char = from_neg_key(neg_index);
        path.push(c);

        for child in &children[to_key(c)] {
            let key = to_key(*child);
            parents[key].remove_item(&c);

            if parents[key].len() == 0 {
                discovered[key] = true;
                frontier.push(to_neg_key(*child));
            }
        }
    }

    path.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1};
    #[test]
    fn examples() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), "CABDFE");
    }
}

