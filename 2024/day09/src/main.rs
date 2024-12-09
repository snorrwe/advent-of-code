/// id, start, size
type File = (usize, usize, usize);
/// start, size
type Empty = (usize, usize);

#[derive(Debug)]
struct Input {
    blocks: Vec<Option<u32>>,
    first_empty: usize,
    last_full: usize,
    files: Vec<File>,
    empty: Vec<Empty>,
}

fn parse(input: String) -> Input {
    let mut first_empty = std::u32::MAX;
    let mut last_full = 0;
    let mut cur = 0;
    let mut files = Vec::new();
    let mut empty = Vec::new();
    let mut blocks = Vec::new();
    for (id, n) in input
        .bytes()
        .enumerate()
        .filter(|(_id, n)| n.is_ascii_digit())
    {
        let size = n - b'0';
        let id = if id % 2 == 0 {
            if size != 0 {
                last_full = (cur + size as u32).saturating_sub(1);
                files.push((id / 2, cur as usize, size as usize));
            }
            Some((id / 2) as u32)
        } else {
            if size != 0 {
                first_empty = first_empty.min(cur);
                empty.push((cur as usize, size as usize));
            }
            None
        };
        cur += size as u32;
        blocks.extend(std::iter::repeat(id).take(size as usize));
    }
    Input {
        blocks,
        first_empty: first_empty as usize,
        last_full: last_full as usize,
        files,
        empty,
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!(
        "{}",
        part1(
            input.blocks,
            input.first_empty as usize,
            input.last_full as usize
        )
    );
    println!("{}", part2(input.files, input.empty));
}

fn part1(mut blocks: Vec<Option<u32>>, mut first_empty: usize, mut last_full: usize) -> usize {
    while first_empty < last_full {
        assert!(blocks[first_empty].is_none());
        assert!(blocks[last_full].is_some());
        blocks.swap(first_empty, last_full);
        first_empty += 1;
        while blocks[first_empty].is_some() {
            first_empty += 1
        }
        last_full -= 1;
        while blocks[last_full].is_none() {
            last_full -= 1
        }
    }
    let mut checksum = 0;

    let mut it = blocks.iter().enumerate();
    while let Some((i, Some(id))) = it.next() {
        checksum += i * *id as usize;
    }

    checksum
}

fn part2(mut files: Vec<File>, mut empty: Vec<Empty>) -> usize {
    for (_fid, fstart, fsize) in files.iter_mut().rev() {
        if let Some((i, x)) = empty
            .iter()
            .enumerate()
            .take_while(|(_, (start, _esize))| start < fstart)
            .find(|(_, (_start, esize))| *fsize <= *esize)
        {
            *fstart = x.0;
            empty[i].0 += *fsize;
            empty[i].1 -= *fsize;
            if empty[i].1 == 0 {
                empty.remove(i);
            }
        }
    }

    let mut checksum = 0;
    for (id, start, size) in files {
        for i in 0..size {
            checksum += id * (start + i);
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(inp.blocks, inp.first_empty, inp.last_full);

        assert_eq!(res, 1928);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(inp.files, inp.empty);

        assert_eq!(res, 2858);
    }
}
