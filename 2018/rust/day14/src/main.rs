fn main() {
    let result = part1(84601);
    println!("Part1: {}", result);
    let result = part2("084601");
    println!("Part2: {}", result);
}

fn part1(input: u32) -> String {
    let mut board = vec![3, 7];
    let mut current = [0, 1];
    for _ in 0..input + 10 {
        tick(&mut board, &mut current);
    }
    let input = input as usize;
    board[input..input + 10]
        .iter()
        .map(|d| d.to_string())
        .collect()
}

fn part2(input: &str) -> u32 {
    let pattern: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).expect("Input must be numerical"))
        .map(|d| d as u8)
        .collect();
    let mut board = vec![3, 7];
    let mut current = [0, 1];
    loop {
        tick(&mut board, &mut current);
        let result = || (board.len() - pattern.len()) as u32;
        if ends_in_pattern(board.iter(), pattern.iter(), 0) {
            return result();
        } else if ends_in_pattern(board.iter(), pattern.iter(), 1) {
            return result() - 1;
        }
    }
}

fn ends_in_pattern<'a, I>(board: I, pattern: I, offset: usize) -> bool
where
    I: DoubleEndedIterator<Item = &'a u8>,
{
    board
        .rev()
        .skip(offset)
        .zip(pattern.rev())
        .all(|(x, y)| *x == *y)
}

fn tick(board: &mut Vec<u8>, current: &mut [usize; 2]) {
    let a = board[current[0]];
    let b = board[current[1]];

    let n = a + b;
    board.append(&mut digits(n));

    current[0] = (current[0] + 1 + a as usize) % board.len();
    current[1] = (current[1] + 1 + b as usize) % board.len();
}

fn digits(n: u8) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).expect("Input to digits() must be numerical"))
        .map(|d| d as u8)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_1() {
        let result = part1(9);
        assert_eq!(result, "5158916779");
    }

    #[test]
    fn test_part1_2() {
        let result = part1(5);
        assert_eq!(result, "0124515891");
    }

    #[test]
    fn test_part1_3() {
        let result = part1(18);
        assert_eq!(result, "9251071085");
    }

    #[test]
    fn test_part1_4() {
        let result = part1(2018);
        assert_eq!(result, "5941429882");
    }

    #[test]
    fn test_part2_1() {
        let result = part2("51589");
        assert_eq!(result, 9);
    }

    #[test]
    fn test_part2_2() {
        let result = part2("01245");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2_3() {
        let result = part2("92510");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2_4() {
        let result = part2("59414");
        assert_eq!(result, 2018);
    }

    #[test]
    fn test_has_pattern() {
        let board = vec![0, 1, 2];
        let pattern = vec![0, 1, 2, 3];

        assert!(!ends_in_pattern(board.iter(), pattern.iter(), 0));

        let board = vec![0, 1, 2];
        let pattern = vec![1, 2];

        assert!(ends_in_pattern(board.iter(), pattern.iter(), 0));

        let board = vec![0, 1, 2];
        let pattern = vec![0, 1];

        assert!(!ends_in_pattern(board.iter(), pattern.iter(), 0));

        let board = vec![
            3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2, 5,
        ];
        let pattern = vec![9, 2, 5, 1, 0];

        assert!(!ends_in_pattern(board.iter(), pattern.iter(), 0));

        let board = vec![
            3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2, 5,
        ];
        let pattern = vec![1, 0, 1, 2, 4];

        assert!(!ends_in_pattern(board.iter(), pattern.iter(), 0));

        let board = vec![
            3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2, 5,
        ];
        let pattern = vec![6, 7, 7, 9, 2, 5];

        assert!(ends_in_pattern(board.iter(), pattern.iter(), 0));
    }
}

