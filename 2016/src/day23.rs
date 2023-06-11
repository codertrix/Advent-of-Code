// ---------------------------------------------------
//  Advent of Code 2016
//  Day 23: Safe Cracking
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day23.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------");
    println!(" Day 23: Safe Cracking");
    println!("-----------------------");

    let executable = compile_program(INPUT);

    println!("Result for puzzle 1: {}", puzzle_one(executable));
    println!("Result for puzzle 2: {}", puzzle_two(12));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Cpyr(usize, usize),
    Cpyv(isize, usize),
    Inc(usize),
    Dec(usize),
    Jnzro(usize, isize),
    Jnzrr(usize, usize),
    Jnzvo(isize, isize),
    Jnzvr(isize, usize),
    Tgl(usize),
    Skipro(usize, isize),
    Skipvo(isize, isize)
}

fn index_for(register: &str) -> usize {
    (register
        .bytes()
        .next()
        .unwrap() - 97) as usize
}

pub fn compile_program(input: &str) -> Vec<Instruction> {
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
            "tgl" => program.push(Instruction::Tgl(index_for(parts[1]))),
            _ => panic!("Unknown instruction!")
        }
    }

    program
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(mut executable: Vec<Instruction>) -> isize {
    let mut register = [7, 0, 0, 0];
    let mut pc: isize = 0;

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
            Instruction::Tgl(reg) => {
                let toggle = pc + register[reg];

                if toggle >= 0 && toggle < executable.len() as isize {
                    let toggle = toggle as usize;

                    match executable[toggle] {
                        Instruction::Cpyr(a, b) => executable[toggle] = Instruction::Jnzrr(a, b),
                        Instruction::Cpyv(a, b) => executable[toggle] = Instruction::Jnzvr(a, b),
                        Instruction::Inc(a) => executable[toggle] = Instruction::Dec(a),
                        Instruction::Dec(a) => executable[toggle] = Instruction::Inc(a),
                        Instruction::Jnzro(a, b) => executable[toggle] = Instruction::Skipro(a, b),
                        Instruction::Jnzrr(a, b) => executable[toggle] = Instruction::Cpyr(a, b),
                        Instruction::Jnzvo(a, b) => executable[toggle] = Instruction::Skipvo(a, b),
                        Instruction::Jnzvr(a, b) => executable[toggle] = Instruction::Cpyv(a, b),
                        Instruction::Tgl(a) => executable[toggle] = Instruction::Inc(a),
                        Instruction::Skipro(a, b) => executable[toggle] = Instruction::Jnzro(a, b),
                        Instruction::Skipvo(a, b) => executable[toggle] = Instruction::Jnzvo(a, b),
                    }
                }
            }
            _ => ()
        }

        pc += 1;
    }

    register[0]
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(value_a: usize) -> usize {
    let mut result = 1;

    for x in 1..=value_a {
        result *= x;
    }

    result + 75 * 72
}
