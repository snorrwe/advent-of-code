use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn hash(s: &str) -> usize {
    let mut res = 0;
    for b in s.bytes() {
        res += b as usize;
        res *= 17;
        res %= 256;
    }
    res
}

fn part1(input: &str) -> usize {
    let Some(line) = input.lines().next() else {
        return 0;
    };
    line.split(',').map(hash).sum()
}

fn part2(input: &str) -> usize {
    let Some(line) = input.lines().next() else {
        return 0;
    };
    let mut boxes = HashMap::<usize, Vec<_>>::new();
    line.split(',').for_each(|seq| {
        let (label, f) = seq.split_once(|c| c == '-' || c == '=').unwrap();
        let slot = hash(label);
        if seq.contains('-') {
            if let Some(boxx) = boxes.get_mut(&slot) {
                if let Some((i, _)) = boxx.iter().enumerate().find(|(_, (l, _))| *l == label) {
                    boxx.remove(i);
                }
            }
        } else {
            assert!(seq.contains('='));
            let f: usize = f.parse().unwrap();
            let boxx = boxes.entry(slot).or_default();

            if let Some((i, _)) = boxx.iter().enumerate().find(|(_, (l, _))| *l == label) {
                boxx[i].1 = f;
            } else {
                boxx.push((label, f));
            }
        }
    });
    boxes
        .into_iter()
        .map(|(box_i, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(slot_i, (_label, f))| (box_i + 1) * (slot_i + 1) * f)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 1320);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 145);
    }
}
