use crate::{Token, TokenTy};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
}

fn run_one(stack: &mut VecDeque<usize>, ops: &mut VecDeque<Op>, start: usize) {
    let mut i = start;
    for j in start..ops.len() {
        if matches!(ops[j], Op::Add) {
            i = j;
            break;
        }
    }
    let a = stack[i];
    let b = stack[i + 1];
    stack[i] = match ops[i] {
        Op::Mul => a * b,
        Op::Add => a + b,
    };
    stack.remove(i + 1);
    ops.remove(i);
}

pub fn execute(line: &str, tokens: &[Token]) -> usize {
    let line = line.as_bytes();
    let mut stack = VecDeque::new();
    let mut ops: VecDeque<Op> = VecDeque::new();
    let mut parens_depth = vec![];

    for token in tokens {
        match token.ty {
            TokenTy::Scalar => {
                let off = token.offset as usize;
                let len = token.len as usize;
                let scalar = &line[off..off + len];
                let scalar: usize =
                    unsafe { std::str::from_utf8_unchecked(scalar).parse().unwrap() };
                stack.push_back(scalar);
            }
            TokenTy::Mul => {
                ops.push_back(Op::Mul);
                if let Some(depth) = parens_depth.last_mut() {
                    *depth += 1
                }
            }
            TokenTy::Add => {
                ops.push_back(Op::Add);
                if let Some(depth) = parens_depth.last_mut() {
                    *depth += 1
                }
            }
            TokenTy::LeftParen => {
                parens_depth.push(0);
            }
            TokenTy::RightParen => {
                let depth = parens_depth.pop().unwrap();
                let offset = ops.len() - depth;
                for _ in 0..depth {
                    run_one(&mut stack, &mut ops, offset);
                }
            }
        }
    }

    while !ops.is_empty() {
        run_one(&mut stack, &mut ops, 0);
    }

    stack.pop_back().unwrap()
}

#[test]
fn test_p2() {
    let inp = r#"2 * 3 + (4 * 5)
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
"#;
    let tokens = crate::tokenize(inp);
    let res = tokens
        .iter()
        .zip(inp.lines())
        .map(|(t, l)| execute(l, t.as_slice()))
        .collect::<Vec<_>>();

    assert_eq!(res.as_slice(), &[46, 23340]);
}
