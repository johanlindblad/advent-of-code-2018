use std::boxed::Box;
use std::collections::VecDeque;

// TODO: separate list for elements moved from back?

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Box<(usize, usize)> {
    let mut iter = input.split(" ");
    let num_players = iter.next().unwrap().parse::<usize>().unwrap();
    let highest_marble = iter.skip(5).next().unwrap().parse::<usize>().unwrap();

    Box::new((num_players, highest_marble))
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &(usize, usize)) -> usize {
    let mut board: VecDeque<usize> = VecDeque::new();
    board.push_back(0);
    let (num_players, highest_marble) = *input;
    let mut scores = vec![0; num_players];

    for marble in 1..=highest_marble {
        let player = (marble - 1) % num_players;

        if (marble % 23) > 0 {
            let popped = board.pop_front().unwrap();
            board.push_back(popped);

            board.push_back(marble);
        } else {
            for _i in 0..7 {
                let popped = board.pop_back().unwrap();
                board.push_front(popped)
            }

            let picked_out = board.pop_back().unwrap();
            scores[player] += marble + picked_out;
            let popped = board.pop_front().unwrap();
            board.push_back(popped)
        }
    }

    *scores.iter().max().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &(usize, usize)) -> usize {
    let (num_players, highest_marble) = *input;
    solve_part1(&(num_players, highest_marble * 100))
}

#[cfg(test)]
mod tests {
    use super::{solve_part1};
    #[test]
    fn examples() {
        assert_eq!(solve_part1(&(9, 25)), 32);
        assert_eq!(solve_part1(&(10, 1618)), 8317);
    }
}

