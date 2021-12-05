mod part1;
mod part2;

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let res = part1::part1(input.as_str());
    println!("{}", res);
    let res = part2::part2(input.as_str());
    println!("{}", res);
}
