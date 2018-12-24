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
    println!("[");
    for ((x, y, z), r) in input {
        println!(" [{},{},{},{}],", x, y, z, r);
    }
    println!("]");
    /*let mut clusters = input.clone();

    for &bot1 in input.iter() {
        let ((x1, y1, z1), range1) = bot1;
        println!("{:?}", bot1);

        for &bot2 in input.iter() {
            if bot1 == bot2 { continue }
            let ((_x2, _y2, _z2), range2) = bot2;
            let distance = manhattan(&bot1, &bot2);

            //let dist = ((x1-x2).abs(), (y1-y2).abs(), (z1-z2).abs());

            if distance <= range2 {
                println!("Has {:?} in range", bot2);
            }

        }
    }*/


    37
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

