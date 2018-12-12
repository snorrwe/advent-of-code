use super::{Rules, World};

pub fn load_initial_state<'a, I>(mut lines: I) -> (World, Rules)
where
    I: Iterator<Item = String>,
{
    let world = lines
        .next()
        .expect("Unexpected end of input")
        .split("initial state: ")
        .map(|x| {
            x.chars().enumerate().filter_map(|(i, c)| match c {
                '#' => Some(i as i32),
                '.' => None,
                _ => panic!("Unexpected character '{}' in map string"),
            })
        })
        .flatten()
        .collect::<World>();

    let rules = lines
        .filter(|line| line.len() > 5)
        .map(|line| {
            let mut split = line.split(" => ");
            let mut state = [false; 5];
            split
                .next()
                .expect("Unexpected end of Input while parsing rules")
                .chars()
                .enumerate()
                .for_each(|(i, c)| match c {
                    '#' => state[i] = true,
                    '.' => state[i] = false,
                    _ => panic!("Unexpected character '{}' in map string"),
                });
            let value = split
                .next()
                .expect("Unexpected end of Input while parsing rules")
                .chars()
                .find_map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    _ => None,
                })
                .expect("Failed to find the value of the expression");
            (state, value)
        })
        .collect::<Rules>();

    (world, rules)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = ["initial state: #..#.#", "", "...## => #", "..#.. => ."]
            .iter()
            .map(|s| s.to_string());

        let expected_world = [0, 3, 5].iter().map(|x| *x).collect::<World>();
        let expected_rules = [
            ([false, false, false, true, true], true),
            ([false, false, true, false, false], false),
        ]
        .iter()
        .map(|x| *x)
        .collect::<Rules>();

        let (world, rules) = load_initial_state(lines);

        assert_eq!(world, expected_world);
        assert_eq!(rules, expected_rules);
    }
}

