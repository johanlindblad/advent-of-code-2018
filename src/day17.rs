#[derive(Clone, Debug)]
pub enum Tile { Sand, Clay, Water, Dried }

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Vec<Tile>> {
    let mut ranges: Vec<(usize, usize, usize, usize)> = input.lines().map(|l| {
        let parts: Vec<&str> = l.split(|c| c == '=' || c == ' ' || c == '.' || c == ',').collect();

        if parts[0] == "x" {
            (
                parts[1].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap(),
                parts[4].parse::<usize>().unwrap(), parts[6].parse::<usize>().unwrap()
            )
        } else {
            (
                parts[4].parse::<usize>().unwrap(), parts[6].parse::<usize>().unwrap(),
                parts[1].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap()
            )
        }
    }).collect();
    ranges.sort();
    let sorted_ranges = ranges.clone();

    let (_, max_x, _, _) = sorted_ranges.last().unwrap();
    let max_y = sorted_ranges.iter().map(|(_, _, _, y)| y).max().unwrap();

    let mut board = vec![vec![Tile::Sand; 1 + max_x]; 1 + max_y];

    for (x1, x2, y1, y2) in ranges {
        for x in x1..=x2 {
            for y in y1..=y2 {
                board[y][x] = Tile::Clay;
            }
        }
    }

    board
}

fn drip(from_y: usize, x: usize, board: &mut Vec<Vec<Tile>>) -> bool {
    for y in from_y..board.len() {
        if let Tile::Sand = board[y][x] { board[y][x] = Tile::Dried };

        if let Tile::Dried = board[y][x] {
            board[y][x] = Tile::Dried;
            continue;
        };

        let flow_at = y - 1;
        let mut filled = false;
        let mut overflows_left = true;
        let mut overflows_right = true;

        for steps in 1.. {
            let left = x - steps;

            if let Tile::Clay = board[flow_at][left] {
                overflows_left = false;
                break;
            }

            board[flow_at][left] = Tile::Dried;

            match board[y][left] {
                Tile::Sand | Tile::Dried => {
                    if drip(flow_at, left, board) { filled = true }
                    break;
                }
                _ => ()
            }
        }

        for steps in 0.. {
            let right = x + steps;

            if let Tile::Clay = board[flow_at][right] {
                overflows_right = false;
                break;
            }

            board[flow_at][right] = Tile::Dried;

            match board[y][right] {
                Tile::Sand | Tile::Dried => {
                    if drip(flow_at, right, board) { filled = true }
                    break;
                }
                _ => ()
            }
        }

        if !overflows_left && !overflows_right {
            filled = true;

            for steps in 1.. {
                let left = x - steps;

                if let Tile::Dried = board[flow_at][left] {
                    board[flow_at][left] = Tile::Water;
                } else { break; }
            }

            for steps in 0.. {
                let right = x + steps;

                if let Tile::Dried = board[flow_at][right] {
                    board[flow_at][right] = Tile::Water;
                } else { break; }
            }
        }

        return filled;
    }

    false
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Vec<Vec<Tile>>) -> isize {
    let mut board = input.clone();


    loop {
        if !drip(0, 500, &mut board) {
            break;
        }
    }

    let mut sum = 0;
    board[0][500] = Tile::Sand;
    let mut has_something = false;

    for i in 0..(board.len()) {
        let row = &board[i];


        for tile in row {
            match tile {
                Tile::Clay => has_something = true,
                _ => ()
            }
        }

        if !has_something { continue }

        for tile in row {
            match tile {
                Tile::Water | Tile::Dried => sum += 1,
                _other => ()
            }
        }
    }

    sum
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Vec<Vec<Tile>>) -> isize {
    let mut board = input.clone();


    loop {
        if !drip(0, 500, &mut board) {
            break;
        }
    }

    let mut sum = 0;
    board[0][500] = Tile::Sand;
    let mut has_something = false;

    for i in 0..board.len() {
        let row = &board[i];


        for tile in row {
            match tile {
                Tile::Clay => has_something = true,
                _ => ()
            }
        }

        if !has_something { continue }

        for tile in row {
            match tile {
                Tile::Water => sum += 1,
                _other => ()
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, input_generator};

    #[test]
    fn examples() {
        let raw = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

        assert_eq!(solve_part1(&input_generator(raw)), 57);
        assert_eq!(solve_part2(&input_generator(raw)), 29);
    }
}

