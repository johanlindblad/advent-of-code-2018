use std::boxed::Box;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct Input {
    depth: usize,
    target: (usize, usize)
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Box<Input> {
    let mut lines = input.lines();
    let depth: usize = lines.next().unwrap().split(" ").skip(1).next().unwrap().parse().unwrap();

    let mut target_parts = lines.next().unwrap().split(" ").skip(1).next().unwrap().split(",");
    let x: usize = target_parts.next().unwrap().parse().unwrap();
    let y: usize = target_parts.next().unwrap().parse().unwrap();
    let target = (x, y);

    Box::new(Input { depth, target })
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let (target_x, target_y) = input.target;
    let depth: usize = input.depth;

    let mut cave = vec![vec![0; target_x+1]; target_y+1];
    let mut total = 0;

    for y in 0..=target_y {
        for x in 0..=target_x {
            let geologic = match (y, x) {
                (0, 0) => 0,
                coords if coords == input.target => 0,
                (0, _) => x * 16807,
                (_, 0) => y * 48271,
                (_, _) => cave[y][x-1] * cave[y-1][x]
            };

            let erosion = (geologic + depth) % 20183;
            cave[y][x] = erosion;
            total += erosion % 3;
        }
    }

    total
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let (target_x, target_y) = input.target;
    let depth: usize = input.depth;

    let max = std::cmp::max(target_x, target_y);
    let width = max * 4;
    let height = max * 4;

    let mut erosion = vec![vec![0; width]; height];
    let mut cave = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            let geologic = match (y, x) {
                (0, 0) => 0,
                coords if coords == input.target => 0,
                (0, _) => x * 16807,
                (_, 0) => y * 48271,
                (_, _) => erosion[y][x-1] * erosion[y-1][x]
            };

            erosion[y][x] = (geologic + depth) % 20183;
            cave[y][x] = erosion[y][x] % 3;
        }
    }

    let mut shortest = usize::max_value();
    let (_neither, torch, _climbing_gear) = (0, 1, 2);
    let (_rocky, _wet, _narrow) = (0, 1, 2);
    // (time, coordinates, tool)
    let mut frontier: BinaryHeap<(isize, (usize, usize), usize)> = BinaryHeap::new();
    frontier.push((0, (0, 0), torch));

    let mut shortest_to = vec![vec![vec![usize::max_value(); 3]; width]; height];

    while let Some((neg_time, (y, x), tool)) = frontier.pop() {
        let time = (-neg_time) as usize;

        if time >= shortest_to[y][x][tool] { continue }
        shortest_to[y][x][tool] = time;

        if (y, x) == (target_y, target_x) && tool == torch {
            shortest = std::cmp::min(shortest, time);
            continue;
        } else if time > shortest { break; }


        let mut options = vec![(y+1, x), (y, x+1)];
        if y > 0 { options.push((y-1, x)) }
        if x > 0 { options.push((y, x-1)) }

        for &(ny, nx) in &options {
            let new_time = time + 1;
            if shortest_to[ny][nx][tool] <= new_time { continue }
            if cave[ny][nx] == tool { continue }

            frontier.push((neg_time - 1, (ny, nx), tool));
        }

        let tile = cave[y][x];
        let mut next_tool = (tool + 1) % 3;
        if next_tool == tile { next_tool = (next_tool + 1) % 3 }

        frontier.push((neg_time - 7, (y, x), next_tool));
    }

    shortest
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, Input};

    #[test]
    fn examples() {
        let depth = 510;
        let target = (10, 10);
        assert_eq!(solve_part1(&Box::new(Input { depth, target })), 114);
        assert_eq!(solve_part2(&Box::new(Input { depth, target })), 45);
    }
}

