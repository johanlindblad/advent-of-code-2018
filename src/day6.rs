// TODO: k-d tree?

use std::cmp::Ordering;

type Pair = (usize, usize);
type Point = (usize, Pair);

#[derive(Clone, Debug)]
enum Place {
    Empty,
    Indeterminate(usize),
    DistanceFrom(usize, usize)
}

fn parse_line(line: &str) -> Pair {
    let mut it = line
        .split(", ")
        .map(|s| s.parse::<usize>().unwrap());

    (it.next().unwrap(), it.next().unwrap())
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| (i, parse_line(l)))
        .collect::<Vec<Point>>()
}

fn distance(a: Pair, b: Pair) -> usize {
    let (x1, y1) = a;
    let (x2, y2) = b;

    let distance_x = (x1 as isize - x2 as isize).abs() as usize;
    let distance_y = (y1 as isize - y2 as isize).abs() as usize;
    distance_x + distance_y
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Point]) -> isize {
    let max_x = *input.iter().map(|(_, (x, _))| x).max().unwrap() + 2;
    let max_y = *input.iter().map(|(_, (_, y))| y).max().unwrap() + 2;
    let mut board: Vec<Vec<Place>> = vec![vec![Place::Empty; max_x + 1]; max_y + 1];

    for &point in input.iter() {
        let (index, (point_x, point_y)) = point;
        for x in 0..=max_x {
            for y in 0..=max_y {
                let distance = distance((point_x, point_y), (x, y));

                match board[y][x] {
                    Place::Empty => board[y][x] = Place::DistanceFrom(index, distance),
                    Place::Indeterminate(at_distance) => {
                        if distance < at_distance {
                            board[y][x] = Place::DistanceFrom(index, distance)
                        }
                    },
                    Place::DistanceFrom(_index, existing_distance) => {
                        match distance.cmp(&existing_distance) {
                            Ordering::Less =>
                                board[y][x] = Place::DistanceFrom(index, distance),
                            Ordering::Equal => board[y][x] = Place::Indeterminate(distance),
                            Ordering::Greater => ()
                        }
                    }
                }
            }
        }
    }

    let mut area: Vec<isize> = vec![0; input.len()];

    for x in 0..=max_x {
        for y in 0..=max_y {
            let at_edge = x == 0 || x == max_x || y == 0 || y == max_y;

            match board[y][x] {
                Place::DistanceFrom(index, _) => {
                    if at_edge {
                        area[index] = -1;
                    }
                    else if area[index] > -1 {
                        area[index] += 1;
                    }
                },
                _ => (),
            }
        }
    }

    *area.iter().max().unwrap()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Point]) -> isize {
    let max_x = *input.iter().map(|(_, (x, _))| x).max().unwrap() + 2;
    let max_y = *input.iter().map(|(_, (_, y))| y).max().unwrap() + 2;

    let mut region_size = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let total_distance = input
                .iter()
                .map(|(_, place)| distance((x, y), *place))
                .sum::<usize>();

            if total_distance < 10000 { region_size += 1 }
        }
    }

    region_size
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1};
    #[test]
    fn examples() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), 17);
    }
}

