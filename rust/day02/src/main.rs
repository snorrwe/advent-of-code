mod input;

use regex::Regex;

/// return [part1, part2]
fn solve(inp: &str) -> [usize; 2] {
    let re = Regex::new(r"(\d+)\-(\d+) ([a-z]): (.*)").unwrap();
    let mut res = [0, 0];
    for line in inp.split('\n') {
        if let Some(caps) = re.captures(line) {
            let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let letter: char = caps.get(3).unwrap().as_str().chars().next().unwrap();
            let pw = caps.get(4).unwrap().as_str();

            let count = pw.chars().filter(|c| *c == letter).count();
            if min <= count && count <= max {
                res[0] += 1;
            }

            let mut chars = pw.chars();

            // min and max are 1-indexed
            let has1 = chars.nth(min - 1).unwrap() == letter;
            let has2 = chars.nth(max - 1 - min).unwrap() == letter; // char has already stepped `min` times

            if has1 ^ has2 {
                // exactly one is true
                res[1] += 1;
            }
        }
    }
    res
}

fn main() {
    let res = solve(input::input());

    println!("{:?}", res);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple() {
        let inp = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
            "#;

        let res = solve(inp);

        assert_eq!(res[0], 2);
    }
}
