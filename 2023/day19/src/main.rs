use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

#[derive(Clone, Copy, Debug)]
enum Next<'a> {
    Accept,
    Reject,
    Continue,
    Rule(&'a str),
}

type Parts<'a> = HashMap<&'a str, i32>;

type Workflows<'a> = HashMap<&'a str, Vec<Box<dyn Fn(&Parts<'a>) -> Next<'a>>>>;

fn parse_next(rule: &str) -> Next {
    match rule {
        "A" => Next::Accept,
        "R" => Next::Reject,
        _ => Next::Rule(rule),
    }
}

fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let workflowre = regex::Regex::new(r"(\w+)\{(.*,?)+\}").unwrap();
    let rulere = regex::Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap();
    let mut workflows = Workflows::new();
    for line in &mut lines {
        let Some(caps) = workflowre.captures(line) else {
            break;
        };

        let (_, [name, rules]) = caps.extract();

        for rule in rules.split(',') {
            match rulere.captures(rule) {
                Some(cap) => {
                    let (_, [entry, op, n, next]) = cap.extract();
                    let n = n.parse().unwrap();
                    let next = parse_next(next);
                    workflows
                        .entry(name)
                        .or_default()
                        .push(Box::new(move |x| match op {
                            "<" => {
                                if x[entry] < n {
                                    next
                                } else {
                                    Next::Continue
                                }
                            }
                            ">" => {
                                if x[entry] > n {
                                    next
                                } else {
                                    Next::Continue
                                }
                            }
                            _ => unreachable!(),
                        }));
                }
                None => {
                    let next = parse_next(rule);
                    workflows
                        .entry(name)
                        .or_default()
                        .push(Box::new(move |_x| next));
                }
            }
        }
    }

    let partre = regex::Regex::new(r"(\w+)=(\d+)").unwrap();
    let mut parts = HashMap::new();
    let mut result = 0;
    for line in lines {
        parts.clear();
        for cap in partre.captures_iter(line) {
            let (_, [entry, n]) = cap.extract();
            parts.insert(entry, n.parse().unwrap());
        }
        if parts.is_empty() {
            break;
        }
        let mut next = Next::Rule("in");
        while let Next::Rule(r) = next {
            let workflow = &workflows[r];
            for w in workflow {
                next = w(&parts);
                if !matches!(next, Next::Continue) {
                    break;
                }
            }
        }
        if let Next::Accept = next {
            for i in parts.values() {
                result += *i;
            }
        }
    }
    result
}

fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 19114);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 167409079868000);
    }
}
