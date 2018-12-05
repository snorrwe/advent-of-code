use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut polymer = String::new();
    File::open("input.txt")?.read_to_string(&mut polymer)?;
    polymer.retain(|c| c.is_alphanumeric());

    let result = react(polymer.clone());
    println!("Day 05 part1: {}", result.len());
    let result = part2(polymer.clone());
    println!("Day 05 part2: {}", result.len());
    Ok(())
}

fn react(mut polymer: String) -> String {
    loop {
        let len = polymer.len();
        tick(&mut polymer);
        if len == polymer.len() {
            break;
        }
    }
    polymer
}

fn tick(polymer: &mut String) {
    let to_remove = polymer
        .chars()
        .enumerate()
        .zip(polymer[1..].chars())
        .find_map(|((i, a), b)| {
            if a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase() {
                Some(i)
            } else {
                None
            }
        });

    if let Some(i) = to_remove {
        polymer.remove(i);
        polymer.remove(i);
    }
}

fn part2(polymer: String) -> String {
    let set = polymer
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    let mut shortest: String = polymer.clone();
    for chr in set {
        let mut polymer = polymer.clone();
        polymer.retain(|c| c.to_ascii_lowercase() != chr);
        let res = react(polymer);
        if res.len() < shortest.len() {
            shortest = res;
        }
    }
    shortest
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_reaction1() {
        let mut polymer = "aA".to_string();

        tick(&mut polymer);

        assert_eq!(polymer, "");
    }

    #[test]
    fn test_reaction2() {
        let mut polymer = "abBA".to_string();

        tick(&mut polymer);

        assert_eq!(polymer, "aA");

        tick(&mut polymer);

        assert_eq!(polymer, "");
    }

    #[test]
    fn test_reaction3() {
        let mut polymer = "aabAAB".to_string();

        tick(&mut polymer);

        assert_eq!(polymer, "aabAAB");
    }

    #[test]
    fn test_part1() {
        let polymer = "dabAcCaCBAcCcaDA".to_string();

        let result = react(polymer);

        assert_eq!(result, "dabCBAcaDA");
    }

    #[test]
    fn test_part2() {
        let polymer = "dabAcCaCBAcCcaDA".to_string();

        let result = part2(polymer);

        assert_eq!(result, "daDA");
    }

}
