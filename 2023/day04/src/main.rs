use std::collections::HashSet;

fn main() {
    let inp = std::fs::read_to_string("input.txt").unwrap();
    let res = part1(&inp);
    println!("part1: {res}");
}

fn part1(inp: &str) -> i32 {
    let cardre = regex::Regex::new(r"Card\s+(\d+):(.*)").unwrap();
    let numre = regex::Regex::new(r"(\d+)").unwrap();

    let mut winning_numbers = HashSet::<&str>::new();

    let mut total = 0;
    for line in inp.lines() {
        let Some(cap) = cardre.captures(line) else {
            continue;
        };
        let (_, [_id, cards]) = cap.extract();
        let (winning, elf) = cards.split_once('|').unwrap();
        winning_numbers.clear();

        for n in numre.captures_iter(winning) {
            let (_, [n]) = n.extract();
            winning_numbers.insert(n);
        }

        let mut count = 0;
        for n in numre.captures_iter(elf) {
            let (_, [n]) = n.extract();
            if winning_numbers.contains(&n) {
                count += 1;
            }
        }
        if count > 0 {
            total += 1 << (count - 1);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn test_p1() {
        let result = part1(INPUT);

        assert_eq!(result, 13);
    }
}
