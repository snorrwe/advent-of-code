use itertools::Itertools;

type Input<'a> = Vec<Line<'a>>;

#[derive(Debug)]
struct Line<'a> {
    pub pattern: &'a [u8],
    pub wiring: Vec<usize>,
    /// wiring offset and size
    pub wiring_group: Vec<(usize, usize)>,
    pub joltage: Vec<i64>,
}

fn parse(input: &'_ str) -> Input<'_> {
    let re = regex::Regex::new(r#"^\[(.*)\]\s?((\([0-9,]+\)\s?)+)\s?(\{[0-9,]+\}\s?)+$"#).unwrap();
    let number_groups = regex::Regex::new(r#"\((\d+,?)+\)"#).unwrap();

    input
        .lines()
        .filter_map(|l| {
            let captures = re.captures(l)?;

            let pattern = captures.get(1).unwrap().as_str();
            let l = captures.get(2).unwrap().as_str();

            let mut wiring = Vec::new();
            let mut wiring_group = Vec::new();

            for c in number_groups.captures_iter(l) {
                let offset = wiring.len();
                let l = c.get(0).unwrap().as_str();
                let l = l.trim_matches(&['(', ')', ' ']);

                let mut size = 0;
                for d in l.split(',') {
                    size += 1;
                    wiring.push(d.parse().unwrap());
                }
                wiring_group.push((offset, size));
            }

            let joltage = captures.get(4).unwrap().as_str();
            let joltage = joltage.trim_matches(&['{', '}', ' ']);
            let joltage = joltage.split(',').map(|d| d.parse().unwrap()).collect();

            Some(Line {
                pattern: pattern.as_bytes(),
                wiring,
                wiring_group,
                joltage,
            })
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn press(l: &Line, button_idx: usize, state: &mut [u8]) {
    let (offset, size) = l.wiring_group[button_idx];

    for wiring_idx in (0..size).map(|x| x + offset) {
        let i = l.wiring[wiring_idx];
        state[i] = 1 - state[i];
    }
}

/// return if equal
///
/// state 0,1
/// pattern: 35,46
fn compare_state(state: &[u8], pattern: &[u8]) -> bool {
    assert_eq!(state.len(), pattern.len());

    for (a, b) in state.iter().zip(pattern.iter()) {
        let a = (1 - a) * 11 + 35;
        if a != *b {
            return false;
        }
    }
    true
}

fn part1(input: &Input) -> usize {
    input
        .into_iter()
        .map(|l| {
            let mut min_presses = usize::MAX;
            let mut state = vec![0; l.pattern.len()];
            'outer: for c in (0..l.wiring_group.len()).powerset() {
                state.fill(0);
                if c.len() > min_presses {
                    continue;
                }
                for (i, b) in c.into_iter().enumerate() {
                    press(l, b, &mut state);
                    if compare_state(&state, l.pattern) {
                        min_presses = min_presses.min(i + 1);
                        continue 'outer;
                    }
                }
            }
            min_presses
        })
        .sum()
}

fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
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

        assert_eq!(res, 42);
    }

    #[test]
    fn test_compare_state() {
        let tests: &[(&[u8], &[u8], bool)] = &[
            (&[0, 1, 1, 0], b".##.", true),
            (&[1, 1, 1, 0], b".##.", false),
            (&[0, 0, 0, 1, 0], b"...#.", true),
            (&[0, 0, 1, 0, 1], b"...#.", false),
            (&[1, 1, 1, 0, 1], b"...#.", false),
        ];
        for (state, pattern, exp) in tests {
            let res = compare_state(state, pattern);
            assert_eq!(res, *exp);
        }
    }
}
