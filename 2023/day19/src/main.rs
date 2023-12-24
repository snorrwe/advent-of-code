fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let workflowre = regex::Regex::new(r"(\w+)\{(.*,?)+\}").unwrap();
    let rulere = regex::Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap();
    for line in &mut lines {
        dbg!(line);
        let Some(caps) = workflowre.captures(line) else {
            break;
        };

        let (_, [name, rules]) = caps.extract();

        dbg!(name);
        for rule in rules.split(',') {
            match rulere.captures(rule) {
                Some(cap) => {
                    dbg!(cap);
                }
                None => {
                    dbg!(rule);
                }
            }
        }
    }
    for line in lines {}
    todo!()
}

fn part2(input: &str) -> i32 {
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

        assert_eq!(res, 42);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 42);
    }
}
