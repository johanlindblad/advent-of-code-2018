use std::process::{Command, Stdio};
use std::io::Write;

pub type Nanobot = ((isize, isize, isize), usize);

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Nanobot> {
    input.lines().map(|line| {
        let mut parts = line.split(|c| c == '<' || c == '>' || c == '=' || c == ',');

        let x: isize = parts.nth(2).unwrap().parse().unwrap();
        let y: isize = parts.next().unwrap().parse().unwrap();
        let z: isize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.nth(2).unwrap().parse().unwrap();

        ((x, y, z), r)
    }).collect()
}

fn manhattan((pos, _): &Nanobot, (pos2, _): &Nanobot) -> usize {
    (pos.0 - pos2.0).abs() as usize + (pos.1 - pos2.1).abs() as usize + (pos.2 - pos2.2).abs() as usize
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[Nanobot]) -> usize {
    let greatest = input.iter().max_by_key(|(_, range)| range).unwrap();
    let (_, greatest_range) = greatest;

    input.iter().filter(|&&bot| manhattan(&greatest, &bot) <= *greatest_range).count()
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[Nanobot]) -> usize {

    let intro = "(declare-fun z () Int)
(declare-fun y () Int)
(declare-fun x () Int)
(declare-fun sum () Int)
(declare-fun dist () Int)".to_string();

    let def_in_range = (0..input.len()).map(|i| format!("(declare-fun in_range_{} () Int)\n", i)).collect::<String>();

    let assert_in_range = input.iter().enumerate().map(|(i, ((x, y, z), r))| {
        format!("
(assert (let ((a!1 (+ (ite (>= (- x {0}) 0) (- x {0}) (- (- x {0})))
              (ite (>= (- y {1}) 0) (- y {1}) (- (- y {1})))
              (ite (>= (- z {2}) 0) (- z {2}) (- (- z {2}))))))
  (= in_range_{4} (ite (<= a!1 {3}) 1 0))))\n", x, y, z, r, i)
    }).collect::<String>();

    let sum_in_range = (0..input.len()).map(|i| format!(" in_range_{}", i)).collect::<String>();
    let assert_sum = format!("(assert (= sum (+ 0{})))", sum_in_range);
    let assert_dist = "(assert (= dist
   (+ (ite (>= x 0) x (- x)) (ite (>= y 0) y (- y)) (ite (>= z 0) z (- z)))))".to_string();

    let outro = "(maximize sum)
(minimize dist)
(check-sat)
(get-value (dist))".to_string();

    let z3_input = [intro, def_in_range, assert_in_range, assert_sum, assert_dist, outro].join("\n");

    let mut child = Command::new("z3")
        .arg("-in")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(z3_input.as_bytes()).unwrap();
    }

    let output = child.wait_with_output().unwrap();
    let output_string = String::from_utf8_lossy(&output.stdout);
    let dist_line = output_string.lines().skip(1).next().unwrap();
    let dist = &dist_line[7..dist_line.len() - 2].parse::<usize>().unwrap();
    *dist
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, input_generator};

    #[test]
    fn examples() {
        let raw = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

        assert_eq!(solve_part1(&input_generator(raw)), 7);

        let raw2 = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        assert_eq!(solve_part2(&input_generator(raw2)), 36);
    }
}

