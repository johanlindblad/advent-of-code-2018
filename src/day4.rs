use std::str::FromStr;
use std::cmp::Ordering;
use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
enum Observation {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp
}

#[derive(Eq, Debug)]
pub struct LogEntry {
    date: (u32, u8, u8),
    time: (u8, u8),
    observation: Observation
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        match self.date.cmp(&other.date) {
            Ordering::Equal => self.time.cmp(&other.time),
            ord => ord
        }
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.date == other.date &&
            self.time == other.time &&
            self.observation == other.observation
    }
}

impl FromStr for Observation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        match parts.next().unwrap() {
            "Guard" => Ok(Observation::BeginsShift(parts.next().unwrap().replace("#", "").parse::<u32>().unwrap())),
            "falls" => Ok(Observation::FallsAsleep),
            "wakes" => Ok(Observation::WakesUp),
            &_ => Err(())
        }
    }
}

impl FromStr for LogEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let re = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)").unwrap();
        let capture = re.captures(s).unwrap();
        let mut caps = capture.iter().skip(1);

        let date_parts: Vec<u32> = caps.by_ref().take(3).map(|c| c.unwrap().as_str().parse::<u32>().unwrap()).collect();
        let time_parts: Vec<u8> = caps.by_ref().take(2).map(|c| c.unwrap().as_str().parse::<u8>().unwrap()).collect();

        let date = (date_parts[0], date_parts[1] as u8, date_parts[2] as u8);
        let time = (time_parts[0], time_parts[1]);
        let observation: Observation = caps.next().unwrap().unwrap().as_str().parse::<Observation>().unwrap();

        Ok(LogEntry { date: date, time: time, observation: observation })
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<LogEntry> {
    let mut parsed =
        input
            .lines()
            .map(|l| l.parse::<LogEntry>().unwrap())
            .collect::<Vec<LogEntry>>();

    parsed.sort();
    parsed
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[LogEntry]) -> u32 {
    let mut guard: u32 = 0;
    let mut minutes = vec![vec![0 as u32; 60]; 4096];
    let mut iter = input.iter();

    while let Some(entry) = iter.next() {
        match entry.observation {
            Observation::BeginsShift(n) => guard = n,
            Observation::FallsAsleep => {
                let next_entry = iter.next().unwrap();
                let (_, minute_end) = next_entry.time;
                let (_, minute_start) = entry.time;

                for i in minute_start..minute_end {
                    minutes[guard as usize][i as usize] += 1;
                }
            },
            _ => panic!("")
        }
    }

    let num_minutes = |minutes: &Vec<u32>| -> u32 {
        minutes.iter().sum()
    };

    let (guard, minutes) =
        minutes
            .iter()
            .enumerate()
            .max_by(|(_, minutes1), (_, minutes2)| num_minutes(minutes1).cmp(&num_minutes(minutes2)))
            .unwrap();

    let (minute, _count) = minutes
        .iter()
        .enumerate()
        .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
        .unwrap();

    guard as u32 * minute as u32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[LogEntry]) -> u32 {
    let mut guard: u32 = 0;
    let mut minutes = vec![vec![0 as u32; 60]; 4096];
    let mut iter = input.iter();

    while let Some(entry) = iter.next() {
        match entry.observation {
            Observation::BeginsShift(n) => guard = n,
            Observation::FallsAsleep => {
                let next_entry = iter.next().unwrap();
                let (_, minute_end) = next_entry.time;
                let (_, minute_start) = entry.time;

                for i in minute_start..minute_end {
                    minutes[guard as usize][i as usize] += 1;
                }
            },
            _ => panic!("")
        }
    }

    let max_minute = |minutes: &Vec<u32>| {
        let (minute, count) =
            minutes
                .iter()
                .enumerate()
                .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
                .unwrap();

        (minute, count.clone())
    };

    let (guard, (minute, _count)) =
        minutes
            .iter()
            .enumerate()
            .map(|(guard, minutes)| (guard, max_minute(minutes)))
            .max_by(|(_, (_, c1)), (_, (_, c2))| c1.cmp(c2))
            .unwrap();

    guard as u32 * minute as u32
}


#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, input_generator};

    #[test]
    fn examples() {
        let raw = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        assert_eq!(solve_part1(&input_generator(raw)), 240);
        assert_eq!(solve_part2(&input_generator(raw)), 4455);
    }
}

