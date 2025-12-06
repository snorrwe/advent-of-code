struct Input<'a> {
    cols: usize,
    numbers: Vec<u64>,
    ops: Vec<u8>,
    input: &'a str,
}

fn parse(input: &'_ str) -> Input<'_> {
    let mut cols = 0;
    let mut numbers = Vec::new();
    let mut operations = Vec::new();

    for row in input.lines().map(|l| l.trim()) {
        let Some((first, _)) = row.split_once(' ') else {
            break;
        };

        if first.parse().map(|_: u64| ()).is_ok() {
            // row of numbers
            for (i, n) in row.split(' ').filter(|x| !x.is_empty()).enumerate() {
                if let Ok(x) = n.parse() {
                    numbers.push(x);
                    cols = i + 1;
                }
            }
        } else {
            // row of ops
            for o in row.split(' ').filter(|o| o.len() == 1) {
                operations.push(o.as_bytes()[0]);
            }
        }
    }

    Input {
        cols,
        numbers,
        ops: operations,
        input,
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> u64 {
    let mut accumulator = vec![0u64; input.cols];
    for (i, o) in input.ops.iter().copied().enumerate() {
        if o == b'*' {
            accumulator[i] = 1;
        }
    }

    for row in input.numbers.as_slice().chunks_exact(input.cols) {
        for (i, n) in row.iter().enumerate() {
            match input.ops[i] {
                b'*' => {
                    accumulator[i] *= *n;
                }
                b'+' => {
                    accumulator[i] += *n;
                }
                c @ _ => unreachable!("invalid op: {}", c as char),
            }
        }
    }

    accumulator.into_iter().sum()
}

fn part2(input: &Input) -> u64 {
    let mut mx: Vec<&[u8]> = input
        .input
        .lines()
        .map(|l| l.trim_end_matches('\n').as_bytes())
        .collect();

    // last row is the ops
    let ops = mx.pop().unwrap();
    let cols = ops.len();
    let mut col_idx = vec![0usize; cols];

    // fill col indices
    {
        let mut idx = 0;
        for i in 0..cols {
            col_idx[i] = idx;
            let cnt = mx.iter().filter(|row| row[i] != b' ').count();
            if cnt == 0 {
                col_idx[i] = !0;
                idx += 1;
            }
        }
    }

    let mut accumulator = vec![0u64; input.cols];
    for (i, o) in input.ops.iter().copied().enumerate() {
        if o == b'*' {
            accumulator[i] = 1;
        }
    }

    let mut buf = String::new();
    for i in 0..cols {
        if col_idx[i] == !0 {
            continue;
        }
        buf.clear();
        buf.extend(mx.iter().map(|l| l[i] as char));
        let num: u64 = buf.trim().parse().unwrap();
        let col_idx = col_idx[i];
        match input.ops[col_idx] {
            b'*' => {
                accumulator[col_idx] *= num;
            }
            b'+' => {
                accumulator[col_idx] += num;
            }
            _ => unreachable!(),
        }
    }

    accumulator.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 4277556);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 3263827);
    }
}
