fn main() {
    let mut horizontal: Vec<i32> = Vec::with_capacity(512);
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    let mut buffer = String::with_capacity(1024);
    if let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        assert!(size > 2);
        let line: &str = &buffer;
        for item in line
            .strip_suffix("\r\n")
            .or_else(|| line.strip_suffix('\n'))
            .unwrap_or(line)
            .split(',')
        {
            let i: i32 = item.parse().unwrap();

            min = i.min(min);
            max = i.max(max);

            horizontal.push(i);
        }
    }

    let p1: i32 = horizontal
        .iter()
        .map(|pos| horizontal.iter().map(|x| (x - pos).abs()).sum())
        .min()
        .unwrap();

    println!("Part1: {}", p1);

    let p2: i32 = (min..=max)
        .map(|pos| {
            horizontal
                .iter()
                .map(|x| {
                    let n = (x - pos).abs();
                    (n * (n + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap();

    println!("Part2: {}", p2);
}
