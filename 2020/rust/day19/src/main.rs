use std::collections::HashMap;
use tracing::{event, Level};

/// Return the end position of the match
pub type Filter = Box<dyn Fn(&Rules, &str) -> Option<usize>>;
pub struct Rules(pub HashMap<i32, Filter>);

/// number of lines parsed + rules
fn parse_rules(inp: &str) -> (usize, Rules) {
    let mut rules = Rules(HashMap::new());

    let mut count = 0;
    for line in inp.lines() {
        count += 1;
        if line.is_empty() {
            break;
        }

        let mut kv = line.split(":");
        let key: i32 = kv.next().unwrap().parse().expect("not a number");

        let val = kv.next().unwrap();
        let or_filters = val
            .split("|")
            .map(|f| {
                let and_filters = f
                    .split(" ")
                    .filter_map(|token| {
                        // one,single rule
                        if let Some(num) = token.parse::<i32>().ok() {
                            // rule is a number, indicating that we want to call another rule
                            let filter = Box::new(move |rules: &Rules, line: &str| {
                                event!(Level::DEBUG, "calling subrule {}", num);
                                let res = rules.0[&num](rules, line);
                                event!(Level::DEBUG, "calling subrule returns {:?}", res);
                                res
                            });
                            Some(filter as Filter)
                        } else {
                            if let Some(ind) = token.find('"') {
                                let substr = &token[ind + 1..];
                                let substr = substr.chars().next().unwrap();
                                let filter = move |_: &Rules, line: &str| {
                                    event!(Level::DEBUG, "matching substr {:?}", substr);
                                    if line.starts_with(substr) {
                                        event!(Level::DEBUG, "match!");
                                        Some(1usize)
                                    } else {
                                        event!(Level::DEBUG, "no match!");
                                        None
                                    }
                                };
                                let filter = Box::new(filter);
                                Some(filter as Filter)
                            } else {
                                None
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                move |rules: &Rules, line: &str| {
                    let mut i = 0;
                    for (rule_id, rule) in and_filters.iter().enumerate() {
                        let line = &line[i..];
                        let span = tracing::span!(
                            Level::DEBUG,
                            "filter 'AND' rule",
                            "rule id" = rule_id,
                            "rules" = and_filters.len(),
                            "line" = line
                        );
                        let _ent = span.enter();
                        event!(Level::DEBUG, "calling rule!");
                        if let Some(j) = rule(rules, line) {
                            event!(Level::DEBUG, "rule matches up to {}!", j);
                            i += j
                        } else {
                            event!(Level::DEBUG, "rule doesn't match!");
                            return None;
                        }
                    }
                    event!(Level::DEBUG, "return {}", i);
                    Some(i)
                }
            })
            .collect::<Vec<_>>();

        let filter: Filter =
            Box::new(move |rules, line| or_filters.iter().find_map(|f| f(rules, line)));

        rules.0.insert(key, filter);
    }

    (count, rules)
}

fn part1<'a>(rules: &Rules, lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .filter(|line| {
            let span = tracing::span!(Level::DEBUG, "", "line" = line);
            let _ent = span.enter();
            rules.0[&0](rules, line)
                .and_then(|i| if i == line.len() { Some(i) } else { None })
                .map(|_| {
                    event!(Level::DEBUG, "matches in p1!");
                })
                .is_some()
        })
        .count()
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let (i, rules) = parse_rules(input.as_str());

    let res = part1(&rules, input.lines().skip(i));
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_env_log::test]
    fn test_p1() {
        let inp = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;

        let (i, rules) = parse_rules(inp);

        let res = part1(&rules, inp.lines().skip(i));

        assert_eq!(res, 2);
    }
}
