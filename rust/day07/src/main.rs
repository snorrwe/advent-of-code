mod input;

#[derive(Debug, Clone)]
struct Amp {
    halt: bool,
    id: Option<i32>,
    out: Option<i32>,
    program: Vec<i32>,
    ptr: usize,
}

fn get_param_mut<'a>(intr: i32, ptr: usize, program: &'a mut [i32]) -> &'a mut i32 {
    let mode = intr % 10;
    match mode {
        0 => &mut program[program[ptr] as usize],
        _ => unreachable!(),
    }
}

fn get_param(intr: i32, ptr: usize, program: &[i32]) -> i32 {
    let mode = intr % 10;
    let ind = program[ptr];
    match mode {
        0 => program[ind as usize],
        1 => ind,
        _ => unreachable!(),
    }
}

fn execute(mut amp: Amp, input: i32) -> Amp {
    let program = &mut amp.program;
    let mut ptr = amp.ptr;
    'a: loop {
        let op = program[ptr];
        let f = op % 100;
        match f {
            1 => {
                let x = get_param(op / 100, ptr + 1, program);
                let y = get_param(op / 1000, ptr + 2, program);
                let z = get_param_mut(op / 10_000, ptr + 3, program);
                *z = x + y;
                ptr += 4;
            }
            2 => {
                let x = get_param(op / 100, ptr + 1, program);
                let y = get_param(op / 1000, ptr + 2, program);
                let z = get_param_mut(op / 10_000, ptr + 3, program);
                *z = x * y;
                ptr += 4;
            }
            3 => {
                // input
                let x = program[ptr + 1];
                let inp = amp.id.or(Some(input)).expect("input");
                amp.id = None;
                program[x as usize] = inp;
                ptr += 2;
            }
            4 => {
                // output
                let x = program[ptr + 1];
                amp.out = Some(program[x as usize]);
                ptr += 2;
                break 'a;
            }
            5 => {
                // jump if true
                let x = get_param(op / 100, ptr + 1, program);
                let z = get_param(op / 1000, ptr + 2, program);
                if x != 0 {
                    ptr = z as usize;
                } else {
                    ptr += 3;
                }
            }
            6 => {
                // jump if false
                let x = get_param(op / 100, ptr + 1, program);
                let z = get_param(op / 1000, ptr + 2, program);
                if x == 0 {
                    ptr = z as usize;
                } else {
                    ptr += 3;
                }
            }
            7 => {
                // lt
                let x = get_param(op / 100, ptr + 1, program);
                let y = get_param(op / 1000, ptr + 2, program);
                let z = get_param_mut(op / 10_000, ptr + 3, program);
                let res = x < y;
                *z = res as i32;
                ptr += 4;
            }
            8 => {
                // eq
                let x = get_param(op / 100, ptr + 1, program);
                let y = get_param(op / 1000, ptr + 2, program);
                let z = get_param_mut(op / 10_000, ptr + 3, program);
                let res = x == y;
                *z = res as i32;
                ptr += 4;
            }
            99 => {
                amp.halt = true;
                break 'a;
            }
            _ => unreachable!("wtf ptr: {} op: {} instr: {} out: {:?}", ptr, op, f, amp),
        };
    }
    amp.ptr = ptr;
    return amp;
}

fn execute_sequence(program: &Vec<i32>, sequence: &[i32]) -> i32 {
    let mut out = 0;
    for s in sequence.iter().cloned() {
        let amp = Amp {
            halt: false,
            id: Some(s),
            out: None,
            program: program.clone(),
            ptr: 0,
        };
        let amp = execute(amp, out);
        out = amp.out.expect("output");
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
        .map(|perm| {
            let mut amps: Vec<_> = perm
                .iter()
                .map(|id| Amp {
                    id: Some(*id),
                    out: None,
                    halt: false,
                    program: program.clone(),
                    ptr: 0,
                })
                .collect();
            let mut out = 0i32;
            let mut i = 0;
            while !amps.last().unwrap().halt {
                let amp = amps[i].clone();
                let mut a = execute(amp, out);
                out = a.out.expect("amp output");
                a.id = None;
                amps[i] = a;
                i = (i + 1) % 5;
            }
            out
        })
        .max()
        .expect("wtf")
}

fn main() {
    let input = input::INPUT.to_vec();
    println!("{}", part1(input));
    let input = input::INPUT.to_vec();
    // let input = vec![
    //     3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
    //     1005, 28, 6, 99, 0, 0, 5,
    // ];
    println!("{}", part2(input));
}
