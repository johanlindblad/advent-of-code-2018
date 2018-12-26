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
    let mut visited: Vec<bool> = vec![true];
    let mut visited_neg: Vec<bool> = vec![false];
    let mut sum = 0;

    for i in input.iter().cycle() {
        sum += i;

        if sum >= 0 {
            let pos = sum as usize;

            if visited.len() <= pos {
                visited.resize(pos + 1, false);
            }

            if visited[pos] {
                return sum;
            }

            visited[pos] = true;
        } else {
            let opposite = (-sum) as usize;

            if visited_neg.len() <= opposite {
                visited_neg.resize(opposite + 1, false);
            }

            if visited_neg[opposite] {
                return sum;
            }

            visited_neg[opposite] = true;
        }
    }

    panic!("{}", "Should not get here");
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};
    #[test]
    fn examples() {
        assert_eq!(solve_part1(&[1,1,1]), 3);
        assert_eq!(solve_part1(&[1,1,-2]), 0);
        assert_eq!(solve_part1(&[-1, -2, -3]), -6);
        assert_eq!(solve_part2(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(solve_part2(&[7, 7, -2, -7, -4]), 14);
    }
}
