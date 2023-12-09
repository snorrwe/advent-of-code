fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn part1(input: &str) -> i32 {
    let mut floor = 0;
    if let Some(line) = input.lines().next() {
        for c in line.chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => {}
            }
        }
    }
    floor
}

fn part2(input: &str) -> usize {
    let mut floor = 0;
    if let Some(line) = input.lines().next() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => {}
            }
            if floor == -1 {
                return i + 1;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
