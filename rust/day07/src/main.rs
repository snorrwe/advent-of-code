mod input;

use std::collections::VecDeque;

fn get_param_mut<'a>(
    intr: i32,
    ptr: usize,
    program: &'a mut [i32],
    _memory: &'a mut [i32],
) -> &'a mut i32 {
    let mode = intr % 10;
    match mode {
        0 => &mut program[program[ptr] as usize],
        _ => unreachable!(),
    }
}

fn get_param(intr: i32, ptr: usize, program: &[i32], _memory: &[i32]) -> i32 {
    let mode = intr % 10;
    let ind = program[ptr];
    match mode {
        0 => program[ind as usize],
        1 => ind,
        _ => unreachable!(),
    }
}

fn execute(mut input: VecDeque<i32>, mut program: Vec<i32>) -> Vec<i32> {
    let mut memory = program.iter().cloned().collect::<Vec<_>>();
    let mut ptr = 0;
    let mut out = vec![];
    'a: while ptr < program.len() {
        let op = program[ptr];
        let f = op % 100;
        match f {
            1 => {
                let x = get_param(op / 100, ptr + 1, &program, &memory);
                let y = get_param(op / 1000, ptr + 2, &program, &memory);
                let z = get_param_mut(op / 10_000, ptr + 3, &mut program, &mut memory);
                *z = x + y;
                ptr += 4;
            }
            2 => {
                let x = get_param(op / 100, ptr + 1, &program, &memory);
                let y = get_param(op / 1000, ptr + 2, &program, &memory);
                let z = get_param_mut(op / 10_000, ptr + 3, &mut program, &mut memory);
                *z = x * y;
                ptr += 4;
            }
            3 => {
                // input
                let x = program[ptr + 1];
                program[x as usize] = input.pop_front().expect("input");
                ptr += 2;
            }
            4 => {
                // output
                let x = program[ptr + 1];
                out.push(program[x as usize]);
                ptr += 2;
            }
            5 => {
                // jump if true
                let x = get_param(op / 100, ptr + 1, &program, &memory);
                let z = get_param(op / 1000, ptr + 2, &program, &memory);
                if x != 0 {
                    ptr = z as usize;
                } else {
                    ptr += 3;
                }
            }
            6 => {
                // jump if false
                let x = get_param(op / 100, ptr + 1, &program, &memory);
                let z = get_param(op / 1000, ptr + 2, &program, &memory);
                if x == 0 {
                    ptr = z as usize;
                } else {
                    ptr += 3;
                }
            }
            7 => {
                // lt
                let x = get_param(op / 100, ptr + 1, &program, &memory);
                let y = get_param(op / 1000, ptr + 2, &program, &memory);
                let z = get_param_mut(op / 10_000, ptr + 3, &mut program, &mut memory);
                let res = x < y;
                *z = res as i32;
                ptr += 4;
            }
            8 => {
                // eq
                let x = get_param(op / 100, ptr + 1, &program, &memory);
                let y = get_param(op / 1000, ptr + 2, &program, &memory);
                let z = get_param_mut(op / 10_000, ptr + 3, &mut program, &mut memory);
                let res = x == y;
                *z = res as i32;
                ptr += 4;
            }
            99 => break 'a, // halt
            _ => unreachable!("wtf ptr: {} op: {} instr: {} out: {:?}", ptr, op, f, out),
        };
    }
    return out;
}

fn execute_sequence(program: &Vec<i32>, sequence: &[i32]) -> i32 {
    let mut out = 0;
    for s in sequence.iter() {
        let mut input = VecDeque::new();
        input.push_back(*s);
        input.push_back(out);
        out = *execute(input, program.clone()).get(0).expect("output");
    }
    out
}

fn part1(program: Vec<i32>) -> i32 {
    use itertools::Itertools;
    (0..=4)
        .permutations(5)
        .map(|perm| execute_sequence(&program, &perm[0..=4]))
        .max()
        .expect("wtf")
}

fn part2(program: Vec<i32>) -> i32 {
    use itertools::Itertools;
    (5..=9)
        .permutations(5)
        .map(|perm| execute_sequence(&program, &perm[0..=4]))
        .max()
        .expect("wtf")
}

fn main() {
    let input = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];

    let input = input::INPUT.to_vec();

    println!("{}", part1(input));
}
