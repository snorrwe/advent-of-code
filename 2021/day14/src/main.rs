use std::{collections::HashMap, mem::swap};

fn update(
    from: &HashMap<String, usize>,
    to: &mut HashMap<String, usize>,
    rules: &HashMap<&str, &str>,
) {
    to.clear();
    let mut tmp = String::new();
    for (k, count) in from.iter() {
        tmp.clear();
        let mut khr = k.chars();
        tmp.push(khr.next().unwrap());
        let rule = rules[k.as_str()];
        tmp.push_str(rule);
        tmp.push(khr.next().unwrap());

        *to.entry(tmp[0..2].to_string()).or_default() += *count;
        *to.entry(tmp[1..3].to_string()).or_default() += *count;
    }
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
    let rules = rules
        .into_iter()
        .map(|[k, v]| (k, v))
        .collect::<HashMap<_, _>>();
    let mut a = rules
        .keys()
        .map(|k| (k.to_string(), 0usize))
        .collect::<HashMap<_, _>>();

    // apply seed
    let template = template.trim_end();
    for i in 0..template.len() - 1 {
        *a.entry(template[i..=i + 1].to_string()).or_default() += 1;
    }

    let mut b = HashMap::new();

    for _ in 0..40 {
        update(&a, &mut b, &rules);
        swap(&mut a, &mut b);
    }

    dbg!(&a);

    let mut cnt = HashMap::new();
    for (k, v) in a.into_iter() {
        for c in k.chars().take(1) {
            *cnt.entry(c).or_insert(0usize) += v;
        }
    }

    let min = cnt.values().min().unwrap();
    let max = cnt.values().max().unwrap();

    // I'm straight up not having a good time
    dbg!(max - min + 1);
}
