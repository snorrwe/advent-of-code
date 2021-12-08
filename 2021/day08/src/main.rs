use itertools::Itertools;
use std::collections::HashMap;

/// get unique candidates
fn len2candidates(len: usize) -> &'static [i32] {
    match len {
        2 => &[1],
        3 => &[7],
        4 => &[4],
        7 => &[8],
        5 => &[2, 3, 5],
        6 => &[0, 6, 9],
        _ => &[],
    }
}

fn sorted_str(s: &str) -> String {
    s.chars().sorted().collect()
}

/// assumes both are sorted
fn contains_letters(haystack: &str, needle: &str) -> bool {
    let mut count = 0;
    let len = needle.len();
    let mut needle = needle.chars();
    let mut n = needle.next().unwrap();

    for c in haystack.chars() {
        if c == n {
            count += 1;
            match needle.next() {
                Some(x) => n = x,
                None => break,
            }
        }
    }

    count == len
}

fn match_digits(
    digit2sorted: &HashMap<usize, Vec<String>>,
    needle: usize,
    haystack: &[&str],
) -> (usize, String) {
    let needle = digit2sorted[&needle][0].as_str();
    haystack
        .iter()
        .enumerate()
        .find_map(|(i, candidate)| {
            let s = sorted_str(candidate);
            contains_letters(s.as_str(), needle).then(move || (i, s))
        })
        .unwrap()
}

fn main() {
    let mut buffer = String::with_capacity(1024);
    let mut p1_count = 0;
    let mut p2_sum = 0;

    let mut decoded = String::new();

    while std::io::stdin().read_line(&mut buffer).is_ok() {
        let mut digit2segment = HashMap::<_, Vec<_>>::new();
        let mut digit2sorted = HashMap::<_, Vec<_>>::new();
        let mut segment2digit = HashMap::new();

        let line: &str = buffer.as_str().trim_end();
        if line.is_empty() {
            break;
        }

        let mut parts = line.split('|');

        let input = parts.next().unwrap();
        let output = parts.next().unwrap();

        for segment in input.split(' ') {
            for digit in len2candidates(segment.len()) {
                digit2segment
                    .entry(*digit as usize)
                    .or_default()
                    .push(segment);
                digit2sorted
                    .entry(*digit as usize)
                    .or_default()
                    .push(sorted_str(segment));
            }
        }

        // insert the unique sequences
        for (k, v) in digit2segment.iter().filter(|(_, v)| v.len() == 1) {
            segment2digit.insert(sorted_str(v[0]), *k);
        }
        // solve the non-unique pieces of shit
        let mut six = String::default();
        for (k, v) in digit2segment
            .iter_mut()
            .filter(|(k, _v)| **k == 2 || **k == 0)
        {
            match *k {
                2 | 3 | 5 => {
                    // only three contains 1 in it
                    let (i, three) = match_digits(&digit2sorted, 1, v);
                    v.swap_remove(i);

                    // we have 1,3,4,7,8 decoded
                    segment2digit.insert(three, 3);
                }
                0 | 6 | 9 => {
                    // only three contains 1 in it
                    let (i, nine) = match_digits(&digit2sorted, 4, v);
                    v.swap_remove(i);
                    let (i, zero) = match_digits(&digit2sorted, 1, v);
                    v.swap_remove(i);
                    six = sorted_str(v[0]);

                    segment2digit.insert(nine, 9);
                    segment2digit.insert(zero, 0);
                    segment2digit.insert(six.clone(), 6);
                }
                _ => unreachable!(),
            }
        }
        // need 2,5
        // use 6
        {
            let v = &digit2segment[&2];
            assert!(v.len() == 2); // assume we removed 3 last time

            let (i, five) = v
                .iter()
                .enumerate()
                .find_map(move |(i, x)| {
                    let x = sorted_str(x);
                    contains_letters(&six, x.as_str()).then(move || (i, x))
                })
                .unwrap();
            let two = v[1 - i];
            segment2digit.insert(five, 5);
            segment2digit.insert(sorted_str(two), 2);
        }

        assert!(segment2digit.len() == 10);

        let mut nums = [0; 4];
        let mut i = 0;
        for segment in output.split(' ') {
            if segment.is_empty() {
                continue;
            }
            if len2candidates(segment.len()).len() == 1 {
                p1_count += 1;
            }
            let digit = segment2digit[&sorted_str(segment)];
            nums[i] = digit;
            i += 1;

            decoded.clear();
        }
        let pp: usize = nums
            .into_iter()
            .enumerate()
            .map(|(i, x)| 10usize.pow(3 - i as u32) * x)
            .sum();

        p2_sum += pp;
        buffer.clear();
    }
    println!("p1 {} p2 {}", p1_count, p2_sum);
}
