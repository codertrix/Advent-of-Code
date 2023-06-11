// ---------------------------------------------------
//  Advent of Code 2016
//  Day 25: Clock Signal
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day25.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------");
    println!(" Day 25: Clock Signal");
    println!("----------------------");

    println!("Result for puzzle 1: {}", puzzle_one());

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}

#[derive(Clone)]
pub enum Instruction {
    Cpyr(usize, usize),
    Cpyv(isize, usize),
    Inc(usize),
    Dec(usize),
    Jnzro(usize, isize),
    Jnzrr(usize, usize),
    Jnzvo(isize, isize),
    Jnzvr(isize, usize),
    Out(usize)
}

fn index_for(register: &str) -> usize {
    (register
        .bytes()
        .next()
        .unwrap() - 97) as usize
}

fn compile_program(input: &str) -> Vec<Instruction> {
    let mut program = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line
            .split_whitespace()
            .collect();

        match parts[0] {
            "cpy" => {
                let reg_idx = index_for(parts[2]);

                if let Ok(value) = parts[1].parse() {
                    program.push(Instruction::Cpyv(value, reg_idx));
                } else {
                    program.push(Instruction::Cpyr(index_for(parts[1]), reg_idx));
                }
            }
            "inc" => program.push(Instruction::Inc(index_for(parts[1]))),
            "dec" => program.push(Instruction::Dec(index_for(parts[1]))),
            "jnz" => {
                if let Ok(offset) = parts[2].parse() {
                    if let Ok(value) = parts[1].parse() {
                        program.push(Instruction::Jnzvo(value, offset));
                    } else {
                        program.push(Instruction::Jnzro(index_for(parts[1]), offset));
                    }
                } else if let Ok(value) = parts[1].parse() {
                    program.push(Instruction::Jnzvr(value, index_for(parts[2])));
                } else {
                    program.push(Instruction::Jnzrr(index_for(parts[1]), index_for(parts[2])));
                }
            }
            "out" => program.push(Instruction::Out(index_for(parts[1]))),
            _ => panic!("Unknown instruction!")
        }
    }

    program
}

fn run(executable: &[Instruction], register: [isize; 4], pc: isize) -> (isize, [isize; 4], isize) {
    let mut register = register;
    let mut pc = pc;

    while pc >= 0 && pc < executable.len() as isize {
        match executable[pc as usize] {
            Instruction::Cpyr(from_reg, to_reg) => register[to_reg] = register[from_reg],
            Instruction::Cpyv(value, to_reg) => register[to_reg] = value,
            Instruction::Inc(reg) => register[reg] += 1,
            Instruction::Dec(reg) => register[reg] -= 1,
            Instruction::Jnzro(reg, offset) => if register[reg] != 0 { pc += offset - 1 },
            Instruction::Jnzrr(reg, reg_offset) => if register[reg] != 0 { pc += register[reg_offset] - 1 },
            Instruction::Jnzvo(value, offset) => if value != 0 { pc += offset - 1 },
            Instruction::Jnzvr(value, reg_offset) => if value != 0 { pc += register[reg_offset] - 1 },
            Instruction::Out(reg) => return (register[reg], register, pc + 1)
        }

        pc += 1;
    }

    (0, register, pc)
}

// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> isize {
    let executable = compile_program(INPUT);
    let mut number = 0;

    loop {
        let (mut out, mut registers, mut pc) = run(&executable, [number, 0, 0, 0], 0);
        let mut last_out = out;
        let mut count = 1;

        while count < 50 {
            (out, registers, pc) = run(&executable, registers, pc);
            if last_out + out != 1 {
                break;
            } else {
                last_out = out;
                count += 1;
            }
        }

        if count == 50 {
            break;
        }

        number += 1;
    }

    number
}


// ----------------------------------------
//  There was no second puzzle on this day
// ----------------------------------------
