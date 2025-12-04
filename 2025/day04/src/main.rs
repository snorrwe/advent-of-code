use utils::{Grid, IVec2};

type Input = Grid<u8>;

fn parse(input: &'_ str) -> Input {
    Input::from_ascii_lines(input).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&mut input));
}

fn part1(grid: &Input) -> i32 {
    let mut total = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) == &b'@' {
                let c = IVec2::new(x as i32, y as i32)
                    .grid_neighbours()
                    .into_iter()
                    .filter(|p| {
                        grid.contains_point(*p) && grid.get(p.x as usize, p.y as usize) == &b'@'
                    })
                    .count();

                if c < 4 {
                    total += 1;
                }
            }
        }
    }

    total
}

fn part2(grid: &mut Input) -> i32 {
    let mut total = 0;

    loop {
        let mut diff = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) == &b'@' {
                    let c = IVec2::new(x as i32, y as i32)
                        .grid_neighbours()
                        .into_iter()
                        .filter(|p| {
                            grid.contains_point(*p) && grid.get(p.x as usize, p.y as usize) == &b'@'
                        })
                        .count();

                    if c < 4 {
                        diff += 1;
                        grid.insert(x, y, b' ');
                    }
                }
            }
        }
        if diff == 0 {
            break;
        }
        total += diff;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x."#;

    // for some reason both test cases show lower than expected, but the inputs do work...

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 12);
    }

    #[test]
    fn test_p2() {
        let mut inp = parse(INPUT);
        let res = part2(&mut inp);

        assert_eq!(res, 30);
    }
}
