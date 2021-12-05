use std::{
    io::Read,
    ops::{Index, IndexMut},
};

#[derive(Debug, Default)]
struct Sheet {
    pub answers: [u32; 26],
    pub total: u32,
}

impl Index<u8> for Sheet {
    type Output = u32;

    fn index(&self, index: u8) -> &Self::Output {
        debug_assert!(b'a' <= index && index <= b'z');
        let ind = (index as u8 - b'a') as usize;
        debug_assert!(ind < 26);
        &self.answers[ind]
    }
}

impl IndexMut<u8> for Sheet {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        debug_assert!(b'a' <= index && index <= b'z');
        let ind = (index as u8 - b'a') as usize;
        debug_assert!(ind < 26);
        &mut self.answers[ind]
    }
}

fn count_any(sheet: &Sheet) -> usize {
    sheet.answers.iter().cloned().filter(|x| *x > 0).count()
}

fn count_all(sheet: &Sheet) -> usize {
    let total = sheet.total;
    sheet
        .answers
        .iter()
        .cloned()
        .filter(|x| *x == total)
        .count()
}

fn parse_sheets(lines: &str) -> Vec<Sheet> {
    let mut res = Vec::new();
    let mut current = Sheet::default();
    let mut total = 0;
    for line in lines.lines() {
        if line.trim().len() == 0 && total > 0 {
            current.total = total;
            res.push(current);
            total = 0;
            current = Sheet::default();
        } else {
            total += (line.len() > 0) as u32;
            for chr in line.bytes() {
                if b'a' <= chr && chr <= b'z' {
                    current[chr] += 1;
                }
            }
        }
    }
    if total > 0 {
        // don't forget the last one
        current.total = total;
        res.push(current);
    }
    res
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    let sheets = parse_sheets(input.as_str());

    let p1: usize = sheets.iter().map(|s| count_any(s)).sum();

    println!("{}", p1);

    let p2: usize = sheets.iter().map(|sheet| count_all(sheet)).sum();
    println!("{}", p2)
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

        let sum: usize = sheets.iter().map(|sheet| count_any(sheet)).sum();

        assert_eq!(sum, 11);
    }

    #[test]
    fn part2_simple() {
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

        let sum: usize = sheets.iter().map(|sheet| count_all(sheet)).sum();

        assert_eq!(sum, 6);
    }

    #[test]
    fn count_all_1() {
        let sheet = Sheet {
            total: 1,
            answers: [
                1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };

        let count = count_all(&sheet);

        assert_eq!(count, 3);
    }
}
