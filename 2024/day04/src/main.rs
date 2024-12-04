use utils::Grid;

type Input = Grid<u8>;

fn parse(input: String) -> Input {
    let w = input
        .lines()
        .next()
        .map(|l| l.as_bytes().len())
        .unwrap_or(0);
    let d = input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.bytes())
        .collect();
    Grid::from_data(d, w)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> i32 {
    let mut total = 0;

    if input.width >= 4 {
        // not true in tests
        for (_y, row) in input.rows().enumerate() {
            for x in 0..=row.len() - 4 {
                let c = &row[x..x + 4];
                if c == b"XMAS" || c == b"SAMX" {
                    total += 1;
                }
            }
        }
    }

    if input.height >= 4 {
        // not true in tests
        for x in 0..input.width {
            for y in 0..=input.height - 4 {
                let c = [
                    *input.get(x, y),
                    *input.get(x, y + 1),
                    *input.get(x, y + 2),
                    *input.get(x, y + 3),
                ];
                if &c == b"XMAS" || &c == b"SAMX" {
                    total += 1;
                }
            }
        }
    }

    // diag
    if input.height >= 4 && input.width >= 4 {
        for y in 0..=input.height - 4 {
            for x in 0..=input.width - 4 {
                let c = [
                    *input.get(x, y),
                    *input.get(x + 1, y + 1),
                    *input.get(x + 2, y + 2),
                    *input.get(x + 3, y + 3),
                ];
                if &c == b"XMAS" || &c == b"SAMX" {
                    total += 1;
                }
                let c = [
                    *input.get(x, y + 3),
                    *input.get(x + 1, y + 2),
                    *input.get(x + 2, y + 1),
                    *input.get(x + 3, y),
                ];
                if &c == b"XMAS" || &c == b"SAMX" {
                    total += 1;
                }
            }
        }
    }

    total
}

fn part2(input: &Input) -> i32 {
    let mut count = 0;
    for y in 0..=input.height - 3 {
        for x in 0..=input.width - 3 {
            if *input.get(x + 1, y + 1) != b'A' {
                continue;
            }
            let edges = [
                *input.get(x, y),
                *input.get(x + 2, y + 2),
                *input.get(x + 2, y),
                *input.get(x, y + 2),
            ];

            match &edges {
                b"MSMS" | b"SMSM" | b"SMMS" | b"MSSM" => {
                    count += 1;
                }
                _ => {}
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_p1_row() {
        let inp = parse("MMMSXXMASAMX".to_string());
        let res = part1(&inp);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_p1_col() {
        let inp = parse("MMMSXXMASAMX".chars().flat_map(|c| [c, '\n']).collect());
        let res = part1(&inp);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 18);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 9);
    }
}
