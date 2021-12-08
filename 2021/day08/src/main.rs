type Segments = [bool; 7];

lazy_static::lazy_static! {
    static ref DIGIT_SEGMENTS: [Segments; 10] = [
        parse_segment("abcefg"),
        parse_segment("cf"),
        parse_segment("acdeg"),
        parse_segment("acdfg"),
        parse_segment("bcdf"),
        parse_segment("abdfg"),
        parse_segment("abdefg"),
        parse_segment("acf"),
        parse_segment("abcdefg"),
        parse_segment("abcdfg"),
    ];
}

fn parse_segment(seg: &str) -> Segments {
    let mut res = [false; 7];
    seg.chars().for_each(|c| {
        let i = c as u8 - 'a' as u8;
        res[i as usize] = true;
    });
    res
}

/// if the digit is uniquely id'd by the segments then return the digit
fn len2digit(len: usize) -> Option<i32> {
    match len {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

fn main() {
    let mut buffer = String::with_capacity(1024);
    let mut p1_count = 0;
    while std::io::stdin().read_line(&mut buffer).is_ok() {
        let line: &str = buffer.as_str().trim_end();
        if line.is_empty() {
            break;
        }

        let mut parts = line.split('|');

        let _input = parts.next().unwrap();
        let output = parts.next().unwrap();

        // part 1
        //

        for segment in output.split(' ') {
            if len2digit(segment.len()).is_some() {
                p1_count += 1;
            }
        }
        buffer.clear();
    }
    println!("p1 {}", p1_count);
}
