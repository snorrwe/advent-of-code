fn is_start(c: char) -> bool {
    matches!(c, '[' | '(' | '{' | '<')
}

fn matches(start: char, end: char) -> bool {
    match start {
        '{' => end == '}',
        '(' => end == ')',
        '[' => end == ']',
        '<' => end == '>',
        _ => false,
    }
}

fn score(c: char) -> usize {
    match c {
        '}' => 1197,
        ')' => 3,
        ']' => 57,
        '>' => 25137,
        '{' => 3,
        '(' => 1,
        '[' => 2,
        '<' => 4,
        _ => 0,
    }
}

fn parse_line(line: &str, p1: &mut usize, stack: &mut Vec<char>) -> bool {
    stack.clear();
    for token in line.chars() {
        if is_start(token) {
            stack.push(token);
        } else {
            match stack.last().filter(|c| matches(**c, token)) {
                Some(_) => {
                    stack.pop();
                }
                None => {
                    *p1 += score(token);
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let mut buffer = String::with_capacity(1024);
    let mut stack = Vec::new();
    let mut p1 = 0;
    let mut p2 = Vec::new();
    while let Ok(_size) = std::io::stdin().read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end();
        if line.len() == 0 {
            break;
        }
        if parse_line(line, &mut p1, &mut stack) {
            let mut acc = 0;
            while let Some(c) = stack.pop() {
                acc *= 5;
                acc += score(c);
            }
            p2.push(acc);
        }
        buffer.clear();
    }

    p2.sort();
    let p2 = p2[p2.len() / 2];

    println!("P1: {} P2: {}", p1, p2);
}
