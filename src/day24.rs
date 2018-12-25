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

pub fn outcome(input: &Vec<RefCell<Group>>, boost: usize) -> (bool, usize) {
    let mut groups = input.clone();

    let mut friendly_left;
    let mut nonfriendly_left;

    if boost > 0 {
        for i in 0..groups.len() {
            let mut group = groups[i].borrow_mut();
            if group.friendly { group.damage += boost }
        }
    }

    loop {
        let mut targets: Vec<Option<usize>> = vec![None; groups.len()];
        let mut chosen: Vec<bool> = vec![false; groups.len()];
        let indices = (0..groups.len()).collect::<Vec<usize>>();

        let choosing_value = |groups: &Vec<RefCell<Group>>, i: usize| -> (isize, isize) { (-(groups[i].borrow().effective_power() as isize), -(groups[i].borrow().initiative as isize)) };
        let mut in_choosing_order = indices.clone();
        in_choosing_order.sort_by_key(|&i| choosing_value(&groups, i));

        for index in in_choosing_order {
            let mut greatest = (1, 0, 0);
            let mut target: Option<usize> = None;
            let group = groups[index].borrow();

            for other_index in 0..groups.len() {
                if index == other_index { continue }
                if chosen[other_index] { continue }
                let other_group = groups[other_index].borrow();
                if other_group.friendly == group.friendly { continue }

                let scores = (group.damage_to(other_group.clone()), other_group.effective_power(), other_group.initiative);
                if scores > greatest {
                    target = Some(other_index);
                    greatest = scores;
                }
            }

            targets[index] = target;
            if let Some(num) = target { chosen[num] = true; }
        }

        let mut some_attack = false;
        let attacking_value = |groups: &Vec<RefCell<Group>>, i: usize| -> isize { -(groups[i].borrow().initiative as isize) };
        let mut in_attacking_order = indices.clone();
        in_attacking_order.sort_by_key(|&i| attacking_value(&groups, i));

        for index in in_attacking_order {
            let group = groups[index].borrow();
            if group.units == 0 { continue }

            if let Some(target_id) = targets[index] {
                let mut target = groups[target_id].borrow_mut();
                if target.units == 0 { panic!("WTF") }

                let total_damage = group.damage_to(target.clone());
                let remainder = total_damage % target.hit_points;
                let rounded_damage = total_damage - remainder;
                let units_killed = std::cmp::min(rounded_damage / target.hit_points, target.units);

                if units_killed > 0 { some_attack = true; }
                target.units -= units_killed;
            }
        }

        if some_attack == false { return (false, usize::max_value()); }

        groups = groups.into_iter().filter(|ref g| g.borrow().units > 0).collect();
        friendly_left = groups.iter().filter(|ref g| g.borrow().friendly).count();
        nonfriendly_left = groups.iter().filter(|ref g| !g.borrow().friendly).count();

        if friendly_left == 0 || nonfriendly_left == 0 {
            break
        }
    }

    let mut sum = 0;
    for g in groups {
        let group = g.borrow();
        sum += group.units;
    }

    (friendly_left > 0, sum)
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Vec<RefCell<Group>>) -> usize {
    let (_elfs_win, sum) = outcome(input, 0);
    sum
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &Vec<RefCell<Group>>) -> usize {
    let mut hundreds = 0;

    for hundred in 1.. {
        let boost = 1000 * hundred;

        let cloned_input = input.iter().map(|i| RefCell::new(i.borrow().clone())).collect::<Vec<RefCell<Group>>>();
        let (elfs_win, _sum) = outcome(&cloned_input, boost);

        if elfs_win {
            hundreds = hundred - 1;
            break;
        }
    }

    for i in 0..=1000 {
        let boost = (hundreds * 1000) + i;
        let cloned_input = input.iter().map(|i| RefCell::new(i.borrow().clone())).collect::<Vec<RefCell<Group>>>();
        let (elfs_win, sum) = outcome(&cloned_input, boost);

        if elfs_win {
            return sum;
        }
    }

    panic!("Should not get here");
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, input_generator, outcome};

    #[test]
    fn examples() {
        let raw = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

        assert_eq!(solve_part1(&input_generator(raw)), 5216);

        assert_eq!(outcome(&input_generator(raw), 1570), (true, 51));
        assert_eq!(solve_part2(&input_generator(raw)), 51);
    }
}

