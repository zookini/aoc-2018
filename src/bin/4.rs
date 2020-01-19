use itertools::enumerate;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let mut events: Vec<_> = include_str!("../../input/4.txt").lines().collect();
    let mut guard = 0;
    let mut timer = 0;
    let mut roster = HashMap::new();

    let re = regex::Regex::new(concat!(
        r"\[\d{4}-(?P<month>\d{2})-(?P<day>\d{2}) \d{2}:(?P<minute>\d{2})] ",
        r"(Guard #(?P<guard>\d+)|(?P<sleep>falls asleep)|(?P<wakes>wakes up))",
    ))?;

    events.sort();

    for event in events {
        let cap = re.captures(event).unwrap();
        let now = cap["minute"].parse()?;

        if let Some(id) = cap.name("guard") {
            guard = id.as_str().parse()?;
        } else if cap.name("wakes").is_some() {
            for time in &mut roster.entry(guard).or_insert_with(|| vec![0; 60])[timer..now] {
                *time += 1;
            }
        }

        timer = now;
    }

    println!("{}", p1(&roster));
    println!("{}", p2(&roster));
    Ok(())
}

fn p1(roster: &HashMap<usize, Vec<usize>>) -> usize {
    checksum(roster, |v| v.iter().sum())
}

fn p2(roster: &HashMap<usize, Vec<usize>>) -> usize {
    checksum(roster, |v| *v.iter().max().unwrap())
}

fn checksum(roster: &HashMap<usize, Vec<usize>>, f: impl Fn(&Vec<usize>) -> usize) -> usize {
    let (&id, timetable) = roster.iter().max_by_key(|(_, v)| f(v)).unwrap();
    id * enumerate(timetable).max_by_key(|&(_, c)| c).unwrap().0
}
