use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let data = data
        .trim()
        .split(' ')
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let tree = build_tree(&mut data.iter());

    let result = part1(&tree);
    println!("Part1: {}", result);
    let result = part2(&tree);
    println!("Part2: {}", result);
    Ok(())
}

fn part1(tree: &Tree) -> usize {
    let mut result = 0;
    result += tree.metadata.iter().sum::<usize>();
    for child in tree.children.iter() {
        result += part1(child);
    }
    result
}

fn part2(tree: &Tree) -> usize {
    if tree.children.len() == 0 {
        tree.metadata.iter().sum()
    } else {
        tree.metadata
            .iter()
            .map(|i| {
                let child = tree.children.get(i - 1);
                if let Some(ref child) = child {
                    part2(child)
                } else {
                    0
                }
            })
            .sum()
    }
}

fn build_tree<'a, I>(input: &mut I) -> Tree
where
    I: Iterator<Item = &'a usize>,
{
    let n_children = input.next().expect("Unexpected end of input1");
    let n_meta = input.next().expect("Unexpected end of input2");
    let mut children = vec![];
    for _ in 0..*n_children {
        children.push(build_tree(input));
    }
    let mut meta = vec![];
    for _ in 0..*n_meta {
        let input = input.next();
        meta.push(*input.expect("Unexpected end of input3"));
    }
    let result = Tree {
        metadata: meta,
        children: children,
    };
    result
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

        let result = build_tree(&mut input.iter());

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

