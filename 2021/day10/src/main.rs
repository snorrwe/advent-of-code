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

fn score(end: char) -> usize {
    match end {
        '}' => 1197,
        ')' => 3,
        ']' => 57,
        '>' => 25137,
        _ => 0,
    }
}

fn score_v2(end: char) -> usize {
    match end {
        '{' => 3,
        '(' => 1,
        '[' => 2,
        '<' => 4,
        _ => 0,
    }
}

fn parse_line(line: &str, p1: &mut usize, s: &mut Vec<char>) -> bool {
    s.clear();
    for token in line.chars() {
        if is_start(token) {
            s.push(token);
        } else {
            match s.last() {
                Some(c) => {
                    if matches(*c, token) {
                        s.pop();
                    } else {
                        *p1 += score(token);
                        return false;
                    }
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
    let mut s = Vec::new();
    let mut p1 = 0;
    let mut p2 = Vec::new();
    while let Ok(_size) = std::io::stdin().read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end();
        if line.len() == 0 {
            break;
        }
        if parse_line(line, &mut p1, &mut s) {
            let mut score = 0;
            while let Some(c) = s.pop() {
                score *= 5;
                score += score_v2(c);
            }
            p2.push(score);
            // incomplete
        }
        buffer.clear();
    }

    p2.sort();
    let p2 = p2[p2.len() / 2];

    println!("P1: {} P2: {}", p1, p2);
}
