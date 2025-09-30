// ---------------------------------------------------
//  Advent of Code 2024
//  Day 17: Chronospatial Computer
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const INPUT: &str = include_str!("day17.in");

pub fn run() {
    let mut lines = INPUT.lines();
    let mut regs = [0; 3];

    for (idx, line) in lines.by_ref().enumerate() {
        if let Some((_, value)) = line.split_once(": ") {
            regs[idx] = value.parse().unwrap();
        } else {
            break;
        }
    }

    let program: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&regs, &program));
    println!("Result for puzzle 2: {}", puzzle_two(&program));
}

struct Computer {
    regs: [u64; 3],
    program: Vec<u64>,
    ip: usize,
    out: Vec<u64>,
}

impl Computer {
    fn run(&mut self) {
        loop {
            if self.ip >= self.program.len() {
                return;
            }

            let instruction = self.program[self.ip];
            let literal = self.program[self.ip + 1];
            let combo = if literal > 3 && literal < 7 {
                self.regs[literal as usize % 4]
            } else {
                literal
            };
            self.ip += 2;

            match instruction {
                0 => self.regs[A] /= 2_u64.pow(combo as u32), // adv
                1 => self.regs[B] ^= literal,                 // bxl
                2 => self.regs[B] = combo % 8,                // bst
                3 => {
                    if self.regs[A] != 0 {
                        self.ip = literal as usize;
                    }
                } // jnz
                4 => self.regs[B] ^= self.regs[C],            // bxc
                5 => self.out.push(combo % 8),                // out
                6 => self.regs[B] = self.regs[A] / 2_u64.pow(combo as u32), // bdv
                7 => self.regs[C] = self.regs[A] / 2_u64.pow(combo as u32), // cdv
                _ => panic!("Invalid instruction!"),
            }
        }
    }
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(regs: &[u64; 3], program: &[u64]) -> String {
    let mut computer = Computer {
        regs: *regs,
        program: program.to_vec(),
        ip: 0,
        out: Vec::new(),
    };

    computer.run();
    computer
        .out
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(program: &[u64]) -> u64 {
    let mut computer = Computer {
        regs: [0; 3],
        program: program.to_vec(),
        ip: 0,
        out: Vec::new(),
    };
    let mut interval = 8_u64.pow(program.len() as u32 - 1);
    let mut initial_reg_a = interval;

    for idx in (0..program.len()).rev() {
        for n in 0..8 {
            computer.regs = [initial_reg_a + n * interval, 0, 0];
            computer.ip = 0;
            computer.out.clear();
            computer.run();

            if computer.out[idx] == program[idx] {
                initial_reg_a += n * interval;
                break;
            }
        }

        interval /= 8;
    }

    initial_reg_a
}
