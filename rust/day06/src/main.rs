use std::io::Read;
type Sheet = [bool; 26];

fn set_sheet(sheet: &mut Sheet, index: u8, value: bool) {
    debug_assert!(b'a' <= index && index <= b'z');
    let ind = (index as u8 - b'a') as usize;
    debug_assert!(ind < 26);
    sheet[ind] = value;
}

fn count_yes(sheet: &Sheet) -> usize {
    sheet.iter().cloned().filter(|x| *x).count()
}

fn parse_sheets(lines: &str) -> Vec<Sheet> {
    let mut res = Vec::new();
    let mut current = [false; 26];
    let mut has_answer = false;
    for line in lines.lines() {
        if line.len() == 0 && has_answer {
            res.push(current);
            has_answer = false;
            current = [false; 26];
        }
        for chr in line.bytes() {
            if b'a' <= chr && chr <= b'z' {
                has_answer = true;
                set_sheet(&mut current, chr, true);
            }
        }
    }
    if has_answer {
        // don't forget the last one
        res.push(current);
    }
    res
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    let sheets = parse_sheets(input.as_str());

    let p1: usize = sheets.iter().map(|s| count_yes(s)).sum();

    println!("{}", p1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_simple() {
        let inp = r#"

abc

a
b
c

ab
ac

a
a
a
a

b
            "#;

        let sheets = parse_sheets(inp);

        let sum: usize = sheets.iter().map(|sheet| count_yes(sheet)).sum();

        assert_eq!(sum, 11);
    }
}
