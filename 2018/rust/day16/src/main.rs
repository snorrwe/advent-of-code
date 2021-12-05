use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

type Registers = [i32; 4];
type Instructions<'a> = HashSet<&'a str>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Input {
    Before(Registers),
    After(Registers),
    Instr(Registers),
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().filter_map(|l| l.ok());

    let (part1, opcodes, test_program) = parse_input(lines);

    println!("Part1 {}", part1);

    let instructions = map_instructions(opcodes);

    let part2 = part2(instructions, test_program);

    println!("Part2 {}", part2[0]);

    Ok(())
}

fn part2(instructions: HashMap<i32, &str>, test_program: Vec<Registers>) -> Registers {
    test_program.iter().fold([0; 4], |result, input| {
        execute(instructions[&input[0]], input, result)
    })
}

fn execute(instruction: &str, input: &Registers, mut registers: Registers) -> Registers {
    let a = input[1] as usize;
    let c = input[3] as usize;
    let b = match instruction.chars().last().unwrap() {
        'r' => {
            let i = input[2] as usize;
            registers[i]
        }
        _ => input[2],
    };
    match &instruction[..3] {
        "add" => {
            registers[c] = registers[a] + b;
        }
        "mul" => {
            registers[c] = registers[a] * b;
        }
        "ban" => {
            registers[c] = registers[a] & b;
        }
        "bor" => {
            registers[c] = registers[a] | b;
        }
        "set" => {
            let a = if instruction.chars().last().unwrap() == 'i' {
                input[1]
            } else {
                registers[a]
            };
            registers[c] = a;
        }
        "gti" => registers[c] = if input[1] > b { 1 } else { 0 },
        "gtr" => registers[c] = if registers[a] > b { 1 } else { 0 },
        "eqi" => registers[c] = if input[1] == b { 1 } else { 0 },
        "eqr" => registers[c] = if registers[a] == b { 1 } else { 0 },

        _ => unimplemented!(),
    }
    registers
}

const INSTRUCTIONS: [&'static str; 16] = [
    "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri",
    "gtrr", "eqir", "eqri", "eqrr",
];

/// Returns the number of examples containing 3 or more opcodes (part 1)
/// And a map of opcode -> {instruction}
/// And the test program
fn parse_input<'a, I>(input: I) -> (usize, HashMap<i32, Instructions<'a>>, Vec<Registers>)
where
    I: Iterator<Item = String>,
{
    let input = input
        .filter_map(|line| parse_line(line.as_str()))
        .collect::<Vec<_>>();
    let mut before = None;
    let mut instr = None;
    let mut instr_map = HashMap::new();
    let mut test_program = Vec::with_capacity(1000);
    let result = input
        .iter()
        .filter(|input| {
            match input {
                Input::Before(reg) => {
                    before = Some(reg);
                }
                Input::Instr(reg) => {
                    if before.is_none() {
                        test_program.push(*reg);
                    } else {
                        instr = Some(reg);
                    }
                }
                Input::After(reg) => {
                    let n = {
                        let before = before.expect("After called without matching 'Before' line");
                        let instr = instr.expect("After called without matching 'Instr' line");
                        let opcodes = opcodes(before, instr, reg);
                        let result = opcodes.len();
                        let existing = instr_map.entry(instr[0]).or_insert_with(|| opcodes.clone());
                        *existing = existing.intersection(&opcodes).map(|x| *x).collect();
                        result
                    };
                    before = None;
                    instr = None;
                    return n >= 3;
                }
            }
            false
        })
        .count();
    (result, instr_map, test_program)
}

fn map_instructions(mut map: HashMap<i32, Instructions>) -> HashMap<i32, &str> {
    let mut result = HashMap::new();
    while result.len() != 16 {
        let (opcode, instructions) = map
            .iter()
            .find(|(_, instructions)| instructions.len() == 1)
            .expect(&format!(
                "Cannot find any more single instructions before ending the mapping\n{:#?}",
                result
            ));
        let instruction = *instructions.iter().next().unwrap();
        result.insert(*opcode, instruction);
        map.remove_entry(&opcode.clone());
        map.values_mut().for_each(|instructions| {
            instructions.remove(instruction);
        });
    }
    result
}

fn parse_line(line: &str) -> Option<Input> {
    if line.len() == 0 {
        return None;
    }

    let result = match line.chars().next().unwrap() {
        'B' => {
            let reg = into_registers(&line[9..]);
            Input::Before(reg)
        }
        'A' => {
            let reg = into_registers(&line[9..]);
            Input::After(reg)
        }
        _ => {
            let reg = into_registers(line);
            Input::Instr(reg)
        }
    };
    Some(result)
}

fn into_registers(line: &str) -> Registers {
    let reg = line
        .split(|c| c == ',' || c == ' ' || c == ']')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();

    debug_assert!(reg.len() == 4);

    [reg[0], reg[1], reg[2], reg[3]]
}

fn opcodes<'a>(before: &Registers, input: &Registers, after: &Registers) -> HashSet<&'a str> {
    INSTRUCTIONS
        .iter()
        .filter_map(|instr| {
            let result = execute(instr, input, before.clone());
            if result == *after {
                Some(instr)
            } else {
                None
            }
        })
        .map(|s| *s)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let instr = "addi";
        let input = [9, 2, 1, 2];
        let registers = [3, 2, 1, 1];

        let result = execute(instr, &input, registers);

        assert_eq!(result, [3, 2, 2, 1]);
    }

    #[test]
    fn test_mul() {
        let instr = "mulr";
        let input = [9, 2, 1, 2];
        let registers = [3, 2, 1, 1];

        let result = execute(instr, &input, registers);

        assert_eq!(result, [3, 2, 2, 1]);
    }

    #[test]
    fn test_opcodes() {
        let before = [3, 2, 1, 1];
        let input = [9, 2, 1, 2];
        let after = [3, 2, 2, 1];

        let expected = ["addi", "mulr", "seti"]
            .iter()
            .map(|x| *x)
            .collect::<Instructions>();

        let result = opcodes(&before, &input, &after);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse() {
        let line = "Before: [2, 0, 0, 1]";

        let result = parse_line(&line).expect("Failed to parse the line");

        assert_eq!(result, Input::Before([2, 0, 0, 1]));

        let line = "After:  [2, 0, -133, 0]";

        let result = parse_line(&line).expect("Failed to parse the line");

        assert_eq!(result, Input::After([2, 0, -133, 0]));

        let line = "2 0 -133 0";

        let result = parse_line(&line).expect("Failed to parse the line");

        assert_eq!(result, Input::Instr([2, 0, -133, 0]));
    }

    #[test]
    fn test_part1() {
        let input = ["Before: [3, 2, 1, 1]", "9 2 1 2", "After:  [3, 2, 2, 1]"]
            .iter()
            .map(|l| l.to_string());

        let result = parse_input(input).0;

        assert_eq!(result, 1);
    }
}

