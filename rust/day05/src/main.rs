use std::io::Read;

/// max inclusive
fn binary(chars: &str, mut min: u32, mut max: u32, split_left: char, split_right: char) -> u32 {
    debug_assert!(min < max);
    debug_assert!((max & 1) == 1, "max must be odd");
    max = max + 1;
    for chr in chars.chars() {
        let mid = (max + min) / 2;
        debug_assert!(mid > 0);

        if chr == split_left {
            max = mid;
        } else if chr == split_right {
            min = mid;
        }
    }
    debug_assert!(max > 0);
    debug_assert!(min < max);
    debug_assert_eq!(min + 1, max);

    min
}

/// The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127)
fn row(line: &str) -> u32 {
    binary(&line[..7], 0, 127, 'F', 'B')
}

/// The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7)
fn column(line: &str) -> u32 {
    binary(&line[7..10], 0, 7, 'L', 'R')
}

fn seat_id(line: &str) -> u32 {
    let row = row(line);
    let col = column(line);
    row * 8 + col
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    let max = input.lines().map(|line| seat_id(line)).max().unwrap();

    println!("p1: {:?}", max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_p1() {
        let inp = r#"FBFBBFFRLR"#;

        let res = row(inp);

        assert_eq!(res, 44);
    }

    #[test]
    fn test_col_p1() {
        let inp = r#"FBFBBFFRLR"#;

        let res = column(inp);

        assert_eq!(res, 5);
    }

    #[test]
    fn test_part1_1() {
        let res = seat_id("FBFBBFFRLR");
        assert_eq!(res, 357);
    }

    #[test]
    fn test_part1_2() {
        let inp = "BFFFBBFRRR";
        let res = seat_id(inp);
        assert_eq!(res, 567);
    }

    #[test]
    fn test_part1_3() {
        let res = seat_id("FFFBBBFRRR");
        assert_eq!(res, 119);
    }

    #[test]
    fn test_part1_4() {
        let res = seat_id("BBFFBBFRLL");
        assert_eq!(res, 820);
    }
}
