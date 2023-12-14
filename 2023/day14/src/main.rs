type Grid = utils::Grid<u8>;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn part1(input: &str) -> usize {
    let Some(width) = input.lines().next().and_then(|l| {
        let l = l.len();
        (l != 0).then_some(l)
    }) else {
        return 0;
    };

    let terrain = Grid::from_data(
        input
            .lines()
            .flat_map(|l| l.bytes())
            .map(|c| match c {
                b'.' | b'#' => c,
                b'O' => b'.',
                _ => unreachable!(),
            })
            .collect(),
        width,
    );

    let mut rocks = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| c == &b'O')
        .map(|(x, y, _)| (x, y))
        .collect::<Vec<_>>();

    let mut rocks_grid = Grid::new(terrain.width, terrain.height);
    rocks_grid.fill(0);
    for (x, y) in rocks.iter() {
        rocks_grid.insert(*x, *y, 1);
    }

    loop {
        let mut changed = false;

        for rock in rocks.iter_mut() {
            if rock.1 == 0 {
                continue;
            }
            let (x, y) = *rock;
            if *terrain.get(x, y - 1) == b'.' && *rocks_grid.get(x, y - 1) == 0 {
                changed = true;
                rocks_grid.insert(x, y, 0);
                rocks_grid.insert(x, y - 1, 1);
                *rock = (rock.0, rock.1 - 1);
            }
        }

        if !changed {
            break;
        }
    }

    let height = rocks_grid.height;
    rocks_grid
        .rows()
        .enumerate()
        .map(|(y, row)| (height - y) * row.iter().filter(|c| **c == 1).count())
        .sum()
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 136);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 42);
    }
}
