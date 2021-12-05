use std::{collections::HashMap, io::Read, mem::take};

static PART1_FIELDS: &[&str] = &[
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*"cid"*/
];

type Passport<'a> = HashMap<&'a str, &'a str>;

fn parse(inp: &str) -> Vec<Passport> {
    let mut res = Vec::new();
    let mut current = HashMap::new();
    for line in inp.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                // line is empty meaning the next passport is coming up
                // take replaces current with a default map and returns the current value which we can
                // push into the vector
                res.push(take(&mut current));
            }
            continue;
        }

        for keyvalue in line.split(" ") {
            let mut entries = keyvalue.split(':');
            let key = entries.next().unwrap();
            let value = entries.next().unwrap();
            current.insert(key, value);
        }
    }
    // push the last one if it isn't empty
    if !current.is_empty() {
        res.push(current)
    }

    res
}

fn part1(required_fields: &[&str], passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|pp| {
            pp.keys()
                .filter(|key| required_fields.iter().find(|rf| rf == key).is_some())
                .count()
                == required_fields.len()
        })
        .count()
}

fn is_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => value
            .parse::<u32>()
            .map(|num| 1920 <= num && num <= 2002)
            .unwrap_or(false),
        "iyr" => value
            .parse::<u32>()
            .map(|num| 2010 <= num && num <= 2020)
            .unwrap_or(false),
        "eyr" => value
            .parse::<u32>()
            .map(|num| 2020 <= num && num <= 2030)
            .unwrap_or(false),
        "hgt" => {
            let limit = value.len() - 2;
            let num: u32 = match value[..limit].parse() {
                Ok(a) => a,
                _ => return false,
            };
            match &value[limit..] {
                "in" => 59 <= num && num <= 76,
                "cm" => 150 <= num && num <= 193,
                _ => false,
            }
        }
        "hcl" => {
            value.chars().next().map(|c| c == '#').unwrap_or(false)
                && u32::from_str_radix(&value[1..], 16).is_ok()
        }
        "ecl" => {
            value.len() == 3
                && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .iter()
                    .find(|c| c == &&value)
                    .is_some()
        }
        "pid" => value.len() == 9 && value.parse::<i64>().is_ok(),
        _ => true,
    }
}

fn part2(required_fields: &[&str], passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|pp| {
            pp.keys()
                .filter(|key| required_fields.iter().find(|rf| rf == key).is_some())
                .count()
                == required_fields.len()
                && pp.iter().all(|(key, value)| is_valid(key, value))
        })
        .count()
}

fn main() {
    let mut s = String::new();

    std::io::stdin().read_to_string(&mut s).unwrap();

    let kv = parse(s.as_str());
    let p1 = part1(PART1_FIELDS, kv.as_slice());
    println!("p1 {}", p1);
    let p2 = part2(PART1_FIELDS, kv.as_slice());
    println!("p1 {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_p1() {
        let inp = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
            "#;

        let kv = parse(inp);

        let res = part1(PART1_FIELDS, kv.as_slice());

        assert_eq!(res, 2, "{:#?}", kv);
    }
}
