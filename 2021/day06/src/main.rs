type Fishies = Vec<i32>;

fn update(fishies: &mut Fishies)  {
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

    let mut fishies = Fishies::with_capacity(1024);

    if let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        assert!(size > 0);
        for item in strip_line_ends(&buffer).split(',') {
            fishies.push(item.parse().unwrap());
        }
    }

    // part1
    //
    for _ in 0..80 {
        update(&mut fishies);
    }

    println!("Part 1: {}", fishies.len());
}
