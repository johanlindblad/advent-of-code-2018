use std::boxed::Box;
use regex::Regex;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct Group {
    units: usize,
    hit_points: usize,
    damage: usize,
    damage_type: String,
    initiative: usize,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
    friendly: bool,
    number: usize
}

impl Group {
    fn damage_to(&self, ref other: Group) -> usize {
        let mut unaccounted = self.effective_power();

        for i in other.immunities.iter() {
            if *i == self.damage_type {
                unaccounted = 0;
                break;
            }
        }

        for w in other.weaknesses.iter() {
            if *w == self.damage_type {
                unaccounted *= 2;
                break;
            }
        }

        unaccounted
    }

    fn effective_power(&self) -> usize {
        self.units * self.damage
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<RefCell<Group>> {
    let sections = input.split("\n\n");

    let line_matcher = Regex::new(r"([0-9]+) units each with ([0-9]+) hit points (\((.*?)\) )?with an attack that does ([0-9]+) ([a-z]+) damage at initiative ([0-9]+)").unwrap();
    let weak_immune_matcher = Regex::new(r"(weak|immune) to ([a-z, ]+)$").unwrap();
    let mut friendly = true;

    let mut groups: Vec<RefCell<Group>> = Vec::new();

    sections.for_each(|section| {
        let mut lines = section.lines();
        let _title = lines.next();
        let mut number = 1;

        for line in lines {
            let capture = line_matcher.captures(line).unwrap();
            let units = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let hit_points = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let weak_and_immune = capture.get(4);
            let damage = capture.get(5).unwrap().as_str().parse::<usize>().unwrap();
            let damage_type = capture.get(6).unwrap().as_str().to_string();
            let initiative = capture.get(7).unwrap().as_str().parse::<usize>().unwrap();

            let mut weaknesses: Vec<String> = Vec::new();
            let mut immunities: Vec<String> = Vec::new();

            if let Some(desc) = weak_and_immune {
                let parts = desc.as_str().split("; ");

                for part in parts {
                    let captures = weak_immune_matcher.captures(part).unwrap();

                    if captures.get(1).unwrap().as_str() == "weak" {
                        weaknesses = captures.get(2).unwrap().as_str().split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
                    } else {
                        immunities = captures.get(2).unwrap().as_str().split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
                    }
                }
            }

            groups.push(
                RefCell::new(
                    Group { units, hit_points, damage, damage_type, initiative, immunities, weaknesses, friendly, number }
                )
            );

            number += 1;
        }

        friendly = false
    });

    groups
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Vec<RefCell<Group>>) -> usize {
    let mut groups = input.clone();

    let mut friendly_left = groups.iter().filter(|ref g| g.borrow().friendly).count();
    let mut nonfriendly_left = groups.iter().filter(|ref g| !g.borrow().friendly).count();

    loop {
        let mut targets: Vec<Option<usize>> = Vec::new();
        let mut chosen: Vec<bool> = vec![false; groups.len()];
        groups.sort_by_key(|ref g| (-(g.borrow().effective_power() as isize), -(g.borrow().initiative as isize)));

        for group in groups.iter() {
            let mut greatest = (0, 0, 0);
            let mut target: Option<usize> = None;
            let instance = group.borrow().clone();
            println!("Selecting for {:?}", instance);

            for index in 0..groups.len() {
                if chosen[index] { continue }
                let i = groups[index].borrow().clone();

                if i.units == 0 { continue }

                if i.friendly != instance.friendly {
                    let this_score = (instance.damage_to(i.clone()), i.effective_power(), i.initiative);
                    println!("on {}: {:?}", i.number, this_score);

                    if this_score > greatest {
                        target = Some(index);
                        greatest = this_score;
                    }
                }
            }

            targets.push(target);
            if let Some(num) = target {
                println!("{:?} chooses {:?}, would deal {}", instance, groups[num].borrow(), instance.damage_to(groups[num].borrow().clone()));

                println!("");
                chosen[num] = true;
            }
        }

        groups.sort_by_key(|ref g| -(g.borrow().initiative as isize));

        for group in groups.iter() {
            let mut greatest = (0, 0, 0);
            let mut target: Option<usize> = None;
            let instance = group.borrow().clone();

            for index in 0..groups.len() {
                if chosen[index] { continue }
                let i = groups[index].borrow().clone();

                if i.units == 0 { continue }
                if targets.contains(&Some(index)) { continue }

                if i.friendly != instance.friendly && instance.damage_to(i.clone()) > 0 {
                    let this_score = (instance.damage_to(i.clone()), i.effective_power(), i.initiative);

                    if this_score > greatest {
                        target = Some(index);
                        greatest = this_score;
                    }
                }
            }

            targets.push(target);
            if let Some(num) = target {
                chosen[num] = true;
            }
        }

        for i in 0..groups.len() {
            let group = &groups[i];

            if group.borrow().units == 0 {
                continue;
            }

            if let Some(group_id) = targets[i] {
                let target = &groups[group_id];

                if target.borrow().units == 0 { continue }

                let mut damage = group.borrow().damage_to(target.borrow().clone());
                let remains = damage % target.borrow().hit_points;
                damage -= remains;

                let existing_units = target.borrow().units;

                if damage >= existing_units {
                    target.borrow_mut().units = 0;

                    if target.borrow().friendly {
                        friendly_left -= 1
                    } else {
                        nonfriendly_left -= 1;
                    }
                } else {
                    target.borrow_mut().units -= damage;
                }

                println!("{:?} DOES {} TO {:?}", group.borrow(), damage, target.borrow());
            }
        }
        println!("{}, {}", friendly_left, nonfriendly_left);

        if friendly_left == 0 || nonfriendly_left == 0 {
            break
        }
    }

    let mut sum = 0;
    for g in groups {
        let group = g.borrow();
        sum += group.units;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, input_generator};

    #[test]
    fn examples() {
        let raw = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

        assert_eq!(solve_part1(&input_generator(raw)), 5216);
    }
}

