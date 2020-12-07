use std::{collections::HashMap, io::Read};

type Color<'a> = (&'a str, &'a str);

#[derive(Debug)]
struct Hold<'a> {
    color: Color<'a>,
    amount: u32,
}

fn parse_color(inp: &str) -> Option<Color> {
    let mut words = inp.split(" ");
    let color1 = words.next()?;
    let color2 = words.next()?;

    if color1.is_empty() || color2.is_empty() {
        return None;
    }

    Some((color1, color2))
}

fn parse_rules(inp: &str) -> HashMap<Color, Vec<Hold>> {
    let mut res = HashMap::new();
    'colors: for line in inp.lines() {
        if let Some(color) = parse_color(line) {
            let entry = res.entry(color).or_insert_with(Vec::new);

            let cont = match line.split("contain").nth(1) {
                Some(a) => a,
                _ => continue 'colors,
            };
            for rule in cont.split(",") {
                let mut words = rule.split(" ");
                words.next();
                let amount = words.next().unwrap();
                let amount = match amount.parse() {
                    Ok(amount) => amount,
                    _ if amount == "no" => 0,
                    _ => unreachable!(),
                };
                let color1 = words.next().unwrap();
                let color2 = words.next().unwrap();

                entry.push(Hold {
                    color: (color1, color2),
                    amount,
                })
            }
        }
    }

    res
}

fn part1(rules: &HashMap<Color, Vec<Hold>>) -> usize {
    rules.keys().filter(|color| _part1(color, rules)).count()
}

fn _part1(current: &Color, rules: &HashMap<Color, Vec<Hold>>) -> bool {
    rules
        .get(current)
        .and_then(|r| {
            r.iter()
                .find(|hold| hold.color == ("shiny", "gold") || _part1(&hold.color, rules))
        })
        .is_some()
}

fn part2(rules: &HashMap<Color, Vec<Hold>>) -> usize {
    _part2(&("shiny", "gold"), rules)
}

fn _part2(color: &Color, rules: &HashMap<Color, Vec<Hold>>) -> usize {
    rules
        .get(color)
        .map(|r| {
            r.iter()
                .map(|Hold { amount, color }| {
                    let amount = *amount as usize;
                    amount + amount * _part2(color, rules)
                })
                .sum()
        })
        .unwrap_or(0)
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    let rules = parse_rules(input.as_str());

    let p1 = part1(&rules);
    let p2 = part2(&rules);
    println!("p1 {} p2 {}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_simple() {
        let input = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
            "#;

        let rules = parse_rules(input);

        let p1 = part1(&rules);

        assert_eq!(p1, 4)
    }

    #[test]
    fn test_p2_simple() {
        let input = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
            "#;

        let rules = parse_rules(input);

        let p2 = part2(&rules);

        assert_eq!(p2, 32)
    }
}
