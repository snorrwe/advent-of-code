mod inp;

use du_core::ndarray::{Data, NdArray};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Plain,
    Tree,
}

type Grid = NdArray<Tile>;

fn parse_map(inp: &str) -> Grid {
    let mut res = Data::new();
    let mut rows = 0;
    'outer: for line in inp.split("\n") {
        let mut seen = false;
        for chr in line.chars() {
            match chr {
                '.' => {
                    seen = true;
                    res.push(Tile::Plain)
                }
                '#' => {
                    seen = true;
                    res.push(Tile::Tree)
                }
                ' ' | '\n' => {}
                _ => continue 'outer,
            }
        }
        rows += seen as u32;
    }
    let cols = res.len() as u32 / rows;
    println!("{} {} {}", res.len(), rows, cols);
    NdArray::new_with_values([rows, cols], res).unwrap()
}

/// return the number of trees
fn traverse(velx: i32, vely: i32, grid: &Grid) -> usize {
    let mut res = 0;

    let mut x = 0;
    let mut y = 0;

    let [rows, cols] = grid.shape().last_two().unwrap();
    let [rows, cols] = [rows as i32, cols as i32];

    while y < rows {
        // acounting for negative coordinates
        let j = (((x % cols) + cols) % cols) as u32;
        let i = (((y % rows) + rows) % rows) as u32;
        match grid.get(&[i, j]).unwrap() {
            Tile::Plain => {}
            Tile::Tree => res += 1,
        }
        x += velx;
        y += vely;
    }

    res
}

fn main() {
    let grid = parse_map(inp::input());

    let vels = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut part2 = 1;
    for (x, y) in vels.iter() {
        let res = traverse(*x, *y, &grid);
        part2 *= res;
        println!("{} {}: {}", x, y, res);
    }
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let grid = parse_map(
            r#"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
            "#,
        );

        let res = traverse(3, 1, &grid);

        assert_eq!(res, 7);
    }
}
