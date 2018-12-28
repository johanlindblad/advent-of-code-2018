#[derive(Debug, Clone)]
pub enum Piece { Intersection, Horizontal, Vertical, CurveRight, CurveLeft, Empty }
#[derive(Debug, Clone)]
pub enum Direction { Up, Down, Left, Right }
#[derive(Debug, Clone, Copy)]
pub enum Choice { Left, Straight, Right }
pub type Board = Vec<Vec<Piece>>;
pub type Elves = Vec<Elf>;

fn next_choice(current: Choice) -> Choice {
    match current {
        Choice::Left => Choice::Straight,
        Choice::Straight =>Choice::Right,
        Choice::Right => Choice::Left
    }
}

fn intersection_result(current: Direction, choice: Choice) -> Direction {
    match (current, choice) {
        (Direction::Up, Choice::Left) | (Direction::Down, Choice::Right) | (Direction::Left, Choice::Straight) => Direction::Left,
        (Direction::Up, Choice::Right) | (Direction::Down, Choice::Left) | (Direction::Right, Choice::Straight) => Direction::Right,
        (Direction::Left, Choice::Right) | (Direction::Right, Choice::Left) | (Direction::Up, Choice::Straight) => Direction::Up,
        (Direction::Left, Choice::Left) | (Direction::Right, Choice::Right) | (Direction::Down, Choice::Straight) => Direction::Down
    }
}

fn turn_result(current: Direction, turn: Piece) -> Direction {
    match (current, turn) {
        (Direction::Up, Piece::CurveRight) | (Direction::Down, Piece::CurveLeft) => Direction::Right,
        (Direction::Up, Piece::CurveLeft) | (Direction::Down, Piece::CurveRight) => Direction::Left,
        (Direction::Left, Piece::CurveLeft) | (Direction::Right, Piece::CurveRight) => Direction::Up,
        (Direction::Left, Piece::CurveRight) | (Direction::Right, Piece::CurveLeft) => Direction::Down,
        _ => panic!("Unexpected")
    }
}

#[derive(Debug, Clone)]
pub struct Elf {
    position: (usize, usize),
    direction: Direction,
    next_choice: Choice,
    id: usize
}

fn parse_line(y: usize, line: &str, line_len: usize) -> (Vec<Piece>, Elves) {
    let mut l = line.to_string();

    // cargo-aoc trims input
    if y == 0 {
        let remains = line_len - l.len();
        for _ in 0..remains { l.insert(0, ' '); }
    }

    let piece = |c: char| -> Piece {
        match c {
            ' ' => Piece::Empty,
            '/' => Piece::CurveRight,
            '\\' => Piece::CurveLeft,
            '+' => Piece::Intersection,
            '-' | '>' | '<' => Piece::Horizontal,
            '|' | 'v' | '^' => Piece::Vertical,
            other => panic!("Unexpected {}", other)
        }
    };

    let elf = |c: char, position: (usize, usize)| -> Option<Elf> {
        match c {
            'v' => Some(Elf { id: 0, position, next_choice: Choice::Left, direction: Direction::Down }),
            '<' => Some(Elf { id: 0, position, next_choice: Choice::Left, direction: Direction::Left }),
            '>' => Some(Elf { id: 0, position, next_choice: Choice::Left, direction: Direction::Right }),
            '^' => Some(Elf { id: 0, position, next_choice: Choice::Left, direction: Direction::Up }),
            _ => None
        }
    };

    let pieces = l.chars().map(|c| piece(c)).collect();
    let mut elves: Vec<Elf> = l
        .chars()
        .enumerate()
        .map(|(x, c)| elf(c, (y, x)))
        .filter(|ref e| e.is_some())
        .map(|e| e.unwrap())
        .collect();

    elves.sort_by_key(|e| e.position);

    (pieces, elves)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Box<(Board, Elves)> {
    let mut board: Vec<Vec<Piece>> = Vec::new();
    let mut elves: Elves = Vec::new();

    let line_len = input.lines().skip(1).next().unwrap().len();

    input
        .lines()
        .enumerate()
        .map(|(y, l)| parse_line(y, l, line_len))
        .for_each(|(row, mut new_elves)| {
            board.push(row);
            elves.append(&mut new_elves);
        });

    elves.iter_mut().enumerate().for_each(|(i, ref mut e)| e.id = i );

    Box::new((board, elves))
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(Board, Elves)) -> String {
    let ((y, x), _) = solve(input);
    format!("{},{}", x, y)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(Board, Elves)) -> String {
    let (_, (y, x)) = solve(input);
    format!("{},{}", x, y)
}

pub fn solve(input: &(Board, Elves)) -> ((usize, usize), (usize, usize)) {
    let board = &input.0;
    let mut elves = input.1.clone();
    let num_elves = elves.len();
    let width = board[0].len();
    let height = board.len();

    let mut occupied_by = vec![vec![None; width]; height];

    for elf in &elves {
        let (y, x) = elf.position;
        occupied_by[y][x] = Some(elf.id);
    }

    let mut first = None;

    loop {
        let mut crashes = vec![false; num_elves];

        for ref mut elf in &mut elves {
            if crashes[elf.id] { continue }
            let (y, x) = elf.position;

            let (ny, nx) = match elf.direction {
                Direction::Left => (y, x-1),
                Direction::Right => (y, x+1),
                Direction::Up => (y-1, x),
                Direction::Down => (y+1, x)
            };

            match occupied_by[ny][nx] {
                None => {
                    occupied_by[ny][nx] = Some(elf.id);
                    occupied_by[y][x] = None;
                    elf.position = (ny, nx);

                    let direction = elf.direction.clone();
                    let choice = elf.next_choice.clone();
                    let new_piece = board[ny][nx].clone();

                    let (new_direction, next_choice) = match new_piece {
                        Piece::Intersection => (intersection_result(direction, choice), next_choice(choice)),
                        p @ Piece::CurveLeft | p @ Piece::CurveRight => (turn_result(direction, p), choice),
                        _  => (direction, choice)
                    };

                    elf.direction = new_direction;
                    elf.next_choice = next_choice;
                },
                Some(other_id) => {
                    crashes[other_id] = true;
                    crashes[elf.id] = true;
                    occupied_by[y][x] = None;
                    occupied_by[ny][nx] = None;
                    if first.is_none() { first = Some((ny, nx)) }
                }
            }
        }

        elves.sort_by_key(|e| e.position);
        elves = elves.into_iter().filter(|ref e| crashes[e.id] == false).collect::<Vec<Elf>>();

        if elves.len() == 1 {
            let (y, x) = elves[0].position;
            return (first.unwrap(), (y, x));
        } else if elves.len() == 0 {
            // Needed because some examples never end up with a lone elf
            return (first.unwrap(), (0, 0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};
    #[test]
    fn examples() {
        let raw = "".to_owned() +
"/->-\\        \n" +
"|   |  /----\\\n" +
"| /-+--+-\\  |\n" +
"| | |  | v  |\n" +
"\\-+-/  \\-+--/\n" +
"  \\------/   \n";

        assert_eq!(solve_part1(&input_generator(&raw)), "7,3");

        let raw2 = "".to_owned() +
"/>-<\\  \n" +
"|   |  \n" +
"| /<+-\\\n" +
"| | | v\n" +
"\\>+</ |\n" +
"  |   ^\n" +
"  \\<->/";

        assert_eq!(solve_part2(&input_generator(&raw2)), "6,4");
    }
}

