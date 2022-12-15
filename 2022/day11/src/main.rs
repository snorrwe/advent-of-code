use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
enum Opt {
    MulConst(u64),
    AddConst(u64),
    Square,
    Noop,
}

impl Default for Opt {
    fn default() -> Self {
        Opt::Noop
    }
}

type MonkeId = usize;

#[derive(Default, Debug, Clone)]
struct Monke {
    opt: Opt,
    items: Vec<u64>,
    test: u64,
    iftrue: MonkeId,
    iffalse: MonkeId,
}

fn parse(input: &str) -> Vec<RefCell<Monke>> {
    let mut result = Vec::with_capacity(16);
    let opregex = regex::Regex::new(r"Operation: new = old (.) (\d+|(old))").unwrap();
    let testregex = regex::Regex::new(r"Test: divisible by (\d+)").unwrap();
    let ifregex = regex::Regex::new(r"If ((true)|(false)): throw to monkey (\d+)").unwrap();
    macro_rules! last {
        () => {
            (*result.last_mut().unwrap()).borrow_mut()
        };
    }

    for line in input.lines() {
        if line.starts_with("Monkey") {
            result.push(RefCell::new(Monke::default()));
            continue;
        }
        let line = line.trim();
        if line.starts_with("Starting items:") {
            let items = line["Starting items: ".len()..].split(", ");
            for item in items {
                if let Ok(i) = item.parse() {
                    last!().items.push(i);
                }
            }
        } else if let Some(cap) = opregex.captures(line) {
            if cap.get(3).is_some() {
                last!().opt = Opt::Square;
            } else {
                let constant = cap.get(2).unwrap().as_str().parse().unwrap();
                match cap.get(1).unwrap().as_str() {
                    "+" => {
                        last!().opt = Opt::AddConst(constant);
                    }
                    "*" => {
                        last!().opt = Opt::MulConst(constant);
                    }
                    _ => unreachable!(),
                }
            }
        } else if let Some(cap) = testregex.captures(line) {
            let div = cap.get(1).unwrap().as_str().parse().unwrap();
            last!().test = div;
        } else if let Some(cap) = ifregex.captures(line) {
            let id = cap.get(4).unwrap().as_str().parse().unwrap();
            match cap.get(1).unwrap().as_str() {
                "false" => {
                    last!().iffalse = id;
                }
                "true" => {
                    last!().iftrue = id;
                }
                _ => unreachable!(),
            }
        }
    }
    result
}

fn part1(mut monkeys: Vec<RefCell<Monke>>) -> usize {
    let mut inspects = monkeys.iter().map(|_| 0).collect::<Vec<_>>();
    for _turn in 0..20 {
        for (monkey_id, monkey) in monkeys.iter().enumerate() {
            let mut monkey = monkey.borrow_mut();
            let items = std::mem::take(&mut monkey.items);
            for item in items {
                inspects[monkey_id] += 1;
                let item = match monkey.opt {
                    Opt::MulConst(i) => item * i,
                    Opt::AddConst(i) => item + i,
                    Opt::Square => item * item,
                    Opt::Noop => unreachable!(),
                };
                let item = item / 3;
                if item % monkey.test == 0 {
                    monkeys[monkey.iftrue].borrow_mut().items.push(item);
                } else {
                    monkeys[monkey.iffalse].borrow_mut().items.push(item);
                }
            }
        }
    }
    inspects.sort_by(|a, b| b.cmp(a));
    inspects[0] * inspects[1]
}

fn part2(mut monkeys: Vec<RefCell<Monke>>) -> usize {
    let mut inspects = monkeys.iter().map(|_| 0).collect::<Vec<_>>();
    let m: u64 = monkeys.iter().map(|m|m.borrow().test).product();
    for _turn in 0..10000 {
        for (monkey_id, monkey) in monkeys.iter().enumerate() {
            let mut monkey = monkey.borrow_mut();
            let items = std::mem::take(&mut monkey.items);
            for item in items {
                inspects[monkey_id] += 1;
                let item = match monkey.opt {
                    Opt::MulConst(i) => item * i,
                    Opt::AddConst(i) => item + i,
                    Opt::Square => item * item,
                    Opt::Noop => unreachable!(),
                };
                let item = item % m;
                if item % monkey.test == 0 {
                    monkeys[monkey.iftrue].borrow_mut().items.push(item);
                } else {
                    monkeys[monkey.iffalse].borrow_mut().items.push(item);
                }
            }
        }
    }
    inspects.sort_by(|a, b| b.cmp(a));
    inspects[0] * inspects[1]
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let monkeys = parse(&input);

    let res = part1(monkeys.clone());
    println!("part1: {res}");
    let res = part2(monkeys);
    println!("part2: {res}");
}

#[test]
fn part1_test() {
    let monkeys = parse(
        r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#,
    );

    let res = part1(monkeys);
    assert_eq!(res, 10605);
}

#[test]
fn part2_test() {
    let monkeys = parse(
        r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#,
    );

    let res = part2(monkeys);
    assert_eq!(res, 2713310158);
}
