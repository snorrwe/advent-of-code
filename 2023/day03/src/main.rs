use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = parse(&input);
    let result = part1(&grid);
    println!("part1: {result}");
    let result = part2(&grid);
    println!("part2: {result}");
}

#[derive(Default, Clone, Copy, Debug)]
enum Item {
    #[default]
    None,
    Punctuation(char),
    Number {
        value: i32,
        span: usize,
    },
}

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, item: T) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[y * self.width + x] = item;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.data[y * self.width + x]
    }
}

fn parse(input: &str) -> Grid<Item> {
    let width = input.lines().next().unwrap().len();
    let height = input.len() / (width + 1);
    let mut grid = Grid::new(width, height);
    let num = regex::Regex::new(r"(\d+)").unwrap();
    for (y, line) in input.lines().enumerate() {
        for m in num.find_iter(line) {
            let start = m.start();
            let end = m.end();
            let value: i32 = m.as_str().parse().unwrap();
            for x in start..end {
                grid.insert(
                    x,
                    y,
                    Item::Number {
                        value,
                        span: end - start,
                    },
                );
            }
        }
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c.is_ascii_punctuation() {
                grid.insert(x, y, Item::Punctuation(c));
            }
        }
    }
    grid
}

fn part1(grid: &Grid<Item>) -> i32 {
    let mut sum = 0;
    let width = grid.width;
    let height = grid.height;

    for y in 0..height {
        let mut x = 0;
        'line: while x < width {
            if let Item::Number { value, span } = grid.get(x, y) {
                let startx = if x == 0 { x } else { x - 1 };
                let endx = if x + span == width {
                    x + span
                } else {
                    x + span + 1
                };

                let starty = if y == 0 { y } else { y - 1 };
                let endy = if y + 1 == height { y } else { y + 1 };

                for yy in starty..=endy {
                    for xx in startx..endx {
                        if let Item::Punctuation(_) = grid.get(xx, yy) {
                            sum += *value;
                            x += span;
                            continue 'line;
                        }
                    }
                }
                x += span;
            } else {
                x += 1;
            }
        }
    }

    sum
}

fn part2(grid: &Grid<Item>) -> i32 {
    let mut sum = 0;
    let width = grid.width;
    let height = grid.height;

    let mut adjacent = HashSet::<i32>::default();
    for y in 0..height {
        for x in 0..width {
            if let Item::Punctuation('*') = grid.get(x, y) {
                adjacent.clear();
                let startx = if x == 0 { x } else { x - 1 };
                let endx = if x + 1 == width { x } else { x + 1 };

                let starty = if y == 0 { y } else { y - 1 };
                let endy = if y + 1 == height { y } else { y + 1 };

                for yy in starty..=endy {
                    for xx in startx..=endx {
                        if let Item::Number { value, .. } = grid.get(xx, yy) {
                            adjacent.insert(*value);
                        }
                    }
                }
                if adjacent.len() == 2 {
                    sum += adjacent.iter().copied().product::<i32>();
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn test_p1() {
        let grid = parse(INPUT);
        let result = part1(&grid);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_p2() {
        let grid = parse(INPUT);
        let result = part2(&grid);
        assert_eq!(result, 467835);
    }
}
