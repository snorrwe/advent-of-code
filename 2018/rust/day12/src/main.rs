use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

mod parse;

pub type World = BTreeSet<i32>;
pub type Rules = HashMap<[bool; 5], bool>;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().filter_map(|l| l.ok());

    let (state, rules) = parse::load_initial_state(lines);

    let part1 = part1(state.clone(), &rules);

    println!("Part1: {}", part1);

    let part2 = part2(state, &rules);

    println!("Part2: {}", part2);

    Ok(())
}

fn part1(mut world: World, rules: &Rules) -> i32 {
    world = tick(world, rules, 20);
    world.iter().sum()
}

fn part2(mut world: World, rules: &Rules) -> u64 {
    world = tick(world, rules, 500);
    world.iter().map(|i| *i as u64 - 500 + 50_000_000_000).sum()
}

fn tick(mut world: World, rules: &Rules, iterations: u64) -> World {
    for _tick in 0..iterations {
        world = world
            .iter()
            .map(|index| index - 2..=index + 2)
            .flatten()
            .filter_map(|index| {
                if on(index, &world, &rules) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<World>();
    }
    world
}

fn on(index: i32, world: &World, rules: &Rules) -> bool {
    let environment = [
        world.get(&(index - 2)).map_or(false, |_| true),
        world.get(&(index - 1)).map_or(false, |_| true),
        world.get(&(index)).map_or(false, |_| true),
        world.get(&(index + 1)).map_or(false, |_| true),
        world.get(&(index + 2)).map_or(false, |_| true),
    ];
    *rules.get(&environment).unwrap_or(&false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tick() {
        let input = [
            "initial state: #..#.#..##......###...###",
            "",
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]
        .iter()
        .map(|s| s.to_string());

        let (state, rules) = parse::load_initial_state(input);

        let actual = tick(state, &rules, 1);

        let expected = "...#...#....#.....#..#..#..#..........."
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '#' { Some(i as i32 - 3) } else { None })
            .collect::<World>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let input = [
            "initial state: #..#.#..##......###...###",
            "",
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]
        .iter()
        .map(|s| s.to_string());

        let (state, rules) = parse::load_initial_state(input);

        let actual = part1(state, &rules);

        assert_eq!(actual, 325);
    }
}

