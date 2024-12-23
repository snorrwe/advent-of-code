use std::collections::{HashMap, HashSet};

type Input<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse(input: &str) -> Input {
    let mut res: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in input
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| l.split_once('-'))
        .flat_map(|(a, b)| [(a, b), (b, a)])
    {
        res.entry(a).or_default().insert(b);
    }
    res
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut threes = HashSet::new();
    for (k, v) in input.iter() {
        debug_assert!(!v.contains(k));
        if !k.starts_with('t') {
            continue;
        }
        for (i, a) in v.iter().enumerate() {
            let va = &input[a];
            for (j, b) in v.iter().enumerate() {
                if i == j {
                    continue;
                }
                if va.contains(b) {
                    let mut set = [*k, *a, *b];
                    set.sort();
                    threes.insert(set);
                }
            }
        }
    }

    threes.len()
}

fn part2(input: &Input) -> String {
    let mut longest = Vec::<&str>::new();
    let mut candidate = Vec::new();
    for (k, v) in input.iter() {
        debug_assert!(!v.contains(k));
        candidate.clear();
        candidate.push(*k);

        'outer: for vert in v.iter() {
            for a in candidate.iter() {
                let va = &input[a];
                if !va.contains(vert) {
                    continue 'outer;
                }
            }
            candidate.push(*vert);
        }
        if candidate.len() > longest.len() {
            std::mem::swap(&mut candidate, &mut longest);
        }
    }

    longest.sort();
    longest.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 7);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, "co,de,ka,ta");
    }
}
