pub fn power_level(x: usize, y: usize, serial: usize) -> isize {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    let third_digit = (power_level / 100) % 10;
    third_digit as isize - 5
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> String {
    let serial = input.trim().parse::<usize>().unwrap();

    let mut grid = [[0; 300]; 300];

    for x in 1..=300 {
        for y in 1..=300 {
            grid[x-1][y-1] = power_level(x, y, serial);
        }
    }

    let mut best_square = (0, 0);
    let mut best_score = isize::min_value();

    for x in 0..298 {
        for y in 0..298 {
            let score = grid[x][y]+grid[x+1][y]+grid[x+2][y]+
                        grid[x][y+1]+grid[x+1][y+1]+grid[x+2][y+1]+
                        grid[x][y+2]+grid[x+1][y+2]+grid[x+2][y+2];

            if score > best_score {
                best_score = score;
                best_square = (x+1, y+1);
            }
        }
    }

    format!("{},{}", best_square.0, best_square.1)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> String {
    let serial = input.trim().parse::<usize>().unwrap();

    let mut grid = [[0; 300]; 300];

    for x in 1..=300 {
        for y in 1..=300 {
            grid[x-1][y-1] = power_level(x, y, serial);
        }
    }

    let mut best_square = (0, 0, 0);
    let mut best_score = isize::min_value();

    for start_x in 0..300 {
        for start_y in 0..300 {
            let max_size = isize::min(300 - start_x, 300 - start_y);
            let mut score = grid[start_x as usize][start_y as usize];

            for size in 1..=max_size {
                for x in (start_x)..(start_x+size-1) {
                    score += grid[x as usize][(start_y + size - 1) as usize];
                }
                for y in (start_y)..(start_y+size-1) {
                    score += grid[(start_x + size - 1) as usize][y as usize];
                }
                score += grid[(start_x + size - 1) as usize][(start_y + size - 1) as usize];

                if score > best_score {
                    best_score = score;
                    best_square = (start_x+1, start_y+1, size);
                }
            }
        }
    }

    format!("{},{},{}", best_square.0, best_square.1, best_square.2)
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, power_level};
    #[test]
    fn examples() {
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}

