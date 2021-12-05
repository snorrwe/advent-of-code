mod input;

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

fn execute(input: i32, mut program: Vec<i32>) -> Vec<i32> {
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
                program[x as usize] = input;
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

fn main() {
    let input = input::get_input();
    println!("{:?}", execute(1, input));

    let input = input::get_input();
    println!("{:?}", execute(5, input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_eq_false_2() {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let res = execute(123, input);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn simple_eq_true_2() {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let res = execute(8, input);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn simple_eq_false() {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let res = execute(42, input);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn simple_eq_true() {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let res = execute(8, input);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn test_jump() {
        let input = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let res = execute(21312312, input);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn test_jump2() {
        let input = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let res = execute(0, input);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn larger() {
        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let res = execute(8, input);
        assert_eq!(res, vec![1000]);
    }
}
