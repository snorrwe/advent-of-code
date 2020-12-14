use std::collections::HashMap;

fn part1(inp: &str) -> u64 {
    let mut mask = [None; 36];
    let mut memory = vec![0; 16000];

    for line in inp.lines() {
        if line.starts_with("mask") {
            for (i, chr) in line[7..].chars().enumerate() {
                mask[i] = chr.to_digit(2);
            }
        } else if line.starts_with("mem") {
            let index: usize = line[4..].split(']').next().unwrap().parse().expect("parse");
            let mut val = line.split(" = ");
            val.next();

            let mut val: u64 = val.next().unwrap().parse().expect("parse");
            for (i, m) in mask
                .iter()
                .rev()
                .enumerate()
                .filter_map(|(i, m)| m.map(|m| (i, m)))
            {
                val &= !(1 << i); // zero out this bit
                val |= (m as u64) << i; // set it to m
            }
            set_memory(&mut memory, index, val)
        }
    }
    memory.iter().sum()
}

fn set_memory(mem: &mut Vec<u64>, index: usize, value: u64) {
    if mem.len() <= index {
        mem.resize(index + 1, 0);
    }
    mem[index] = value;
}

fn part2(inp: &str) -> u64 {
    let mut mask = [None; 36];
    let mut memory = HashMap::new();

    for line in inp.lines() {
        if line.starts_with("mask") {
            for (i, chr) in line[7..].chars().enumerate() {
                mask[i] = chr.to_digit(2);
            }
        } else if line.starts_with("mem") {
            let mut index: usize = line[4..].split(']').next().unwrap().parse().expect("parse");
            let mut val = line.split(" = ");
            val.next();

            let val: u64 = val.next().unwrap().parse().expect("parse");

            // set the masked bits
            for (i, m) in mask
                .iter()
                .enumerate()
                .filter_map(|(i, m)| m.map(|m| (i, m)))
            {
                let offset = 35 - i;
                index |= (m as usize) << offset;
            }
            let index_orig = index;
            let mut indices = vec![index_orig];
            for (i, _) in mask.iter().rev().enumerate().filter(|(_, m)| m.is_none()) {
                // for each 'none' mask set both memory values
                //
                let news = indices
                    .iter()
                    .map(|index| {
                        index ^ (1 << i) // flip the bit at i
                    })
                    .collect::<Vec<_>>();

                indices.extend_from_slice(&news[..]);
            }

            for index in indices {
                memory.insert(index, val);
            }
        }
    }
    memory.values().sum()
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let res = part1(input.as_str());
    println!("{}", res);
    let res = part2(input.as_str());
    println!("{}", res);
}

#[test]
fn test_part1() {
    let inp = r#"
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
    "#;

    let res = part1(inp);

    assert_eq!(res, 165);
}

#[test]
fn test_part2() {
    let inp = r#"
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
    "#;

    let res = part2(inp);

    assert_eq!(res, 208);
}
