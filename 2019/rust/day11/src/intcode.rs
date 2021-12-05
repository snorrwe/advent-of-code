#[derive(Debug, Clone)]
pub struct IntCodeVM {
    program: Vec<i64>,
    ptr: usize,
    relative_base: usize,
}

impl IntCodeVM {
    pub fn new(mut program: Vec<i64>) -> Self {
        program.resize(program.len() * 2, 0);
        Self {
            program,
            ptr: 0,
            relative_base: 0,
        }
    }

    fn get_param_mut<'a>(
        instr: i64,
        ptr: usize,
        relative_base: usize,
        program: &'a mut [i64],
    ) -> &'a mut i64 {
        let mode = instr % 10;
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

    fn get_param(instr: i64, ptr: usize, relative_base: usize, program: &[i64]) -> i64 {
        let mode = instr % 10;
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

    /// Return wether the program halts
    pub fn execute(&mut self, input: i64, output: &mut Vec<i64>) -> bool {
        let ptr = &mut self.ptr;
        let relative_base = &mut self.relative_base;

        let program = self.program.as_mut_slice();
        'a: loop {
            let op = program[*ptr];
            let f = op % 100;
            match f {
                1 => {
                    // add
                    let x = Self::get_param(op / 100, *ptr + 1, *relative_base, program);
                    let y = Self::get_param(op / 1000, *ptr + 2, *relative_base, program);
                    let z = Self::get_param_mut(op / 10_000, *ptr + 3, *relative_base, program);
                    *z = x + y;
                    *ptr += 4;
                }
                2 => {
                    // mul
                    let x = Self::get_param(op / 100, *ptr + 1, *relative_base, program);
                    let y = Self::get_param(op / 1000, *ptr + 2, *relative_base, program);
                    let z = Self::get_param_mut(op / 10_000, *ptr + 3, *relative_base, program);
                    *z = x * y;
                    *ptr += 4;
                }
                3 => {
                    // input
                    let z = Self::get_param_mut(op / 100, *ptr + 1, *relative_base, program);
                    *z = input;
                    *ptr += 2;
                }
                4 => {
                    // output
                    let x = Self::get_param(op / 100, *ptr + 1, *relative_base, program);
                    output.push(x);
                    *ptr += 2;
                    return false;
                }
                5 => {
                    // jump if true
                    let x = Self::get_param(op / 100, *ptr + 1, *relative_base, program);
                    let z = Self::get_param(op / 1000, *ptr + 2, *relative_base, program);
                    if x != 0 {
                        *ptr = z as usize;
                    } else {
                        *ptr += 3;
                    }
                }
                6 => {
                    // jump if false
                    let x = Self::get_param(op / 100, *ptr + 1, *relative_base, program);
                    let z = Self::get_param(op / 1000, *ptr + 2, *relative_base, program);
                    if x == 0 {
                        *ptr = z as usize;
                    } else {
                        *ptr += 3;
                    }
                }
                7 => {
                    // lt
                    let x = Self::get_param(op / 100, *ptr + 1, *relative_base, program);
                    let y = Self::get_param(op / 1000, *ptr + 2, *relative_base, program);
                    let z = Self::get_param_mut(op / 10_000, *ptr + 3, *relative_base, program);
                    let res = x < y;
                    *z = res as i64;
                    *ptr += 4;
                }
                8 => {
                    // eq
                    let x = Self::get_param(op / 100, *ptr + 1, *relative_base, program);
                    let y = Self::get_param(op / 1000, *ptr + 2, *relative_base, program);
                    let z = Self::get_param_mut(op / 10_000, *ptr + 3, *relative_base, program);
                    let res = x == y;
                    *z = res as i64;
                    *ptr += 4;
                }
                9 => {
                    // adjust relative_base
                    let x = Self::get_param(op / 100, *ptr + 1, *relative_base, program);
                    *relative_base = (*relative_base as i64 + x) as usize;
                    *ptr += 2;
                }
                99 => {
                    return true;
                }
                _ => unreachable!("wtf ptr: {} op: {} instr: {}", ptr, op, f),
            };
        }
    }
}

pub fn parse_program(inp: &str) -> IntCodeVM {
    let program = inp
        .split(",")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    IntCodeVM::new(program)
}
