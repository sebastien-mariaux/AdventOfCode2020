use std::collections::BTreeMap;
use std::fs;
struct Instruction {
    name: String,
    value: i32,
    done: bool,
}

impl Instruction {
    fn new(full_instruction: &str) -> Instruction {
        let mut split_instruction = full_instruction.split(' ');
        Instruction {
            name: split_instruction.next().unwrap().to_string(),
            value: split_instruction.next().unwrap().parse::<i32>().unwrap(),
            done: false,
        }
    }
}

fn run_instruction(
    mut acc: u32,
    mut index: u32,
    mut program: BTreeMap<u32, Instruction>,
) -> Option<(u32, u32, BTreeMap<u32, Instruction>)> {
    let instruction = program.get_mut(&index).unwrap();
    if instruction.done {
        return None;
    }
    match instruction.name.as_str() {
        "nop" => {
            index += 1;
        }
        "acc" => {
            index += 1;
            acc = acc.wrapping_add(instruction.value as u32)
        }
        "jmp" => index = index.wrapping_add(instruction.value as u32),
        _ => panic!("Impossible!"),
    }
    instruction.done = true;
    Some((acc, index, program))
}

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let mut program: BTreeMap<u32, Instruction> = BTreeMap::new();
    data.lines().enumerate().for_each(|(idx, line)| {
        program.insert(idx as u32, Instruction::new(line));
    });
    let mut acc = 0;
    let mut next_position = 0;
    while let Some(result) = run_instruction(acc, next_position, program) {
        acc = result.0;
        next_position = result.1;
        program = result.2;
    }
    println!("Oy, oy, oy, the result is {}", acc);
}
