use std::cell::RefCell;

pub struct Point {
    position: (isize, isize),
    velocity: (isize, isize)
}

impl Point {
    fn step(&mut self) {
        let (x, y) = self.position;
        let (xv, yv) = self.velocity;
        self.position = (x + xv, y + yv);
    }

    fn parse(input: &str) -> Point {
        let parts = input
                    .split(|c| c == '<' || c == ',' || c == '>')
                    .collect::<Vec<&str>>();
        let selected = [parts[1], parts[2], parts[4], parts[5]].iter()
            .map(|p| p.trim().parse::<isize>().unwrap()).collect::<Vec<isize>>();

        let (x, y) = (selected[0], selected[1]);
        let (xv, yv) = (selected[2], selected[3]);

        Self { position: (x, y), velocity: (xv, yv) }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<RefCell<Point>> {
    input
        .lines()
        .map(|l| RefCell::new(Point::parse(l)))
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[RefCell<Point>]) -> usize {
    let points: &[RefCell<Point>] = input.clone();

    let mut min_width = usize::max_value();

    for time in 1..100000 {
        for i in 0..points.len() {
            points[i].borrow_mut().step()
        }
        let min_x = points.iter().map(|p| p.borrow().position.0).min().unwrap();
        let min_y = points.iter().map(|p| p.borrow().position.1).min().unwrap();
        let max_x = points.iter().map(|p| p.borrow().position.0).max().unwrap();
        let _max_y = points.iter().map(|p| p.borrow().position.1).max().unwrap();

        let width = (max_x - min_x).abs() as usize;
        let height = (max_x - min_x).abs() as usize;

        if width < 100 {
            let mut output = vec![vec![" "; width + 1]; height + 1];
            for point in points {
                let (x, y) = point.borrow().position;
                output[(y - min_y) as usize][(x - min_x) as usize] = "#";
            }

            println!("After {} seconds", time);
            for line in output {
                println!("{}", line.join(""));
            }
            println!("");
        }

        if width < min_width {
            min_width = width;
        } else {
            break
        }
    }
    5
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1};
    #[test]
    fn examples() {
    }
}


