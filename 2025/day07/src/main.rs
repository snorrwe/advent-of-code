use std::collections::HashSet;

use utils::Grid;

type Input = Grid<u8>;

fn parse(input: &'_ str) -> Input {
    Grid::from_ascii_lines(input).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> i32 {
    let mut splits = 0;
    let mut beams = HashSet::new();
    let mut bbuffer = HashSet::new();

    for x in 0..input.width {
        if input.get(x, 0) == &b'S' {
            beams.insert(x);
            break;
        }
    }

    for y in 1..input.height {
        bbuffer.clear();
        for x in beams.drain() {
            if input.get(x, y) == &b'^' {
                bbuffer.insert(x - 1);
                bbuffer.insert(x + 1);
                splits += 1;
            } else {
                bbuffer.insert(x);
            }
        }
        std::mem::swap(&mut beams, &mut bbuffer);
    }
    splits
}

fn part2(input: &Input) -> usize {
    let mut row = vec![1; input.width];
    for y in (0..input.height).rev() {
        for x in 0..input.width {
            match input.get(x, y) {
                b'^' => {
                    row[x] = row[x - 1] + row[x + 1];
                }
                b'S' => {
                    return row[x];
                }
                _ => {}
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 21);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 40);
    }
}
