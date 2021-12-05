use crate::{Token, TokenTy};
use std::collections::VecDeque;

pub fn execute(line: &str, tokens: &[Token]) -> usize {
    let line = line.as_bytes();
    let mut stack = VecDeque::new();
    let mut ops: VecDeque<fn(usize, usize) -> usize> = VecDeque::new();
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
                ops.push_back(|a, b| a * b);
                if let Some(depth) = parens_depth.last_mut() {
                    *depth += 1
                }
            }
            TokenTy::Add => {
                ops.push_back(|a, b| a + b);
                if let Some(depth) = parens_depth.last_mut() {
                    *depth += 1
                }
            }
            TokenTy::LeftParen => {
                parens_depth.push(0);
            }
            TokenTy::RightParen => {
                let depth = parens_depth.pop().unwrap();
                let start = ops.len() - depth;
                for i in start..ops.len() {
                    let op = ops[i];
                    let a = stack[i];
                    let b = stack[i + 1];
                    stack[i + 1] = op(a, b);
                }
                // write the final result to the beginning
                stack[start] = stack[ops.len()];
                // remove no longer needed ops and values
                for _ in 0..depth {
                    ops.pop_back();
                    stack.pop_back();
                }
            }
        }
    }

    for op in ops.into_iter() {
        let b = stack.pop_front().unwrap();
        let a = stack.pop_front().unwrap();
        stack.push_front(op(a, b));
    }

    stack.pop_back().unwrap()
}

#[test]
fn test_p1() {
    let inp = r#"2 * 3 + (4 * 5)
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 20
"#;
    let tokens = crate::tokenize(inp);
    let res = tokens
        .iter()
        .zip(inp.lines())
        .map(|(t, l)| execute(l, t.as_slice()))
        .collect::<Vec<_>>();

    assert_eq!(res.as_slice(), &[26, 136320]);
}
