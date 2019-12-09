#![feature(test)]
extern crate test;

mod input;

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

    let mut output = vec![];
    'a: loop {
        let op = program[ptr];
        let f = op % 100;
        match f {
            1 => {
                let x = get_param(op / 100, ptr + 1, relative_base, program);
                let y = get_param(op / 1000, ptr + 2, relative_base, program);
                let z = get_param_mut(op / 10_000, ptr + 3, relative_base, program);
                *z = x + y;
                ptr += 4;
            }
            2 => {
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

fn main() {
    let mut program = input::program();
    program.resize(1 << 11, 0);
    println!("Part1: {:?}", execute(&mut program, 1));
    let mut program = input::program();
    program.resize(1 << 11, 0);
    println!("Part2: {:?}", execute(&mut program, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_both(b: &mut Bencher) {
        b.iter(|| main());
    }
}
