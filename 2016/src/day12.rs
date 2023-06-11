// ---------------------------------------------------
//  Advent of Code 2016
//  Day 12: Leonardo's Monorail
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day12.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------");
    println!(" Day 12: Leonardo's Monorail");
    println!("-----------------------------");

    let executable = compile_program(INPUT);

    println!("Result for puzzle 1: {}", puzzle_one_two(&executable, 0));
    println!("Result for puzzle 2: {}", puzzle_one_two(&executable, 1));

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
    Jnzvr(isize, usize)
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
            _ => panic!("Unknown instruction!")
        }
    }

    program
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(executable: &Vec<Instruction>, value_c: isize) -> isize {
    let mut register = [0; 4];
    let mut pc: isize = 0;

    register[2] = value_c;

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
        }

        pc += 1;
    }

    register[0]
}
