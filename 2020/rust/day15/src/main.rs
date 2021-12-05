use std::collections::HashMap;

fn speak(memory: &mut HashMap<i32, Vec<i32>>, turn: i32, last: &mut i32) {
    if let Some(v) = memory.get(last) {
        if v.len() >= 2 {
            *last = v.last().unwrap() - v[v.len() - 2];
        } else if v.len() == 1 {
            *last = 0;
        }
    }
    memory.entry(*last).or_insert_with(Vec::new).push(turn);
}

fn run(inp: &str, ticks: i32) -> i32 {
    let mut memory = HashMap::new();
    let mut turn = 0;
    for num in inp.split(',').filter_map(|n| n.parse().ok()) {
        turn += 1;
        memory.insert(num, vec![turn]);
    }
    // in the 4th turn we'll speak 0
    turn += 1;
    let mut last = 0;
    memory.entry(0i32).or_insert_with(Vec::new).push(turn);
    while turn < ticks {
        turn += 1; // turns are indexed from 1
        speak(&mut memory, turn, &mut last);
    }
    last
}

fn main() {
    let res = run("0,6,1,7,2,19,20", 2020);
    println!("{}", res);
    let res = run("0,6,1,7,2,19,20", 30000000);
    println!("{}", res);
}

#[test]
fn test_p1() {
    let res = run("0,3,6", 2020);
    assert_eq!(res, 436);
}


#[test]
fn test_p2() {
    let res = run("0,3,6", 30000000);
    assert_eq!(res, 175594);
}
