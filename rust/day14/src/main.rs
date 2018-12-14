fn main() {
    let result = part1(84601);
    println!("Part1: {}", result);
}

fn digits(n: u8) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).expect("Input to digits() must be numerical"))
        .map(|d| d as u8)
        .collect()
}

fn part1(input: u32) -> String {
    let mut board = vec![3, 7];
    let mut current = [0, 1];
    for _ in 0..input+10 {
        let a = board[current[0]];
        let b = board[current[1]];

        let n = a + b;
        board.append(&mut digits(n));

        current[0] = (current[0] + 1 + a as usize) % board.len();
        current[1] = (current[1] + 1 + b as usize) % board.len();
    }
    let input = input as usize;
    board[input..input + 10]
        .iter()
        .map(|d| d.to_string())
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
}

