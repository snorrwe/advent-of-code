fn p1(inp: &str) -> u32 {
    let mut lines = inp.lines();
    let time: u32 = loop {
        if let Some(t) = lines.next().unwrap().trim().parse().ok() {
            break t;
        }
    };

    let ids = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|x| x.trim().parse().ok());

    let (id, next_depart) = ids
        .map(|id: u32| (id, (time / id + 1) * id))
        .min_by_key(|(_id, time)| *time)
        .unwrap();

    let wait = next_depart - time;

    wait * id
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> i64 {
    let (g, x, _) = egcd(x, n);
    assert_eq!(g, 1);
    (x % n + n) % n
}

fn p2(inp: &str) -> i64 {
    let mut lines = inp.lines();
    // just for parsing
    let _time: u32 = loop {
        if let Some(t) = lines.next().unwrap().trim().parse().ok() {
            break t;
        }
    };

    // None means no restriction on the departure
    let ids = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().ok());

    // ( index / remainder , transformed id / modulus )
    let indices: Vec<(i64, i64)> = ids
        .enumerate()
        .filter_map(|(i, id)| {
            id.map(|id: i64| {
                let i = i as i64;
                let i = ((-i % id) + id) % id; // make sure i is positive
                // T = i mod id
                (i, id)
            })
        })
        .collect();

    // chinese remainder theorem
    let prod: i64 = indices.iter().map(|(_, n)| *n).product();

    let mut sum = 0;
    for (residue, modulus) in indices {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus) * p;
    }
    sum % prod
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let res = p1(input.as_str());
    println!("{}", res);
    let res = p2(input.as_str());
    println!("{}", res);
}

#[test]
fn part1() {
    let inp = r#"
939
7,13,x,x,59,x,31,19
"#;

    let res = p1(inp);

    assert_eq!(res, 295);
}

#[test]
fn part2() {
    let inp = r#"
939
7,13,x,x,59,x,31,19
"#;

    let res = p2(inp);

    assert_eq!(res, 1_068_781);
}
