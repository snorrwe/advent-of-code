extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let mut events: Vec<Event> = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| Event::parse(&line))
        .collect();

    events.sort_by_key(|event| event.time);
    let result = part1(events.iter());
    println!("Day04 Part1: {}", result);
    Ok(())
}

fn part1<'a, T>(events: T) -> u32
where
    T: Iterator<Item = &'a Event>,
{
    let schedule = create_schedule(events);
    let most_asleep_guard = most_asleep_guard(&schedule).expect("Oops 1");
    let most_slept = schedule
        .get(&most_asleep_guard.0)
        .expect("Oops 2")
        .iter()
        .max_by_key(|(_, &value)| value)
        .expect("Oops 3");

    most_asleep_guard.0 * most_slept.0.minute()
}

fn most_asleep_guard(schedule: &Schedule) -> Option<(u32, usize)> {
    schedule
        .iter()
        .map(|(id, sleep)| (*id, sleep.len()))
        .max_by_key(|(_, value)| *value)
}

fn create_schedule<'a, T>(events: T) -> Schedule
where
    T: Iterator<Item = &'a Event>,
{
    let mut current: u32 = 0;
    let mut asleep: Option<DateTime<Utc>> = None;
    let mut schedule = Schedule::new();
    'b: for event in events {
        match event.event {
            EventType::Id(id) => {
                current = id;
                asleep = None;
            }
            EventType::Awake => {
                let asleep = asleep.expect("Oops 4");
                let delta: Duration = event.time - asleep;
                let asleep = asleep.time();
                let schedule = schedule.entry(current).or_insert_with(|| BTreeMap::new());
                for i in 0..delta.num_minutes() {
                    let i = asleep + Duration::minutes(i);
                    *schedule.entry(i).or_insert_with(|| 0) += 1;
                }
            }
            EventType::Asleep => {
                asleep = Some(event.time);
            }
        }
    }
    schedule
}

type Schedule = BTreeMap<u32, BTreeMap<chrono::NaiveTime, usize>>;

#[derive(Debug, Eq, PartialEq)]
struct Event {
    time: DateTime<Utc>,
    event: EventType,
}

#[derive(Debug, Eq, PartialEq)]
enum EventType {
    Asleep,
    Awake,
    Id(u32),
}

impl Event {
    pub fn parse(string: &str) -> Option<Event> {
        let dt = &string[1..17];
        let dt = Utc.datetime_from_str(dt, "%Y-%m-%d %H:%M").expect("Oops 5");
        let event = string[19..].split(' ').collect::<Vec<&str>>();

        let event = match event[0] {
            "Guard" => EventType::Id(event[1][1..].parse().expect("Oops 6")),
            "wakes" => EventType::Awake,
            "falls" => EventType::Asleep,
            _ => return None,
        };
        Some(Event {
            time: dt,
            event: event,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let events = [
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
        .iter()
        .filter_map(|line| Event::parse(line))
        .collect::<Vec<Event>>();

        let result = part1(events.iter());

        assert_eq!(result, 240);
    }

    #[test]
    fn test_part1_overflowing_time() {
        let mut events = [
            "[1518-11-01 23:55] falls asleep",
            "[1518-11-02 23:55] falls asleep",
            "[1518-11-02 23:00] Guard #99 begins shift",
            "[1518-11-02 23:57] wakes up",
            "[1518-11-03 23:56] wakes up",
            "[1518-11-03 23:00] Guard #10 begins shift",
            "[1518-11-02 00:05] wakes up",
            "[1518-11-03 23:45] falls asleep",
            "[1518-11-01 00:00] Guard #10 begins shift",
        ]
        .iter()
        .filter_map(|line| Event::parse(line))
        .collect::<Vec<Event>>();
        events.sort_by_key(|event| event.time);

        let result = part1(events.iter());

        assert_eq!(result, 10 * 55);
    }
}
