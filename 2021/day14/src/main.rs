use counter::Counter;
use std::mem::swap;

fn update(inp: &str, out: &mut String, rules: &[Rule]) {
    out.clear();
    for i in 0..inp.len() - 1 {
        out.push(inp.chars().skip(i).next().unwrap());
        match rules.iter().find(|[from, _]| from == &&inp[i..i + 2]) {
            Some([_, to]) => {
                out.push_str(to);
            }
            None => {}
        }
    }
    out.push(inp.chars().skip(inp.len() - 1).next().unwrap());
}

type Rule<'a> = [&'a str; 2];
fn main() {
    let mut template = String::with_capacity(1024);
    std::io::stdin().read_line(&mut template).unwrap();

    let mut buffer = String::with_capacity(1024);
    let mut rules = Vec::<Rule>::new();
    std::io::stdin().read_line(&mut buffer).unwrap(); // empty line
    while let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        let line: &str = buffer[buffer.len() - size..].trim_end();
        if line.len() == 0 {
            break;
        }
    }
    for line in buffer.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let mut s = line.split(" -> ");
        let rule = [s.next().unwrap(), s.next().unwrap()];
        assert!(rule[0].len() == 2);
        assert!(rule[1].len() == 1);
        rules.push(rule);
    }

    let mut a = template;
    let mut b = String::new();

    let i = 10;
    for _ in 0..i {
        update(&a.trim_end(), &mut b, &rules);
        swap(&mut a, &mut b);
    }
    if i % 2 == 1 {
        swap(&mut a, &mut b);
    }

    let count = a.chars().collect::<Counter<_>>();

    let sorted = count.most_common();

    let p1 = sorted[0].1 - sorted.last().unwrap().1;

    println!("p1: {}", p1);
}
