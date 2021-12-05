use std::{collections::HashMap, mem::swap};

fn parse_point(inp: &str) -> [i32; 2] {
    let mut inp = inp.split(',');
    let x = inp.next().expect("x next").parse().expect("x parse");
    let y = inp.next().expect("y next").parse().expect("y parse");
    [x, y]
}

/// strip the line ending
fn strip_line_ends(line: &str) -> &str {
    line.strip_suffix("\r\n")
        .or_else(|| line.strip_suffix('\n'))
        .unwrap_or(line)
}

fn main() {
    let mut lines = Vec::with_capacity(500);
    let mut buffer = String::new();
    while let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        if size == 0 {
            break;
        }
        let line = strip_line_ends(buffer.as_str());

        let mut points = line.split(" -> ");
        let a = parse_point(points.next().unwrap());
        let b = parse_point(points.next().unwrap());

        lines.push([a, b]);

        buffer.clear();
    }

    let mut diagram = HashMap::<_, isize>::with_capacity(500 * 500);
    // part 1
    let mut count_p1 = 0;
    for [[mut x1, mut y1], [mut x2, mut y2]] in
        lines.iter().filter(|[a, b]| a[0] == b[0] || a[1] == b[1])
    {
        if x1 > x2 {
            swap(&mut x1, &mut x2);
        }
        if y1 > y2 {
            swap(&mut y1, &mut y2);
        }
        assert!(x1 - x2 == 0 || y1 - y2 == 0);
        for y in y1..=y2 {
            for x in x1..=x2 {
                let entry = diagram.entry([x, y]).or_default();
                *entry += 1;

                if *entry == 2 {
                    count_p1 += 1;
                }
            }
        }
    }

    // part2
    let mut count_p2 = count_p1;
    for [[x1, y1], [x2, y2]] in lines.iter().filter(|[a, b]| a[0] != b[0] && a[1] != b[1]) {
        // diagonals
        //
        assert!((x2 - x1).abs() == (y2 - y1).abs());
        let delta = (x2 - x1).abs();
        let dx = (x2 - x1) / delta;
        let dy = (y2 - y1) / delta;
        for d in 0..=delta {
            let [x, y] = [x1 + d * dx, y1 + d * dy];

            let entry = diagram.entry([x, y]).or_default();
            *entry += 1;

            if *entry == 2 {
                count_p2 += 1;
            }
        }
    }

    println!("part1: {} part2: {}", count_p1, count_p2);
}
