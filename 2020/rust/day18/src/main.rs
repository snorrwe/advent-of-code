mod part1;
mod part2;

#[derive(Debug, Clone, Copy)]
pub struct Token {
    ty: TokenTy,
    offset: u32,
    len: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenTy {
    Scalar,
    Mul,
    Add,
    LeftParen,
    RightParen,
}

fn tokenize_line(line: &str) -> Vec<Token> {
    let line = line.as_bytes();

    let mut i = 0;
    let len = line.len();

    let mut tokens = vec![];
    while i < len {
        let byte = line[i] as char;
        match byte {
            '(' => tokens.push(Token {
                ty: TokenTy::LeftParen,
                len: 1,
                offset: i as u32,
            }),
            ')' => tokens.push(Token {
                ty: TokenTy::RightParen,
                len: 1,
                offset: i as u32,
            }),
            '+' => tokens.push(Token {
                ty: TokenTy::Add,
                len: 1,
                offset: i as u32,
            }),
            '*' => tokens.push(Token {
                ty: TokenTy::Mul,
                len: 1,
                offset: i as u32,
            }),
            c if c.is_digit(10) => {
                let mut count = 0;
                let j = i;
                while line
                    .get(i)
                    .map(|c| (*c as char))
                    .map(|c| c.is_digit(10))
                    .unwrap_or(false)
                {
                    count += 1;
                    i += 1
                }
                tokens.push(Token {
                    ty: TokenTy::Scalar,
                    len: count,
                    offset: j as u32,
                });
                continue;
            }
            _ => {}
        }
        i += 1;
    }

    tokens
}

fn tokenize(inp: &str) -> Vec<Vec<Token>> {
    inp.lines().map(tokenize_line).collect()
}

fn part1(inp: &str) -> usize {
    let tokens = tokenize(inp);
    let res = tokens
        .iter()
        .zip(inp.lines())
        .map(|(t, l)| part1::execute(l, t.as_slice()))
        .collect::<Vec<_>>();

    res.iter().cloned().sum()
}

fn part2(inp: &str) -> usize {
    let tokens = tokenize(inp);
    let res = tokens
        .iter()
        .zip(inp.lines())
        .map(|(t, l)| part2::execute(l, t.as_slice()))
        .collect::<Vec<_>>();

    res.iter().cloned().sum()
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let res = part1(input.as_str());
    println!("{}", res);
    let res = part2(input.as_str());
    println!("{}", res);
}
