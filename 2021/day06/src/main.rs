fn update(eta: &mut [u32; 9]) {
    let zero = eta[0];
    for i in 1..=8 {
        eta[i - 1] = eta[i];
    }
    eta[6] += zero;
    eta[8] = zero;
}

fn main() {
    let mut eta = [0; 9];

    let mut buffer = String::with_capacity(1024);
    if let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        assert!(size > 0);
        let line: &str = &buffer;
        for item in line
            .strip_suffix("\r\n")
            .or_else(|| line.strip_suffix('\n'))
            .unwrap_or(line)
            .split(',')
        {
            let i: usize = item.parse().unwrap();
            eta[i] += 1;
        }
    }

    let start = std::time::Instant::now();

    // part1
    //
    for _ in 0..80 {
        update(&mut eta);
    }

    let p1: usize = eta.iter().map(|x| *x as usize).sum();

    // part2
    //
    for _ in 80..256 {
        update(&mut eta);
    }
    let end = std::time::Instant::now();

    let dur = end - start;

    let p2: usize = eta.iter().map(|x| *x as usize).sum();
    println!("Part1: {}, Part 2: {} t: {:?}", p1, p2, dur);
}
