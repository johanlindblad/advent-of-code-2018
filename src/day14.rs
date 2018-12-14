#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &usize) -> String {
    let after: usize = *input;

    let mut scoreboard: Vec<usize> = vec![3, 7];
    let mut positions = (0, 1);

    for _i in 0..(after + 10) {
        let (current1, current2) = (scoreboard[positions.0], scoreboard[positions.1]);
        let current_score = current1 + current2;

        if current_score >= 10 {
            scoreboard.push(1);
        }
        scoreboard.push(current_score % 10);

        positions = (
            (positions.0 + 1 + current1) % scoreboard.len(),
            (positions.1 + 1 + current2) % scoreboard.len()
        );

    }

    let next_ten = &scoreboard[after..after + 10];
    let concat = next_ten
        .iter()
        .enumerate()
        .map(|(i, &n)| n as isize * (10 as isize).pow(9 - i as u32))
        .fold(0, |acc, n| acc + n);

    format!("{:010}", concat)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &usize) -> usize {
    let after: usize = *input;
    let after_len = format!("{}", after).len();

    let mut scoreboard: Vec<usize> = vec![3, 7];
    let mut positions = (0, 1);

    let mut sum = 37;
    let modulo = (10 as isize).pow(after_len as u32);

    loop {
        let (current1, current2) = (scoreboard[positions.0], scoreboard[positions.1]);
        let current_score = current1 + current2;

        if current_score >= 10 {
            scoreboard.push(1);
            sum = ((sum * 10) + 1) % modulo;

            if sum == after as isize {
                return scoreboard.len() - after_len
            }
        }
        let score = current_score % 10;
        scoreboard.push(score);
        sum = ((sum * 10) + score as isize) % modulo;

        positions = (
            (positions.0 + 1 + current1) % scoreboard.len(),
            (positions.1 + 1 + current2) % scoreboard.len()
        );

        if sum == after as isize {
            return scoreboard.len() - after_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn examples() {
        assert_eq!(solve_part1(&5), "0124515891");
        assert_eq!(solve_part1(&18), "9251071085");
        assert_eq!(solve_part1(&2018), "5941429882");
        assert_eq!(solve_part2(&51589), 9);
        assert_eq!(solve_part2(&92510), 18);
        assert_eq!(solve_part2(&59414), 2018);
    }
}

