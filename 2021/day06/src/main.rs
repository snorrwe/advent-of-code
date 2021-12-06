type FishiesETA = Vec<u8>;
type ETA = [usize; 9];

fn update2(eta: &mut ETA) {
    let zero = eta[0];
    for i in 1..=8 {
        eta[i - 1] = eta[i];
    }
    eta[6] += zero;
    eta[8] = zero;
}

fn update1(fishies: &mut FishiesETA) {
    let mut spawn = 0;
    for eta in fishies.iter_mut() {
        if *eta == 0 {
            *eta = 6;
            spawn += 1;
        } else {
            *eta -= 1;
        }
    }

    for _ in 0..spawn {
        fishies.push(8);
    }
}

fn strip_line_ends(line: &str) -> &str {
    line.strip_suffix("\r\n")
        .or_else(|| line.strip_suffix('\n'))
        .unwrap_or(line)
}

fn main() {
    let mut buffer = String::new();

    let mut fishies = FishiesETA::with_capacity(1024);
    let mut eta = [0; 9];

    if let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        assert!(size > 0);
        for item in strip_line_ends(&buffer).split(',') {
            let i = item.parse().unwrap();
            fishies.push(i);
            eta[i as usize] += 1;
        }
    }

    // part1
    //
    for _ in 0..80 {
        update1(&mut fishies);
    }

    println!("Part 1: {}", fishies.len());

    // part2
    //
    for _ in 0..256 {
        update2(&mut eta);
    }

    let p2: usize = eta.iter().sum();
    println!("Part 2: {}", p2);
}
