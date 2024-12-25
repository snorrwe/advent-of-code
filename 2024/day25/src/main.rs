use utils::Grid;

type Input = Vec<Grid<u8>>;

fn parse(input: String) -> Input {
    input
        .split("\n\n")
        .map(|l| Grid::from_ascii_lines(l).unwrap())
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> i32 {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for (i, g) in input.iter().enumerate() {
        assert_eq!(g.width, 5);
        assert_eq!(g.height, 7);

        let mut height = [0; 5];
        for row in g.rows().skip(1).take(5) {
            for x in 0..5usize {
                if row[x] == b'#' {
                    height[x] += 1;
                }
            }
        }

        if g.row(0) == b"#####" {
            locks.push((i, height));
        } else {
            assert_eq!(g.row(6), b"#####");
            keys.push((i, height));
        }
    }

    let mut count = 0;
    for (_, l) in locks.iter() {
        'search: for (_, k) in keys.iter() {
            for (l, k) in l.iter().zip(k.iter()) {
                if l + k > 5 {
                    continue 'search;
                }
            }
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 3);
    }
}
