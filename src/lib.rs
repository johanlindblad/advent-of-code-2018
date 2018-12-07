#![feature(vec_remove_item)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate aoc_runner;
extern crate regex;

#[macro_use]
extern crate aoc_runner_derive;
extern crate itertools;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

aoc_lib!{ year = 2018 }
