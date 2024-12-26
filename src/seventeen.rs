use std::ops::Div;

use crate::util::read_lines;

pub fn main() {
    let read_lines = read_lines("data/day-17/input.txt");
    let a: i64 = read_lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let b: i64 = read_lines[1].split_once(": ").unwrap().1.parse().unwrap();
    let c: i64 = read_lines[2].split_once(": ").unwrap().1.parse().unwrap();
    let instructions = read_lines[4].split_once(": ").unwrap().1;

    let mut machine = Machine::new(Registers { a, b, c }, instructions);

    loop {
        let step_success = machine.step();
        if !step_success {
            break;
        }
    }

    let output = machine
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    println!("{}", output);
}

struct Machine {
    registers: Registers,
    instructions: Vec<i64>,
    instruction_pointer: usize,
    output: Vec<i64>,
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
            output: Vec::new(),
        }
    }

    fn step(&mut self) -> bool {
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
        self.apply_instruction(instruction);

        return true;
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Adv { operand } => adv(operand, self),
            Instruction::Bxl { operand } => bxl(operand, self),
            Instruction::Bst { operand } => bst(operand, self),
            Instruction::Jnz { operand } => jnz(operand, self),
            Instruction::Bxc => bxc(self),
            Instruction::Out { operand } => out(operand, self),
            Instruction::Bdv { operand } => bdv(operand, self),
            Instruction::Cdv { operand } => cdv(operand, self),
        }
    }

    fn increase_instruction_pointer(&mut self) {
        self.instruction_pointer = self.instruction_pointer + 2;
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

fn bxl(operand: i64, machine: &mut Machine) {
    let number = machine.registers.b;
    let res = number ^ operand;
    machine.registers.b = res;

    machine.increase_instruction_pointer();
}

fn bst(operand: i64, machine: &mut Machine) {
    let operand = parse_combo_operator(operand, machine);
    let res = operand % 8;
    machine.registers.b = res;

    machine.increase_instruction_pointer();
}

fn jnz(operand: i64, machine: &mut Machine) {
    let a = machine.registers.a;
    if a == 0 {
        machine.increase_instruction_pointer();
        return;
    }

    machine.instruction_pointer = operand.try_into().unwrap();
}

fn bxc(machine: &mut Machine) {
    let b = machine.registers.b;
    let c = machine.registers.c;

    let res = b ^ c;
    machine.registers.b = res;

    machine.increase_instruction_pointer();
}

fn out(operand: i64, machine: &mut Machine) {
    let operand = parse_combo_operator(operand, machine);
    let mo = operand % 8;
    machine.output.push(mo);

    machine.increase_instruction_pointer();
}

fn adv(operand: i64, machine: &mut Machine) {
    let numerator = machine.registers.a;
    let operand = parse_combo_operator(operand, machine);
    let denominator: i64 = 2_i64.pow(operand.try_into().unwrap());

    let res = numerator.div(denominator);
    machine.registers.a = res;

    machine.increase_instruction_pointer();
}

fn bdv(operand: i64, machine: &mut Machine) {
    let numerator = machine.registers.a;
    let operand = parse_combo_operator(operand, machine);
    let denominator: i64 = 2_i64.pow(operand.try_into().unwrap());

    let res = numerator.div(denominator);
    machine.registers.b = res;

    machine.increase_instruction_pointer();
}

fn cdv(operand: i64, machine: &mut Machine) {
    let numerator = machine.registers.a;
    let operand = parse_combo_operator(operand, machine);
    let denominator: i64 = 2_i64.pow(operand.try_into().unwrap());

    let res = numerator.div(denominator);
    machine.registers.c = res;

    machine.increase_instruction_pointer();
}

fn parse_combo_operator(operand: i64, machine: &Machine) -> i64 {
    match operand {
        0..=3 => operand,
        4 => machine.registers.a,
        5 => machine.registers.b,
        6 => machine.registers.c,
        _ => unreachable!(),
    }
}
