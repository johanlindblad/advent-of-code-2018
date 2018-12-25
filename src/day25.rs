use disjoint_sets::UnionFind;
pub type Point = (isize, isize, isize, isize);

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
        let parts = line.trim().split(",").map(|part| part.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        (parts[0], parts[1], parts[2], parts[3])
    }).collect()
}

fn reaches(a: &Point, b: &Point) -> bool {
    let distance = (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs();
    distance <= 3
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &[Point]) -> usize {
    let mut sets: UnionFind<usize> = UnionFind::new(input.len());
    let mut constellations = input.len();

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == 0 { continue }

            if !sets.equiv(i, j) && reaches(&input[i], &input[j]) {
                sets.union(i, j);
                constellations -= 1;
            }
        }
    }

    constellations
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, input_generator};

    #[test]
    fn examples() {
         let raw = "0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";
         assert_eq!(solve_part1(&input_generator(raw)), 2);
    }
}

