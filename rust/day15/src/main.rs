use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

mod creature;
mod pathfinder;
mod point;
use self::creature::*;
use self::point::*;

pub type OccupiedPoints = BTreeMap<Point, Option<Race>>;
pub type Creatures = Vec<Creature>;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().filter_map(|l| l.ok());

    let (map, creatures) = build_map(lines);

    let part1 = battle(map, creatures);

    println!("Part1: {:?}", part1);

    Ok(())
}

fn battle(mut map: OccupiedPoints, mut creatures: Creatures) -> u32 {
    let n_creatures = |creatures: &Creatures| {
        creatures.iter().fold([0, 0], |mut count, c| {
            if c.hp <= 0 {
                return count;
            }
            if c.race == Race::Elf {
                count[0] += 1;
            } else {
                count[1] += 1
            }
            count
        })
    };
    let mut ticks = 0;
    let mut count = n_creatures(&creatures);
    while count[0] > 0 && count[1] > 0 {
        if !tick(&mut map, &mut creatures) {
            ticks += 1;
        }
        count = n_creatures(&creatures);
    }
    let total_hp: u32 = creatures
        .iter()
        .filter_map(|c| if c.hp > 0 { Some(c.hp as u32) } else { None })
        .sum();
    println!("{:#?}", creatures);
    println!("{} {}", ticks, total_hp);
    ticks * total_hp
}

/// Returns wether the battle finished (any creature found no target)
fn tick(map: &mut OccupiedPoints, creatures: &mut Creatures) -> bool {
    // Reading order
    creatures.sort_unstable_by_key(|creature| creature.position);
    debug_assert!(!creatures
        .iter()
        .zip(creatures[1..].iter())
        .any(|(c1, c2)| c1.position.y > c2.position.y
            || (c1.position.y == c2.position.y && c1.position.x > c2.position.x)));
    // Using a c-style `for` to be able to modify values in the collection
    let mut finished = false;
    for i in 0..creatures.len() {
        let mut creature = creatures.get_mut(i).unwrap().clone();
        if creature.hp <= 0 {
            continue;
        }
        let found_target = creature.tick(creatures, map);
        creatures[i] = creature;
        if !found_target {
            finished = true;
        }
    }
    creatures.retain(|c| c.hp > 0);
    finished
}

fn build_map<'a, I>(lines: I) -> (OccupiedPoints, Creatures)
where
    I: Iterator<Item = String>,
{
    let mut creatures = Vec::with_capacity(100);
    let mut add_creature = |x: usize, y: usize, race: Race| {
        let x = x as i32;
        let y = y as i32;
        let creature = Creature::new(x, y, race);
        creatures.push(creature);
    };
    let point = |x: usize, y: usize, value| Some((Point::new(x as i32, y as i32), value));
    let map = lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => point(x, y, None),
                    'E' => {
                        add_creature(x, y, Race::Elf);
                        point(x, y, Some(Race::Elf))
                    }
                    'G' => {
                        add_creature(x, y, Race::Goblin);
                        point(x, y, Some(Race::Goblin))
                    }

                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    creatures.shrink_to_fit();
    (map, creatures)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_building() {
        let input = ["##", "#.G", "E.G"];

        let exp_map = [
            (Point::new(0, 0), None),
            (Point::new(1, 0), None),
            (Point::new(0, 1), None),
            (Point::new(2, 1), Some(Race::Goblin)),
            (Point::new(0, 2), Some(Race::Elf)),
            (Point::new(2, 2), Some(Race::Goblin)),
        ]
        .iter()
        .cloned()
        .collect::<OccupiedPoints>();

        let exp_creatures = [
            Creature::new(2, 1, Race::Goblin),
            Creature::new(0, 2, Race::Elf),
            Creature::new(2, 2, Race::Goblin),
        ]
        .iter()
        .cloned()
        .collect::<Creatures>();

        let (map, creatures) = build_map(input.iter().map(|s| s.to_string()));

        assert_eq!(map, exp_map);
        assert_eq!(creatures, exp_creatures);
    }

    #[test]
    fn test_ticks() {
        let input = [
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
            "                     ",
        ];

        let (mut map, mut creatures) = build_map(input.iter().map(|s| s.to_string()));

        let exp_input = [
            "#######",
            "#...G.#",
            "#..GEG#",
            "#.#.#G#",
            "#...#E#",
            "#.....#",
            "#######",
            "                    ",
        ];
        let (exp_map, mut exp_creatures) = build_map(exp_input.iter().map(|s| s.to_string()));

        // Set healths'
        exp_creatures[0].hp = 200;
        exp_creatures[1].hp = 200;
        exp_creatures[2].hp = 188;
        exp_creatures[3].hp = 194;
        exp_creatures[4].hp = 194;
        exp_creatures[5].hp = 194;

        tick(&mut map, &mut creatures);
        tick(&mut map, &mut creatures);
        creatures.sort_unstable_by_key(|creature| creature.position);

        assert_eq!(map, exp_map);
        assert_eq!(creatures, exp_creatures);
    }

    #[test]
    fn test_battle() {
        let input = [
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
            "                  ",
        ];

        let (map, creatures) = build_map(input.iter().map(|s| s.to_string()));

        let result = battle(map, creatures);

        assert_eq!(result, 27730);
    }

    #[test]
    fn test_battle_2() {
        let input = [
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
            "                  ",
        ];

        let (map, creatures) = build_map(input.iter().map(|s| s.to_string()));

        let result = battle(map, creatures);

        assert_eq!(result, 18740);
    }

    #[test]
    fn test_battle_3() {
        let input = [
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
            "                  ",
        ];

        let (map, creatures) = build_map(input.iter().map(|s| s.to_string()));

        let result = battle(map, creatures);

        assert_eq!(result, 28944);
    }

    #[test]
    fn test_battle_4() {
        let input = [
            "#######",
            "#G..#E#",
            "#E#E.E#",
            "#G.##.#",
            "#...#E#",
            "#...E.#",
            "#######",
            "                  ",
        ];

        let (map, creatures) = build_map(input.iter().map(|s| s.to_string()));

        let result = battle(map, creatures);

        assert_eq!(result, 36334);
    }

    #[test]
    fn test_battle_5() {
        let input = [
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
            "                  ",
        ];

        let (map, creatures) = build_map(input.iter().map(|s| s.to_string()));

        let result = battle(map, creatures);

        assert_eq!(result, 39514);
    }

    #[test]
    fn test_battle_6() {
        let input = [
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
            "                  ",
        ];

        let (map, creatures) = build_map(input.iter().map(|s| s.to_string()));

        let result = battle(map, creatures);

        assert_eq!(result, 27755);
    }
}

