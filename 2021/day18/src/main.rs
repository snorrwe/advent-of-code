#![feature(let_else)]

use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnailfishNumber {
    values: Vec<i32>,
    head: Tree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tree {
    Value(usize),
    Pair { x: Box<Tree>, y: Box<Tree> },
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::fmt_pls(&self.values[..], &self.head, f)
    }
}

impl SnailfishNumber {
    fn fmt_pls(values: &[i32], t: &Tree, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match t {
            Tree::Value(x) => write!(f, "{}", values[*x]),
            Tree::Pair { x, y } => {
                write!(f, "[")?;
                Self::fmt_pls(values, x, f)?;
                write!(f, ",")?;
                Self::fmt_pls(values, y, f)?;
                write!(f, "]")?;
                Ok(())
            }
        }
    }

    pub fn parse(line: &str) -> Self {
        let mut s = Vec::new();
        let mut values = Vec::new();
        Self::_parse(line.trim_end(), &mut values, &mut s);
        assert_eq!(s.len(), 1, "{:?}\n{:?}", s, values);
        let head = s.pop().unwrap();
        Self { values, head }
    }

    fn _parse(line: &str, values: &mut Vec<i32>, stack: &mut Vec<Tree>) {
        for (i, c) in line.chars().enumerate() {
            match c {
                '[' => {
                    Self::_parse(&line[i + 1..], values, stack);
                    break;
                }
                ']' => {
                    // reduce
                    let val = Tree::Pair {
                        // correct order of pops is y x!!
                        y: Box::new(stack.pop().unwrap()),
                        x: Box::new(stack.pop().unwrap()),
                    };
                    stack.push(val)
                }
                ',' => {
                    Self::_parse(&line[i + 1..], values, stack);
                    break;
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    stack.push(Tree::Value(values.len()));
                    values.push((c as u8 - '0' as u8) as i32);
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn magnitude(&self) -> i32 {
        Self::_magnitude(&self.values[..], &self.head)
    }

    fn _magnitude(values: &[i32], head: &Tree) -> i32 {
        match head {
            Tree::Value(x) => values[*x],
            Tree::Pair { x, y } => {
                3 * Self::_magnitude(values, x) + 2 * Self::_magnitude(values, y)
            }
        }
    }

    pub fn reduce(&mut self) -> bool {
        let res = Self::_reduce(&mut self.values, &mut self.head, 0);
        if res {
            self.head.rebuild();
        }
        res
    }

    /// return if an action was taken
    fn _reduce(values: &mut Vec<i32>, head: &mut Tree, depth: usize) -> bool {
        match head {
            Tree::Value(x) => {
                let x = *x;
                let xval = values[x];
                if xval < 10 {
                    return false;
                }
                // split
                let lhs = xval / 2;
                let rhs = (xval as f32 / 2.0).ceil() as i32;
                values[x] = lhs;
                values.insert(x + 1, rhs);
                *head = Tree::Pair {
                    x: Box::new(Tree::Value(0)),
                    y: Box::new(Tree::Value(0)),
                };
                return true;
            }

            Tree::Pair {
                ref mut x,
                ref mut y,
            } => {
                match (&mut **x, &mut **y) {
                    (Tree::Value(ix), Tree::Value(iy)) => {
                        if depth >= 4 {
                            // explode
                            if *ix > 0 {
                                values[*ix - 1] += values[*ix];
                            }
                            if *iy + 1 < values.len() {
                                values[*iy + 1] += values[*iy];
                            }
                            values[*ix] = 0;
                            values.remove(*iy);
                            *head = Tree::Value(*ix);
                            return true;
                        }
                    }
                    _ => {}
                }
                let x = Self::_reduce(values, x, depth + 1);
                if x {
                    return x;
                }
                Self::_reduce(values, y, depth + 1)
            }
        }
    }

    pub fn just_add(mut self, rhs: Self) -> Self {
        self.values.extend_from_slice(rhs.values.as_slice());
        self.head = Tree::Pair {
            x: Box::new(self.head),
            y: Box::new(rhs.head),
        };
        self.head.rebuild();
        self
    }
}

impl Tree {
    pub fn walk_values(&mut self, action: &mut impl FnMut(&mut usize)) {
        match self {
            Tree::Value(x) => {
                action(x);
            }
            Tree::Pair { x, y } => {
                x.walk_values(action);
                y.walk_values(action);
            }
        }
    }

    pub fn rebuild(&mut self) {
        let mut i = 0;
        self.walk_values(&mut |j| {
            *j = i;
            i += 1;
        });
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.just_add(rhs);
        while result.reduce() {}
        result
    }
}

fn main() {
    let mut buffer = String::with_capacity(1024);
    while let Ok(_size) = std::io::stdin().read_line(&mut buffer) {
        if _size == 0 {
            break;
        }
    }
    let mut lines = buffer.lines();
    let mut num = SnailfishNumber::parse(lines.next().unwrap());
    for line in lines {
        num = num + SnailfishNumber::parse(line);
    }

    // < 3919
    println!("{}", num.magnitude());
}

#[test]
fn test_parse() {
    let num = SnailfishNumber::parse("[1,2]");

    assert_eq!(num.values, vec![1, 2]);
    assert_eq!(
        num.head,
        Tree::Pair {
            x: Box::new(Tree::Value(0)),
            y: Box::new(Tree::Value(1)),
        }
    );

    let num = SnailfishNumber::parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
    assert_eq!(num.values, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(
        num.head,
        Tree::Pair {
            x: Box::new(Tree::Pair {
                x: Box::new(Tree::Pair {
                    x: Box::new(Tree::Pair {
                        x: Box::new(Tree::Value(0)),
                        y: Box::new(Tree::Value(1)),
                    }),
                    y: Box::new(Tree::Pair {
                        x: Box::new(Tree::Value(2)),
                        y: Box::new(Tree::Value(3)),
                    })
                }),
                y: Box::new(Tree::Pair {
                    x: Box::new(Tree::Pair {
                        x: Box::new(Tree::Value(4)),
                        y: Box::new(Tree::Value(5)),
                    }),
                    y: Box::new(Tree::Pair {
                        x: Box::new(Tree::Value(6)),
                        y: Box::new(Tree::Value(7)),
                    })
                }),
            }),
            y: Box::new(Tree::Value(8)),
        }
    );
}

#[test]
fn test_add() {
    let lhs = SnailfishNumber::parse("[1,2]");
    let rhs = SnailfishNumber::parse("[[3,4],5]");

    let result = lhs + rhs;

    assert_eq!(result.values, vec![1, 2, 3, 4, 5]);
    let exp = Tree::Pair {
        x: Box::new(Tree::Pair {
            x: Box::new(Tree::Value(0)),
            y: Box::new(Tree::Value(1)),
        }),
        y: Box::new(Tree::Pair {
            x: Box::new(Tree::Pair {
                x: Box::new(Tree::Value(2)),
                y: Box::new(Tree::Value(3)),
            }),
            y: Box::new(Tree::Value(4)),
        }),
    };
    assert_eq!(result.head, exp);

    let num =
        SnailfishNumber::parse("[[[[4,3],4],4],[7,[[8,4],9]]]") + SnailfishNumber::parse("[1,1]");
    assert_eq!(
        num,
        SnailfishNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    )
}

#[test]
fn test_explode() {
    for (from, to) in [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
    ] {
        let mut num = SnailfishNumber::parse(from);
        let res = num.reduce();
        dbg!(from, to);
        assert!(res);
        assert_eq!(num, SnailfishNumber::parse(to));
    }
}

#[test]
fn test_split() {
    let mut num = SnailfishNumber::parse("9");
    num.values[0] = 10;
    assert!(num.reduce());
    assert_eq!(num, SnailfishNumber::parse("[5,5]"));
}

#[test]
fn full() {
    let lhs = SnailfishNumber::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let rhs = SnailfishNumber::parse("[1,1]");

    let num = lhs + rhs;

    assert_eq!(
        num,
        SnailfishNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    )
}

#[test]
fn homework() {
    let input = [
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        "[7,[5,[[3,8],[1,4]]]]",
        "[[2,[2,2]],[8,[8,1]]]",
        "[2,9]",
        "[1,[[[9,3],9],[[9,0],[0,7]]]]",
        "[[[5,[7,4]],7],1]",
        "[[[[4,2],2],6],[8,7]]",
    ];

    let mut num = SnailfishNumber::parse(input[0]);
    for line in &input[1..] {
        let rhs = SnailfishNumber::parse(line);
        num = num + rhs;
    }

    let exp = SnailfishNumber::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    assert_eq!(num, exp, "\n{}\n{}", num, exp)
}

#[test]
fn magnitude() {
    let num = SnailfishNumber::parse("[[1,2],[[3,4],5]]");
    assert_eq!(num.magnitude(), 143);

    let num = SnailfishNumber::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    assert_eq!(num.magnitude(), 3488);
}

#[test]
fn homework_simple() {
    let input = ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"];

    let mut num = SnailfishNumber::parse(input[0]);
    for line in &input[1..] {
        let rhs = SnailfishNumber::parse(line);
        num = num + rhs;
    }

    let exp = SnailfishNumber::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");
    assert_eq!(num, exp, "\n{}\n{}", num, exp)
}
