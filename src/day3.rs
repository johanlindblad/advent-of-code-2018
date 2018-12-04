type Specification = (u32, (u32, u32), (u32, u32));
const WIDTH: usize = 1000;

fn parse(line: &str) -> Specification {
    let split_fn = |c| c == '#' || c == '@' || c == ',' || c == ':' || c == 'x';

    let parts: Vec<u32> =
        line
            .split(split_fn)
            .map(|part| part.trim())
            .skip(1)
            .map(|part| part.parse::<u32>().unwrap())
            .collect();

    (parts[0], (parts[1], parts[2]), (parts[3], parts[4]))
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Specification> {
    input
        .lines()
        .map(|l| parse(l))
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Specification]) -> usize {
    let mut fabric = vec![vec![0 as u32; WIDTH]; WIDTH];

    for (_id, (xstart, ystart), (w, h)) in input {
        for x in *xstart..(*xstart + *w) {
            for y in *ystart..(*ystart + *h) {
                fabric[x as usize][y as usize] += 1;
            }
        }
    }

    let overlaps_in_column = |column: &Vec<u32>| (
        column
            .iter()
            .filter(|&n| *n > 1)
            .count()
    );

    fabric.iter().map(overlaps_in_column).sum()
}

fn overlap(a: &Specification, b: &Specification) -> bool {
    let (_, (x1, y1), (w1, h1)) = *a;
    let (_, (x2, y2), (_w2, h2)) = *b;

    if x1 > x2 { return overlap(b, a) }
    if x1 + w1 <= x2 { return false }
    if y1 + h1 <= y2 { return false }
    if y2 + h2 <= y1 { return false }

    true
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Specification]) -> u32 {
    for &piece in input {
        let mut free = true;

        for &other_piece in input {
            if piece == other_piece { continue }

            if overlap(&piece, &other_piece) {
                free = false;
                break;
            }
        }

        if free {
            let (id, _, _) = piece;
            return id;
        }
    }

    panic!("Found no non-overlapping piece");
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2, overlap};

    #[test]
    fn test_overlap() {
        let a = (1, (1, 1), (5, 5));
        let b = (2, (3, 3), (2, 2));
        let c = (3, (6, 6), (1, 1));
        let d = (4, (1, 3), (5, 5));
        assert!(overlap(&a, &b));
        assert!(overlap(&b, &a));
        assert!(!overlap(&a, &c));
        assert!(!overlap(&c, &a));
        assert!(overlap(&a, &d));
        assert!(overlap(&d, &a));
    }
    #[test]
    fn examples() {
        let raw = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        let input = input_generator(raw);
        assert_eq!(solve_part1(&input), 4);
        assert_eq!(solve_part2(&input), 3);
    }
}

