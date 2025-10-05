// ---------------------------------------------------
//  Advent of Code 2024
//  Day 25: Code Chronicle
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day25.in");

pub fn run() {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut heights = [0; 5];
    let mut is_key = false;

    for (idx, line) in INPUT.lines().enumerate() {
        let idx = idx & 0b111;

        match idx {
            0 => is_key = line == ".....",
            6 => {
                if is_key {
                    keys.push(heights);
                } else {
                    locks.push(heights);
                }

                heights = [0; 5];
            }
            _ => {
                for (idx, c) in line.chars().enumerate() {
                    if c == '#' {
                        heights[idx] += 1;
                    }
                }
            }
        }
    }

    println!("Result for puzzle 1: {}", puzzle_one(&keys, &locks));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(keys: &[[u32; 5]], locks: &[[u32; 5]]) -> usize {
    let mut pairs = 0;

    for lock in locks {
        'k: for key in keys {
            for idx in 0..5 {
                if lock[idx] + key[idx] > 5 {
                    continue 'k;
                }
            }

            pairs += 1;
        }
    }

    pairs
}

// ------------------------------------------
//   There was no second puzzle on this day
// ------------------------------------------
