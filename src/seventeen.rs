use crate::util::read_lines;

pub fn main() {
    let read_lines = read_lines("data/day-17/input.txt");
    let a: i64 = read_lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let b: i64 = read_lines[1].split_once(": ").unwrap().1.parse().unwrap();
    let c: i64 = read_lines[2].split_once(": ").unwrap().1.parse().unwrap();
    let instructions = read_lines[4].split_once(": ").unwrap().1;

    let machine = Machine::new(Registers { a, b, c }, instructions);

    loop {
        let step_success = machine.step();
        if !step_success {
            break;
        }
    }
}

struct Machine {
    registers: Registers,
    instructions: Vec<i64>,
    instruction_pointer: usize,
}

struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

impl Machine {
    fn new(registers: Registers, instructions: &str) -> Machine {
        let instructions: Vec<i64> = instructions
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        Machine {
            registers,
            instructions,
            instruction_pointer: 0,
        }
    }

    fn step(&self) -> bool {
        let instruction_pointer = self.instruction_pointer;
        let instruction = self.instructions.get(instruction_pointer);
        let operand = self.instructions.get(instruction_pointer + 1);

        if instruction.is_none() {
            return false;
        }

        if operand.is_none() {
            println!("Operand is none! What to do?");
            return false;
        }

        let instruction = parse_instruction(instruction.unwrap().clone(), operand.unwrap().clone());

        return true;
    }
}

fn parse_instruction(operation: i64, operand: i64) -> Instruction {
    match operation {
        0 => Instruction::Adv { operand },
        1 => Instruction::Bxl { operand },
        2 => Instruction::Bst { operand },
        3 => Instruction::Jnz { operand },
        4 => Instruction::Bxc,
        5 => Instruction::Out { operand },
        6 => Instruction::Bdv { operand },
        7 => Instruction::Cdv { operand },
        _ => unreachable!(),
    }
}

enum Instruction {
    Adv { operand: i64 },
    Bxl { operand: i64 },
    Bst { operand: i64 },
    Jnz { operand: i64 },
    Bxc,
    Out { operand: i64 },
    Bdv { operand: i64 },
    Cdv { operand: i64 },
}
