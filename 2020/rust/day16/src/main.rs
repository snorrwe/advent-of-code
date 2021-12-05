use std::collections::{HashMap, HashSet};

type Range = [i32; 2];

#[inline(always)]
fn is_in([from, to]: Range, value: i32) -> bool {
    from <= value && value <= to
}

fn parse_rules(inp: &str) -> HashMap<String, Vec<Range>> {
    let mut rules = HashMap::new();
    for line in inp.lines() {
        if line.is_empty() {
            return rules;
        }

        let mut fields = line.split(": ");
        let key = fields.next().expect("key not found");
        let fromto = fields.next().expect("from,to not found").split(" or ");
        let mut rs = Vec::new();

        for fromto in fromto {
            let mut fromto = fromto.split("-");
            let from = fromto
                .next()
                .expect("from not found")
                .parse()
                .expect("from parse");
            let to = fromto
                .next()
                .expect("to not found")
                .parse()
                .expect("to parse");

            rs.push([from, to]);
        }

        rules.insert(key.to_string(), rs);
    }
    rules
}

fn part1(inp: &str, discarded: &mut Vec<usize>) -> i32 {
    let rules = parse_rules(inp);
    let skip = rules.len();

    let mut invalids = vec![];

    for (i, line) in inp.lines().skip(skip + 5).enumerate() {
        // +5 to skip "your ticket"
        for num in line.split(",") {
            let num = num.parse().unwrap();

            if rules
                .values()
                .flat_map(|x| x)
                .all(|range| !is_in(*range, num))
            {
                invalids.push(num);
                discarded.push(i);
            }
        }
    }
    invalids.into_iter().sum()
}

/// return name:value for your ticket
fn solve_ticket(inp: &str, discarded: &[usize]) -> HashMap<String, i32> {
    let rules = parse_rules(inp);
    let skip = rules.len();

    let myticket_values: Vec<i32> = inp
        .lines()
        .skip(skip + 2)
        .next()
        .unwrap()
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut candidates: Vec<HashSet<&str>> = myticket_values
        .iter()
        .map(|i| {
            rules
                .iter()
                // filter out keys that can't represent this field
                .filter_map(|(k, v)| {
                    if v.iter().any(|range| is_in(*range, *i)) {
                        Some(k.as_str())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    for (i, line) in inp.lines().skip(skip + 5).enumerate() {
        // +5 to skip "your ticket"
        if discarded.contains(&i) {
            continue;
        }
        for (i, num) in line.split(",").enumerate() {
            let num = num.parse().unwrap();

            for (rule, _) in rules
                .iter()
                .filter(|(_, v)| v.iter().all(move |range| !is_in(*range, num)))
            {
                candidates[i].remove(rule.as_str());
            }
        }
    }

    let mut to_remove = Vec::with_capacity(candidates.len());

    loop {
        to_remove.clear();
        for (i, rule) in candidates.iter().enumerate() {
            debug_assert!(!rules.is_empty());
            if rule.len() == 1 {
                to_remove.push((i, rule.iter().cloned().next().unwrap()));
            }
        }
        assert!(!to_remove.is_empty());
        if to_remove.len() == candidates.len() {
            break;
        }
        for (i, key) in to_remove.iter() {
            for (j, cand) in candidates.iter_mut().enumerate() {
                if *i != j {
                    cand.remove(*key);
                }
                assert!(
                    cand.len() >= 1,
                    "Removed the last rule for a field {:?}",
                    key
                );
            }
        }
    }

    myticket_values
        .into_iter()
        .zip(candidates.into_iter())
        .map(|(value, v)| {
            let key = v.iter().next().unwrap();
            (key.to_string(), value)
        })
        .collect()
}

fn part2(inp: &str, discarded: &[usize]) -> isize {
    let stuff = solve_ticket(inp, discarded);
    stuff
        .iter()
        .filter(|(k, _)| k.as_str().starts_with("departure"))
        .map(|(_, v)| *v as isize)
        .product()
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let mut discard = Vec::with_capacity(128);
    let res = part1(input.as_str(), &mut discard);
    println!("{}", res);

    let res = part2(input.as_str(), &discard);
    println!("{:?}", res);
}

#[test]
fn test_p1() {
    let inp = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    let mut discard = Vec::with_capacity(128);
    let res = part1(inp, &mut discard);
    assert_eq!(res, 71);
}

#[test]
fn test_p2() {
    let inp = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;

    let res = solve_ticket(inp, &[]);
    assert_eq!(
        res,
        vec![
            ("class".to_string(), 12i32),
            ("row".to_string(), 11),
            ("seat".to_string(), 13),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
    );
}
