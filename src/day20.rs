use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;

fn calculate_distances(input: &str) -> BTreeMap<(isize, isize), usize> {
    let trimmed = &input[1..input.len()];

    let mut neighbours: BTreeMap<(isize, isize), Vec<(isize, isize)>> = BTreeMap::new();
    let mut chars = trimmed.chars().peekable();

    let mut positions_stack: Vec<(BTreeSet<(isize, isize)>, BTreeSet<(isize, isize)>)> = Vec::new();
    let mut positions: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut starting_points: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut ending_points: BTreeSet<(isize, isize)> = BTreeSet::new();
    positions.insert((0, 0));
    starting_points.insert((0, 0));

    while let Some(c) = chars.next() {
        match c {
            'N' | 'W' | 'E' | 'S' => {
                let mut new_positions: BTreeSet<(isize, isize)> = BTreeSet::new();

                for &(y, x) in positions.iter() {
                    let new_position = match c {
                        'N' => (y-1, x),
                        'W' => (y, x-1),
                        'E' => (y, x+1),
                        'S' => (y+1, x),
                        _ => panic!("")
                    };

                    neighbours.entry((y, x)).or_insert(Vec::new());
                    neighbours.get_mut(&(y, x)).unwrap().push(new_position);
                    new_positions.insert(new_position);
                }

                positions.clear();
                for point in new_positions {
                    positions.insert(point);
                }
            }
            '$' => break,
            '(' => {
                positions_stack.push((starting_points, ending_points));
                starting_points = positions.clone();
                ending_points = BTreeSet::new();
            },
            ')' => {
                positions.append(&mut ending_points);
                let (s, e) = positions_stack.pop().unwrap();
                starting_points = s.clone();
                ending_points = e.clone();
            },
            '|' => {
                ending_points.append(&mut positions);
                positions = starting_points.clone();
            }
            other => panic!("Unexpected {}", other)
        }

    }

    let mut frontier: VecDeque<(usize, isize, isize)> = VecDeque::new();
    frontier.push_back((0, 0, 0));
    let mut distance_to: BTreeMap<(isize, isize), usize> = BTreeMap::new();

    while let Some((distance, y, x)) = frontier.pop_back() {

        if let None = distance_to.get(&(y, x)) {
            distance_to.insert((y, x), distance);
        } else {
            let other_distance: usize = distance_to.get(&(y, x)).unwrap().clone();
            if other_distance > distance {
                distance_to.insert((y, x), distance);
            } else {
                continue;
            }
        }

        if let Some(adjacent) = neighbours.get(&(y, x)) {
            for (y, x) in adjacent {
                frontier.push_back((distance + 1, *y, *x));
            }
        }
    }

    distance_to
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &str) -> usize {
    let distance_to = calculate_distances(input);

    let mut max = 0;
    for value in distance_to.values() {
        if *value > max { max = *value }
    }

    max
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &str) -> usize {
    let distance_to = calculate_distances(input);

    let mut total = 0;
    for value in distance_to.values() {
        if *value >= 1000 { total += 1 }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::{solve_part1};

    #[test]
    fn examples() {
        let reg = "^WNE$";
        assert_eq!(solve_part1(reg), 3);

        let reg = "^WNE(E|N)$";
        assert_eq!(solve_part1(reg), 4);

        let reg0 = "^ENWWW(NEEE|SSE(EE|N))$";
        assert_eq!(solve_part1(reg0), 10);

        let reg1 = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        assert_eq!(solve_part1(reg1), 23);

        let reg2 = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        assert_eq!(solve_part1(reg2), 31);

        let reg3 = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        assert_eq!(solve_part1(reg3), 18);
    }
}

