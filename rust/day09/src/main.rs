#![feature(test)]
extern crate test;
use std::fs::read_to_string;

fn get_param_mut<'a>(
    intr: i64,
    ptr: usize,
    relative_base: usize,
    program: &'a mut [i64],
) -> &'a mut i64 {
    let mode = intr % 10;
    let ind = program[ptr];
    match mode {
        0 => &mut program[ind as usize],
        2 => {
            let ind = relative_base as i64 + ind;
            &mut program[ind as usize]
        }
        _ => unreachable!(),
    }
}

fn get_param(intr: i64, ptr: usize, relative_base: usize, program: &[i64]) -> i64 {
    let mode = intr % 10;
    let ind = program[ptr];
    match mode {
        0 => program[ind as usize],
        1 => ind,
        2 => {
            let ind = relative_base as i64 + ind;
            program[ind as usize]
        }
        _ => unreachable!(),
    }
}

fn execute(program: &mut [i64], input: i64) -> Vec<i64> {
    let mut ptr = 0;
    let mut relative_base = 0;

    let mut output = Vec::with_capacity(16);
    'a: loop {
        let op = program[ptr];
        let f = op % 100;
        match f {
            1 => {
                // add
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                let y = get_param(op / 1000, ptr + 2, relative_base, program);
                let z = get_param_mut(op / 10_000, ptr + 3, relative_base, program);
                *z = x + y;
                ptr += 4;
            }
            2 => {
                // mul
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                let y = get_param(op / 1000, ptr + 2, relative_base, program);
                let z = get_param_mut(op / 10_000, ptr + 3, relative_base, program);
                *z = x * y;
                ptr += 4;
            }
            3 => {
                // input
                let z = get_param_mut(op / 100, ptr + 1, relative_base, program);
                *z = input;
                ptr += 2;
            }
            4 => {
                // output
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                output.push(x);
                ptr += 2;
            }
            5 => {
                // jump if true
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                let z = get_param(op / 1000, ptr + 2, relative_base, program);
                if x != 0 {
                    ptr = z as usize;
                } else {
                    ptr += 3;
                }
            }
            6 => {
                // jump if false
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                let z = get_param(op / 1000, ptr + 2, relative_base, program);
                if x == 0 {
                    ptr = z as usize;
                } else {
                    ptr += 3;
                }
            }
            7 => {
                // lt
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                let y = get_param(op / 1000, ptr + 2, relative_base, program);
                let z = get_param_mut(op / 10_000, ptr + 3, relative_base, program);
                let res = x < y;
                *z = res as i64;
                ptr += 4;
            }
            8 => {
                // eq
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                let y = get_param(op / 1000, ptr + 2, relative_base, program);
                let z = get_param_mut(op / 10_000, ptr + 3, relative_base, program);
                let res = x == y;
                *z = res as i64;
                ptr += 4;
            }
            9 => {
                // adjust relative_base
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                relative_base = (relative_base as i64 + x) as usize;
                ptr += 2;
            }
            99 => {
                break 'a;
            }
            _ => unreachable!("wtf ptr: {} op: {} instr: {}", ptr, op, f),
        };
    }
    output
}

fn run(mut program: Vec<i64>, input: i64) -> Vec<i64> {
    program.resize(1 << 11, 0);
    execute(&mut program, input)
}

fn parse_program(inp: &str) -> Vec<i64> {
    let program = read_to_string(inp).unwrap();
    program
        .split(",")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn main() {
    let program = parse_program("input.txt");
    println!("Part1: {:?}", run(program, 1));
    let program = parse_program("input.txt");
    println!("Part2: {:?}", run(program, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let program = parse_program("input.txt");
        b.iter(|| {
            let program = parse_program("input.txt");
            let program = test::black_box(program);
            run(program, 1)
        });
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| {
            let program = parse_program("input.txt");
            let program = test::black_box(program);
            run(program, 2)
        });
    }

    #[bench]
    fn bench_input_parsing(b: &mut Bencher) {
        b.iter(|| parse_program("input.txt"));
    }
}
