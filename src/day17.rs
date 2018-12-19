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

#[aoc(day17, part1)]
pub fn solve_part1(input: &Vec<Vec<Tile>>) -> isize {
    let mut board = input.clone();

    let mut stack: Vec<(usize, usize)> = vec![(500, 1)];

    while let Some((x, y)) = stack.pop() {
        let current = board[y][x].clone();
        println!("{},{}", y, x);
        match current {
            Tile::Sand | Tile::Dried => {
                board[y][x] = Tile::Dried;

                if y != (board.len() - 1) {
                    if let Tile::Water = board[y+1][x] {
                        if let Tile::Dried = board[y][x+1] {
                            stack.push((x+1, y));
                        }
                        if let Tile::Dried = board[y][x-1] {
                            stack.push((x-1, y));
                        }
                    } else {
                        stack.push((x, y+1));
                    }
                }
                continue;
            },
            Tile::Clay | Tile::Water => {
                let fill_y = y-1;
                let mut stays = true;
                let mut overflows_left = false;
                let mut overflows_right = false;

                let mut left_x = x;
                let mut right_x = x;

                for dx in 1.. {
                    if x+dx >= board[0].len() {
                        for i in 0..board.len() {
                            board[i].push(Tile::Sand);
                        }
                    }

                    match (&board[fill_y][x+dx], &board[fill_y+1][x+dx]) {
                        (Tile::Clay, _) => {
                            right_x = x+dx;
                            break;
                        },
                        (other, Tile::Sand) => {
                            println!("Doesn't stay because of ({:?},{:?}) at {},{}", other, Tile::Sand, x+dx, fill_y+1);
                            stays = false;
                            right_x = x+dx+1;
                            overflows_right = true;
                            break;
                        },
                        (Tile::Sand, Tile::Clay) | (Tile::Sand, Tile::Water) | (Tile::Dried, Tile::Clay) | (Tile::Dried, Tile::Water) => (),
                        (right, under) => {
                            for yy in 0..board.len() {
                                let row = &board[yy];
                                println!("{}", &row[x-100..=x+100].iter().map(|t| match t {
                                    Tile::Sand => '.',
                                    Tile::Clay => '#',
                                    Tile::Water => '~',
                                    Tile::Dried => '|'
                                }).collect::<String>());
                            }

                            panic!("Unexpected {:?}, {:?} at {},{} (max: {})", right, under, x, y, board.len());
                        }
                    }
                }

                for dx in 1.. {
                    match (&board[fill_y][x-dx], &board[fill_y+1][x-dx]) {
                        (Tile::Clay, _) => {
                            left_x = x-dx;
                            break;
                        },
                        (_, Tile::Sand) => {
                            stays = false;
                            left_x = x-dx-1;
                            overflows_left = true;
                            break;
                        },
                        (Tile::Sand, Tile::Clay) | (Tile::Sand, Tile::Water) | (Tile::Dried, Tile::Clay) | (Tile::Dried, Tile::Water) => (),
                        (right, under) => {
                            for yy in 0..board.len() {
                                let row = &board[yy];
                                println!("{}", &row[x-100..=x+100].iter().map(|t| match t {
                                    Tile::Sand => '.',
                                    Tile::Clay => '#',
                                    Tile::Water => '~',
                                    Tile::Dried => '|'
                                }).collect::<String>());
                            }

                            let row = &board[y];
                            println!("{}", &row[x-2..=x+2].iter().map(|t| match t {
                                Tile::Sand => '.',
                                Tile::Clay => '#',
                                Tile::Water => '~',
                                Tile::Dried => '|'
                            }).collect::<String>());
                            panic!("Unexpected {:?}, {:?} at {},{} (max: {})", right, under, x, y, board.len());
                        }
                    }
                }

                println!("{}-{}, {}", left_x, right_x, stays);


                if stays {
                    for x in (left_x+1)..=(right_x-1) {
                        board[fill_y][x] = Tile::Water;
                    }

                    stack.push((x, fill_y));
                } else {
                    for x in (left_x+1)..=(right_x-1) {
                        board[fill_y][x] = Tile::Dried;
                    }

                    if overflows_left {
                        stack.push((left_x + 1, y));
                    }
                    if overflows_right {
                        stack.push((right_x - 1, y));
                    }
                }
            },
            ref other => panic!("Unhandled {:?}", other)
        };

        /*for row in &board {
            println!("{}", &row[400..=600].iter().map(|t| match t {
                Tile::Sand => '.',
                Tile::Clay => '#',
                Tile::Water => '~',
                Tile::Dried => '|'
            }).collect::<String>());
        }
        println!("");
        println!("");*/
            /*println!("{:?}", &board[fill_y-1][495..505]);
            println!("{:?}", &board[fill_y][495..505]);
            println!("{:?}", &board[fill_y+1][495..505]);*/
    };

    let mut sum = 0;

    for row in &board {
        for tile in row {
            match tile {
                Tile::Water | Tile::Dried => sum += 1,
                other => ()
            }
        }
        println!("{}", &row.iter().map(|t| match t {
            Tile::Sand => '.',
            Tile::Clay => '#',
            Tile::Water => '~',
            Tile::Dried => '|'
        }).collect::<String>());
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, input_generator};

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
    }
}

