use std::io::Read;
#[derive(Debug, Clone, Copy)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

impl Op {
    pub fn parse(s: &str) -> Option<Op> {
        match s {
            "acc" => Some(Op::Acc),
            "jmp" => Some(Op::Jmp),
            "nop" => Some(Op::Nop),
            _ => None,
        }
    }
}

type Instruction = (Op, i32);

fn parse(lines: &str) -> Vec<Instruction> {
    let mut res = Vec::new();
    for line in lines.lines() {
        let mut tokens = line.split(" ");
        if let Some(op) = tokens.next().and_then(|t| Op::parse(t)) {
            res.push((op, tokens.next().unwrap().parse().unwrap()));
        }
    }
    res
}

/// return Err(accumulated) if the program would loop
fn run(program: &[Instruction]) -> Result<i32, i32> {
    let mut res = 0;

    let mut ptr = 0i32;

    let mut visited = vec![false; program.len()];

    while 0 <= 0 && (ptr as usize) < program.len() {
        let i = ptr as usize;
        if visited[i] {
            return Err(res);
        }
        visited[i] = true;
        let (op, arg) = program[i];
        match op {
            Op::Acc => res += arg,
            Op::Jmp => ptr += arg - 1,
            Op::Nop => {}
        }
        ptr += 1;
    }
    assert!(0 <= ptr);
    Ok(res)
}

fn part2(program: Vec<Instruction>) -> i32 {
    for (i, (op, _)) in program.iter().enumerate() {
        let mut program = program.clone();
        match *op {
            Op::Nop => program[i].0 = Op::Jmp,
            Op::Jmp => program[i].0 = Op::Nop,
            Op::Acc => continue,
        }
        if let Ok(res) = run(&program) {
            return res
        }
    }
    panic!()
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    let program = parse(input.as_str());

    let res = run(&program).unwrap_err();
    println!("{}", res);
    let res = part2(program);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let inp = r#"
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
            "#;

        let program = parse(inp);
        assert_eq!(program.len(), 9);

        let res = run(&program).unwrap_err();

        assert_eq!(res, 5, "{:?}", program);
    }
}
