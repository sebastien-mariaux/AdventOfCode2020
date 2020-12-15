use std::collections::BTreeMap;
use std::fs;

struct Instruction {
    name: String,
    value: i32,
    done: bool,
}

impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        Instruction {
            name: self.name.clone(),
            value: self.value,
            done: self.done,
        }
    }
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

    fn change_name(mut self) -> Instruction {
        self.name = match self.name.as_str() {
            "nop" => "jmp".to_string(),
            "jmp" => "nop".to_string(),
            "acc" => "acc".to_string(),
            _ => panic!("Impossible"),
        };
        self
    }
}

fn run_instruction(
    mut acc: u32,
    mut index: u32,
    mut program: BTreeMap<u32, Instruction>,
) -> Result<(u32, u32, BTreeMap<u32, Instruction>), bool> {
    let instruction = match program.get_mut(&index) {
        Some(x) => x,
        None => return Err(true),
    };
    if instruction.done {
        return Err(false);
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
    Ok((acc, index, program))
}

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let mut counter = 0;
    loop {
        match run_program(&data, counter) {
            true => break,
            false => {
                counter += 1;
            },
        };
    }
    println!("over.")
}

fn run_program(data: &String, modify: u32) -> bool {
    let mut program: BTreeMap<u32, Instruction> = BTreeMap::new();
    data.lines().enumerate().for_each(|(idx, line)| {
        program.insert(idx as u32, Instruction::new(line));
    });

    let original_instruction = program.get_mut(&modify).unwrap();
    *original_instruction = original_instruction.clone().change_name();
    let mut acc = 0;
    let mut next_position = 0;
    let program_complete;
    loop {
        match run_instruction(acc, next_position, program) {
            Ok(result) => {
                acc = result.0;
                next_position = result.1;
                program = result.2;
            }
            Err(x) => {
                program_complete = x;
                break;
            }
        }
    }
    println!("Program completed: {}", program_complete);
    println!("Oy, oy, oy, the result is {}", acc);
    program_complete
}
