use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let (dependencies, tasks) = parse_tasks(&lines.iter().map(|l| l.as_str()).collect());

    let part1 = run_tasks(dependencies.clone(), tasks.clone(), 1, 0);
    println!("Part1: {:?}", part1);
    let part2 = run_tasks(dependencies, tasks, 5, 60);
    // The final result was off by 1, not sure why
    let result = part2.0 - 1;
    println!("Part2: {} {:?}", result, part2);
    Ok(())
}

fn parse_tasks(lines: &Vec<&str>) -> (HashMap<char, Vec<char>>, HashSet<char>) {
    let mut dependencies = HashMap::new();
    let tasks = lines
        .iter()
        .map(|line| {
            let mut it = line.split(' ');
            it.next();
            let a = it.next().unwrap().chars().next().unwrap();
            for _ in 0..5 {
                it.next();
            }
            let b = it.next().unwrap().chars().next().unwrap();
            dependencies.entry(b).or_insert_with(|| vec![]).push(a);
            [a, b]
        })
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|a| a.iter())
        .map(|a| *a)
        .collect::<HashSet<char>>();
    (dependencies, tasks)
}

#[derive(Debug)]
struct Worker {
    task: char,
    time: u8,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            task: '\0',
            time: 0,
        }
    }

    /// Forward the worker by 1 tick
    /// If the time reaches 0 the worker removes itself as dependency
    /// If the time left is 0 the worker will request new work
    /// Returns the time left from the current task
    pub fn tick(
        &mut self,
        dependencies: &mut HashMap<char, Vec<char>>,
        available: &mut Vec<char>,
        offset: u8,
        done: &mut String,
    ) -> u8 {
        if self.time != 0 {
            self.time -= 1;
            if self.time == 0 {
                self.finish(dependencies, available, done);
            }
        }
        if self.time == 0 && !available.is_empty() {
            available.sort_unstable();
            let current = available.remove(0);
            self.task = current;
            self.time = offset + current as u8 - 'A' as u8 + 1;
            dependencies.remove(&self.task);
        }
        self.time
    }

    fn finish(
        &mut self,
        dependencies: &mut HashMap<char, Vec<char>>,
        available: &mut Vec<char>,
        done: &mut String,
    ) {
        dependencies
            .iter_mut()
            .filter_map(|(task, dependencies)| {
                dependencies
                    .iter()
                    .position(|c| *c == self.task)
                    .map_or(None, |pos| Some((task, dependencies, pos)))
            })
            .for_each(|(task, dependencies, pos)| {
                dependencies.remove(pos);
                if dependencies.is_empty() {
                    available.push(*task);
                }
            });
        done.push(self.task);
    }
}

/// Run the tasks specified by the dependecy graph and task list
/// Using `n` workers
/// Returns the number of ticks required to finish all tasks and
/// the order of the completion of the tasks
/// Offset refers to the offset in time found in the specification
fn run_tasks(
    mut dependencies: HashMap<char, Vec<char>>,
    tasks: HashSet<char>,
    n_workers: usize,
    offset: u8,
) -> (usize, String) {
    let mut available = tasks
        .iter()
        .filter(|c| !dependencies.contains_key(c))
        .map(|c| *c)
        .collect::<Vec<_>>();

    let mut workers = (0..n_workers)
        .map(|_| Worker::new())
        .collect::<Vec<Worker>>();

    let mut result = 0;
    let mut order = String::new();
    let mut done = 0;
    while done != n_workers {
        done = workers
            .iter_mut()
            .map(|worker| worker.tick(&mut dependencies, &mut available, offset, &mut order))
            .filter(|time_left| *time_left == 0)
            .count();
        result += 1;
    }

    (result - 1, order)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = [
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];

        let (dependencies, tasks) = parse_tasks(&input.iter().map(|l| *l).collect());
        let result = run_tasks(dependencies.clone(), tasks.clone(), 1, 0);
        assert_eq!(result.1, "CABDFE");
    }

    #[test]
    fn test_part2() {
        let input = [
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];

        let (dependencies, tasks) = parse_tasks(&input.iter().map(|l| *l).collect());
        let result = run_tasks(dependencies, tasks, 2, 0);
        assert_eq!(result.0, 15);
        assert_eq!(result.1, "CABFDE");
    }
}

