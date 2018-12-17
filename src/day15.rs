use std::collections::BTreeSet;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Unit {
    hit_points: isize,
    friendly: bool
}

#[derive(Debug, Clone)]
pub enum Square { Wall, Space, Occupied(Unit) }


fn parse_line(input: &str) -> Vec<Square> {
    input.chars().map(|c| match c {
        '#' => Square::Wall,
        '.' => Square::Space,
        'E' | 'G' => Square::Occupied(Unit { hit_points: 200, friendly: c == 'E' }),
        other => panic!("Unexpected {}", other)
    }).collect()
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Vec<Square>> {
    input.lines().map(|l| parse_line(l)).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Vec<Vec<Square>>) -> usize {
    let mut unit_positions: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut board: Vec<Vec<Square>> = input.clone().to_vec();
    let (height, width) = (board.len(), board[0].len());

    for row in 0..height {
        for column in 0..width {
            match &board[row][column] {
                Square::Occupied(_) => unit_positions.insert((row, column)),
                _ => true
            };
        }
    }

    let adjacent: Vec<(isize, isize)> = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

    for round in 1.. {
        let mut new_positions: BTreeSet<(usize, usize)> = BTreeSet::new();

        for (row, column) in unit_positions {
            let mut attack_at: Option<(usize, usize)> = None;
            let mut should_move = true;
            let mut friendly = false;

            if let Square::Occupied(unit) = &board[row][column] {
                friendly = unit.friendly;
                if unit.hit_points <= 0 { continue }

                let mut lowest = 201;

                for (dy, dx) in &adjacent {
                    let y = (row as isize + dy) as usize;
                    let x = (column as isize + dx) as usize;

                    if let Square::Occupied(unit) = &board[y][x] {
                        if unit.friendly == !friendly {

                            match attack_at {
                                None => attack_at = Some((y, x)),
                                Some(_) => {
                                    if unit.hit_points < lowest {
                                        attack_at = Some((y, x));
                                        lowest = unit.hit_points;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                continue;
            }

            if let None = attack_at {
                let mut new_position: Option<(usize, usize)> = None;
                let mut frontier: VecDeque<((usize, usize), Option<(isize, isize)>)> = VecDeque::new();
                let mut considered = vec![vec![false; width]; height];

                frontier.push_back(((row, column), None));

                while let Some(((y, x), first_move)) = frontier.pop_front() {
                    for (dy, dx) in &adjacent {
                        let ny = (y as isize + dy) as usize;
                        let nx = (x as isize + dx) as usize;
                        if let Square::Space = board[ny][nx] {
                            let mut first = first_move;
                            if let None = first { first = Some((*dy, *dx)) }
                            let new_position = (ny, nx);
                            if considered[ny][nx] { continue }
                            frontier.push_back((new_position, first));
                            considered[ny][nx] = true;
                        } else if let Square::Occupied(by) = &board[ny][nx] {
                            if friendly != by.friendly {
                                let mut first = first_move;
                                if let None = first { first = Some((*dy, *dx)) }
                                let (dy, dx) = first.unwrap();
                                let ny = (row as isize + dy) as usize;
                                let nx = (column as isize + dx) as usize;
                                new_position = Some((ny, nx));
                                frontier.clear();
                            }
                        }
                    }
                }

                if let Some((ny, nx)) = new_position {
                    let unit = board[row][column].clone();
                    board[ny][nx] = unit.clone();
                    board[row][column] = Square::Space;
                    new_positions.insert((ny, nx));
                    let mut lowest = 201;
                    for (dy, dx) in &adjacent {
                        let y = (ny as isize + dy) as usize;
                        let x = (nx as isize + dx) as usize;

                        if let Square::Occupied(unit) = &board[y][x] {
                            if unit.friendly == !friendly {
                                match attack_at {
                                    None => attack_at = Some((y, x)),
                                    Some(_) => {
                                        if unit.hit_points < lowest {
                                            attack_at = Some((y, x));
                                            lowest = unit.hit_points;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else { new_positions.insert((row, column)); }
            } else { new_positions.insert((row, column)); }

            if let Some((y, x)) = attack_at {
                let mut killed = false;

                if let &mut Square::Occupied(ref mut target) = &mut board[y][x] {
                    target.hit_points -= 3;
                    if target.hit_points <= 0 {
                        killed = true;
                    }
                } else { panic!("WTF") }

                if killed {
                    board[y][x] = Square::Space;
                    new_positions.remove(&(y, x));
                }
            }
        }

        unit_positions = new_positions;

        let mut found_elf = false;
        let mut found_goblin = false;

        for (row, column) in &unit_positions {
            if let Square::Occupied(ref unit) = board[*row][*column] {
                if unit.friendly { found_elf = true }
                else { found_goblin = true }
            }
        }

        for row in &board {
            println!("{}", row.iter().map(|s| match s {
                Square::Occupied(unit) => if unit.friendly { 'E' } else { 'G' },
                Square::Space => '.',
                Square::Wall => '#'
            }).collect::<String>());
        }
        println!("");

        if found_elf != found_goblin {
            let mut outcome = 0usize;

            for row in &board {
                for column in row {
                    if let Square::Occupied(unit) = column {
                        outcome += unit.hit_points as usize;
                    }
                }
            }

            return outcome * round;
        }
    }

    5
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, input_generator};

    #[test]
    fn examples() {
        let raw = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let input = input_generator(raw);
        assert_eq!(solve_part1(&input), 27730);

        let raw2 = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

        let input2 = input_generator(raw2);
        assert_eq!(solve_part1(&input2), 36334);
    }
}

