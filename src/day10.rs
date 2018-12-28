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

fn solve(points: &[RefCell<Point>]) -> (String, usize) {
    let mut min_width = usize::max_value();
    let mut last_output = "".to_string();

    for time in 1..100000 {
        for i in 0..points.len() {
            points[i].borrow_mut().step()
        }
        let min_x = points.iter().map(|p| p.borrow().position.0).min().unwrap();
        let min_y = points.iter().map(|p| p.borrow().position.1).min().unwrap();
        let max_x = points.iter().map(|p| p.borrow().position.0).max().unwrap();
        let max_y = points.iter().map(|p| p.borrow().position.1).max().unwrap();

        let width = (max_x - min_x).abs() as usize;
        let height = (max_y - min_y).abs() as usize;
        let mut this_output = "\n".to_string();

        if width < 100 {
            let mut output = vec![vec![" "; width + 1]; height + 1];
            for point in points {
                let (x, y) = point.borrow().position;
                output[(y - min_y) as usize][(x - min_x) as usize] = "#";
            }

            for line in output {
                this_output += &line.join("");
                this_output += "\n";
            }
        }


        if width < min_width {
            min_width = width;
            last_output = this_output;
        } else {
            return (last_output, time - 1);
        }
    }

    panic!("Did not finish");
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[RefCell<Point>]) -> String {
    let (output, _time) = solve(input);
    output
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[RefCell<Point>]) -> usize {
    let (_output, time) = solve(input);
    time
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, input_generator};

    #[test]
    fn examples() {
        let raw = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

        assert_eq!(solve_part1(&input_generator(raw)),
"\n".to_owned() +
"#   #  ###\n" +
"#   #   # \n" +
"#   #   # \n" +
"#####   # \n" +
"#   #   # \n" +
"#   #   # \n" +
"#   #   # \n" +
"#   #  ###\n");
        assert_eq!(solve_part2(&input_generator(raw)), 3);
    }
}


