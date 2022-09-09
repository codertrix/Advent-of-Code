// ---------------------------------------------------
//  Advent of Code 2015
//  Day 23: Opening the Turing Lock
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day23.in");

pub fn run() {
    let mut program = Vec::new();

    for line in INPUT.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        program.push(match parts[0] {
            "hlf" => Instruction::Hlf(if parts[1].starts_with('a') { 0 } else { 1 }),
            "tpl" => Instruction::Tpl(if parts[1].starts_with('a') { 0 } else { 1 }),
            "inc" => Instruction::Inc(if parts[1].starts_with('a') { 0 } else { 1 }),
            "jie" => Instruction::Jie(if parts[1].starts_with('a') { 0 } else { 1 }, parts[2].parse::<isize>().unwrap_or(0)),
            "jio" => Instruction::Jio(if parts[1].starts_with('a') { 0 } else { 1 }, parts[2].parse::<isize>().unwrap_or(0)),
            _ => Instruction::Jmp(parts[1].parse::<isize>().unwrap_or(0)),
        });
    }

    println!("Result for puzzle 1: {}", puzzle_one(&program));
    println!("Result for puzzle 2: {}", puzzle_two(&program));
}

enum Instruction {
        Hlf(usize),
        Tpl(usize),
        Inc(usize),
        Jmp(isize),
        Jie(usize, isize),
        Jio(usize, isize)
}

fn execute(program: &[Instruction], a: usize) -> usize {
    let program_size = program.len();
    let mut instruction_pointer = 0;
    let mut register = [0usize; 2];

    register[0] = a;

    loop {
        if instruction_pointer >= program_size {
            break;
        }

        match program[instruction_pointer] {
            Instruction::Hlf(reg) => register[reg] /= 2,
            Instruction::Tpl(reg) => register[reg] *= 3,
            Instruction::Inc(reg) => register[reg] += 1,
            Instruction::Jmp(offset) => {
                let new_ip = instruction_pointer as isize + offset;

                if new_ip < 0 {
                    break;
                } else {
                    instruction_pointer = new_ip as usize;
                    continue;
                }
            }
            Instruction::Jie(reg, offset) => {
                if register[reg] % 2 == 0 {
                    let new_ip = instruction_pointer as isize + offset;

                    if new_ip < 0 {
                        break;
                    } else {
                        instruction_pointer = new_ip as usize;
                        continue;
                    }
                }
            }
            Instruction::Jio(reg, offset) => {
                if register[reg] == 1 {
                    let new_ip = instruction_pointer as isize + offset;

                    if new_ip < 0 {
                        break;
                    } else {
                        instruction_pointer = new_ip as usize;
                        continue;
                    }
                }
            }
        }

        instruction_pointer += 1;
    }

    register[1]
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(program: &[Instruction]) -> usize {
    execute(program, 0)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(program: &[Instruction]) -> usize {
    execute(program, 1)
}
