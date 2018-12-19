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

    let mut elfs_left = 0;
    let mut goblins_left = 0;

    for row in 0..height {
        for column in 0..width {
            match &board[row][column] {
                Square::Occupied(by) => {
                    unit_positions.insert((row, column));
                    if by.friendly { elfs_left += 1 } else { goblins_left += 1};
                },
                _ => ()
            };
        }
    }

    let adjacent: Vec<(isize, isize)> = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

    let mut full_round = true;

    for round in 1.. {
        let mut new_positions: BTreeSet<(usize, usize)> = BTreeSet::new();

        for (mut row, mut column) in unit_positions {
            let friendly = match &board[row][column] {
                Square::Wall | Square::Space => continue,
                Square::Occupied(by) => by.friendly
            };

            if goblins_left == 0 || elfs_left == 0 {
                full_round = false;
                break;
            }

            let mut found_enemy: Option<(usize, usize)> = None;
            let mut closest = usize::max_value();
            let mut move_to: Option<(usize, usize)> = None;
            let mut lowest_hit_points = 201;

            for (dy, dx) in &adjacent {
                let mut frontier: VecDeque<(usize, (usize, usize))> = VecDeque::new();
                let first_y = (row as isize + dy) as usize;
                let first_x = (column as isize + dx) as usize;
                let mut visited = vec![vec![false; width]; height];

                if let Square::Space = &board[first_y][first_x] {
                    frontier.push_back((1, (first_y, first_x)));

                    while let Some((distance, (y, x))) = frontier.pop_front() {
                        if distance > closest { break };

                        for (dy, dx) in &adjacent {
                            let new_y = (y as isize + dy) as usize;
                            let new_x = (x as isize + dx) as usize;

                            if let Square::Space = board[new_y][new_x] {
                                if visited[new_y][new_x] { continue };
                                visited[new_y][new_x] = true;
                                frontier.push_back((distance + 1, (new_y, new_x)));
                            } else if let Square::Occupied(ref by) = board[new_y][new_x] {
                                if by.friendly == friendly { continue }
                                if found_enemy.is_none() || (y, x) < found_enemy.unwrap() || distance < closest {
                                    println!("Can make it to {:?} in {} with {:?}", (y, x), distance, (first_y, first_x));
                                    found_enemy = Some((y, x));
                                    closest = distance;
                                    move_to = Some((first_y, first_x));
                                }
                            }
                        }
                    }
                } else if let Square::Occupied(ref by) = board[first_y][first_x] {
                    if by.friendly == friendly { continue }
                    closest = 0;
                    move_to = None;
                    break;
                }
            }

            if let Some((new_y, new_x)) = move_to {
                let unit = board[row][column].clone();
                board[new_y][new_x] = unit.clone();
                board[row][column] = Square::Space;
                new_positions.insert((new_y, new_x));
                row = new_y;
                column = new_x;
            } else { new_positions.insert((row, column)); }

            let mut attack_at: Option<(usize, usize)> = None;
            if closest <= 1 {
                let mut lowest_points = 201;

                for (dy, dx) in &adjacent {
                    let ny = (row as isize + *dy) as usize;
                    let nx = (column as isize + *dx) as usize;

                    if let Square::Occupied(by) = &board[ny][nx] {
                        if by.friendly != friendly && by.hit_points < lowest_points {
                            attack_at = Some((ny, nx));
                            lowest_points = by.hit_points;
                        }
                    }
                }
            }

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

                    if friendly { goblins_left -=1 } else { elfs_left -= 1 };
                }
            }
        }

        unit_positions = new_positions;

        if elfs_left == 0 || goblins_left == 0 {
            let mut outcome = 0usize;

            for row in &board {
                for column in row {
                    if let Square::Occupied(unit) = column {
                        outcome += unit.hit_points as usize;
                    }
                }
            }

            if full_round { return outcome * round };
            return outcome * (round - 1);
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

        let raw3 = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

        let input3 = input_generator(raw3);
        assert_eq!(solve_part1(&input3), 18740);
    }
}

