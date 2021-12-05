use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut data_it = data
        .trim()
        .split(' ')
        .filter_map(|c| c.parse::<usize>().ok());

    let tree = build_tree(&mut data_it);

    let result = part1(&tree);
    println!("Part1: {}", result);
    let result = part2(&tree);
    println!("Part2: {}", result);
    Ok(())
}

fn part1(tree: &Tree) -> usize {
    tree.metadata.iter().sum::<usize>()
        + tree
            .children
            .iter()
            .map(|child| part1(child))
            .sum::<usize>()
}

fn part2(tree: &Tree) -> usize {
    if tree.children.is_empty() {
        tree.metadata.iter().sum()
    } else {
        tree.metadata
            .iter()
            .map(|i| {
                let child = tree.children.get(i - 1);
                child.map_or(0, |child| part2(child))
            })
            .sum()
    }
}

fn build_tree<'a, I>(input: &mut I) -> Tree
where
    I: Iterator<Item = usize>,
{
    let n_children = input.next().expect("Unexpected end of input1");
    let n_meta = input.next().expect("Unexpected end of input2");
    let children = (0..n_children).map(|_| build_tree(input)).collect();
    let meta = (0..n_meta).filter_map(|_| input.next()).collect();
    Tree {
        metadata: meta,
        children: children,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Tree {
    metadata: Vec<usize>,
    children: Vec<Tree>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tree_building() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];

        let result = build_tree(&mut input.iter().map(|x| *x));

        let expected = Tree {
            metadata: vec![1, 1, 2],
            children: vec![
                Tree {
                    metadata: vec![10, 11, 12],
                    children: vec![],
                },
                Tree {
                    metadata: vec![2],
                    children: vec![Tree {
                        metadata: vec![99],
                        children: vec![],
                    }],
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let tree = Tree {
            metadata: vec![1, 1, 2],
            children: vec![
                Tree {
                    metadata: vec![10, 11, 12],
                    children: vec![],
                },
                Tree {
                    metadata: vec![2],
                    children: vec![Tree {
                        metadata: vec![99],
                        children: vec![],
                    }],
                },
            ],
        };

        let result = part1(&tree);

        assert_eq!(result, 138);
    }

    #[test]
    fn test_part2() {
        let tree = Tree {
            metadata: vec![1, 1, 2],
            children: vec![
                Tree {
                    metadata: vec![10, 11, 12],
                    children: vec![],
                },
                Tree {
                    metadata: vec![2],
                    children: vec![Tree {
                        metadata: vec![99],
                        children: vec![],
                    }],
                },
            ],
        };

        let result = part2(&tree);

        assert_eq!(result, 66);
    }
}

