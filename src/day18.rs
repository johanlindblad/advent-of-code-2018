use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Tile { Tree, Lumberyard, Open, Null }

fn char_to_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Open,
        '|' => Tile::Tree,
        '#' => Tile::Lumberyard,
        _ => panic!("Unexpected {}", c)
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(|l|
        l.chars().map(|c| char_to_tile(c)).collect()
    ).collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Vec<Vec<Tile>>) -> isize {
    let mut area = input.clone();
    let height = area.len();
    let width = area[0].len();

    let get = |area: &Vec<Vec<Tile>>, y: isize, x: isize| -> Tile {
        if x >= 0 && y >= 0 && y < height as isize && x < width as isize {
            area[y as usize][x as usize].clone()
        } else {
            Tile::Null
        }
    };

    for _min in 1..=10 {
        let mut next = area.clone();

        for y in 0..area.len() {
            for x in 0..area[0].len() {
                let adjacent = [
                    get(&area, y as isize - 1, x as isize - 1), get(&area, y as isize - 1, x as isize),
                    get(&area, y as isize - 1, x as isize + 1), get(&area, y as isize, x as isize - 1),
                    get(&area, y as isize, x as isize + 1), get(&area, y as isize + 1, x as isize - 1),
                    get(&area, y as isize + 1, x as isize), get(&area, y as isize + 1, x as isize + 1)
                ];

                let (tree, lumber) = adjacent.iter().fold((0, 0), |(tree, lumber), tile| (
                        match tile {
                            Tile::Tree => (tree + 1, lumber),
                            Tile::Lumberyard => (tree, lumber + 1),
                            _ => (tree, lumber)
                        }
                ));

                match area[y][x] {
                    Tile::Open => {
                        if tree >= 3 {
                            next[y][x] = Tile::Tree
                        }
                    },
                    Tile::Tree => {
                        if lumber >= 3 {
                            next[y][x] = Tile::Lumberyard
                        }
                    },
                    Tile::Lumberyard => {
                        if lumber >= 1 && tree >= 1 {
                            next[y][x] = Tile::Lumberyard
                        } else {
                            next[y][x] = Tile::Open
                        }
                    },
                    _ => panic!("No null tiles should exist")
                }
            }
        }

        area = next;
    }

    let (tree, lumber) = area.iter().map(|row| (
        row.iter().fold((0, 0), |(tree, lumber), tile| (
            match tile {
                Tile::Tree => (tree + 1, lumber),
                Tile::Lumberyard => (tree, lumber + 1),
                _ => (tree, lumber)
            }
        ))
    )).fold((0, 0), |(tree, lumber), (row_t, row_l)| (tree + row_t, lumber + row_l));;

    tree * lumber
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Vec<Vec<Tile>>) -> isize {
    let mut area = input.clone();
    let height = area.len();
    let width = area[0].len();

    let get = |area: &Vec<Vec<Tile>>, y: isize, x: isize| -> Tile {
        if x >= 0 && y >= 0 && y < height as isize && x < width as isize {
            area[y as usize][x as usize].clone()
        } else {
            Tile::Null
        }
    };

    let mut seen_at: HashMap<String, usize> = HashMap::new();

    let mut stop_at = 0;
    let mut stopping = false;

    for minute in 1..=1000000000 {
        let mut next = area.clone();

        for y in 0..area.len() {
            for x in 0..area[0].len() {
                let adjacent = [
                    get(&area, y as isize - 1, x as isize - 1), get(&area, y as isize - 1, x as isize),
                    get(&area, y as isize - 1, x as isize + 1), get(&area, y as isize, x as isize - 1),
                    get(&area, y as isize, x as isize + 1), get(&area, y as isize + 1, x as isize - 1),
                    get(&area, y as isize + 1, x as isize), get(&area, y as isize + 1, x as isize + 1)
                ];

                let (tree, lumber) = adjacent.iter().fold((0, 0), |(tree, lumber), tile| (
                        match tile {
                            Tile::Tree => (tree + 1, lumber),
                            Tile::Lumberyard => (tree, lumber + 1),
                            _ => (tree, lumber)
                        }
                ));

                match area[y][x] {
                    Tile::Open => {
                        if tree >= 3 {
                            next[y][x] = Tile::Tree
                        }
                    },
                    Tile::Tree => {
                        if lumber >= 3 {
                            next[y][x] = Tile::Lumberyard
                        }
                    },
                    Tile::Lumberyard => {
                        if lumber >= 1 && tree >= 1 {
                            next[y][x] = Tile::Lumberyard
                        } else {
                            next[y][x] = Tile::Open
                        }
                    },
                    _ => panic!("No null tiles should exist")
                }
            }
        }

        area = next;

        let dump = area.clone().iter().map(|row| (
            row.iter().map(|c| match c {
                Tile::Open => '.',
                Tile::Tree => '|',
                Tile::Lumberyard => '#',
                Tile::Null => ' '
            }).collect::<String>()
        )).collect::<Vec<String>>().join("\n");

        if stopping {
            if minute == stop_at { break; }
            continue;
        }

        if seen_at.contains_key(&dump) {
            let start = seen_at.get(&dump).unwrap().clone();
            let period = minute - start;
            let remaining = 1000000000 - minute;
            let into_cycle = remaining % period;
            stop_at = minute + into_cycle;
            stopping = true;
        }

        seen_at.insert(dump, minute);
    }

    let (tree, lumber) = area.iter().map(|row| (
        row.iter().fold((0, 0), |(tree, lumber), tile| (
            match tile {
                Tile::Tree => (tree + 1, lumber),
                Tile::Lumberyard => (tree, lumber + 1),
                _ => (tree, lumber)
            }
        ))
    )).fold((0, 0), |(tree, lumber), (row_t, row_l)| (tree + row_t, lumber + row_l));;

    tree * lumber
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, input_generator};

    #[test]
    fn examples() {
        let raw = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

        assert_eq!(solve_part1(&input_generator(raw)), 1147);
    }
}

