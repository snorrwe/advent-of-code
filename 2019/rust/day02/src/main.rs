macro_rules! program {
    () => {
        [
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 13, 19, 1, 10, 19, 23, 2, 9, 23,
            27, 1, 6, 27, 31, 1, 10, 31, 35, 1, 35, 10, 39, 1, 9, 39, 43, 1, 6, 43, 47, 1, 10, 47,
            51, 1, 6, 51, 55, 2, 13, 55, 59, 1, 6, 59, 63, 1, 10, 63, 67, 2, 67, 9, 71, 1, 71, 5,
            75, 1, 13, 75, 79, 2, 79, 13, 83, 1, 83, 9, 87, 2, 10, 87, 91, 2, 91, 6, 95, 2, 13, 95,
            99, 1, 10, 99, 103, 2, 9, 103, 107, 1, 107, 5, 111, 2, 9, 111, 115, 1, 5, 115, 119, 1,
            9, 119, 123, 2, 123, 6, 127, 1, 5, 127, 131, 1, 10, 131, 135, 1, 135, 6, 139, 1, 139,
            5, 143, 1, 143, 9, 147, 1, 5, 147, 151, 1, 151, 13, 155, 1, 5, 155, 159, 1, 2, 159,
            163, 1, 163, 6, 0, 99, 2, 0, 14, 0,
        ]
    };
}

fn part1() -> i32 {
    let mut program = program!();
    program[1] = 12;
    program[2] = 2;
    'a: for i in (0..program.len()).step_by(4) {
        let op = program[i];
        let op: &dyn Fn(i32, i32) -> i32 = match op {
            1 => &|x, y| x + y,
            2 => &|x, y| x * y,
            99 => return program[0],
            _ => unreachable!(),
        };
        let x = program[i + 1] as usize;
        let y = program[i + 2] as usize;
        let z = program[i + 3] as usize;

        program[z] = op(program[x], program[y]);
    }
    unreachable!()
}

fn execute(noun: i32, verb: i32) -> i32 {
    let program = program!();
    let mut memory = program!();
    memory[1] = noun;
    memory[2] = verb;
    'a: for i in (0..program.len()).step_by(4) {
        let op = program[i];
        let op: &dyn Fn(i32, i32) -> i32 = match op {
            1 => &|x, y| x + y,
            2 => &|x, y| x * y,
            99 => return memory[0],
            _ => unreachable!(),
        };
        let x = program[i + 1] as usize;
        let y = program[i + 2] as usize;
        let z = program[i + 3] as usize;

        memory[z] = op(memory[x], memory[y]);
    }
    unreachable!()
}

fn part2() -> i32 {
    for x in 0..100 {
        for y in 0..100 {
            let output = execute(x, y);
            if output == 19690720 {
                return 100 * x + y;
            }
        }
    }
    unreachable!()
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
