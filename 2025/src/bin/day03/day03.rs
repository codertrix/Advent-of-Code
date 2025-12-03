// ---------------------------------------------------
//  Advent of Code 2025
//  Day 3: Lobby
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day03.in");

pub fn run() {
    let banks: Vec<Vec<u32>> = INPUT
        .lines()
        .map(|bank| bank.chars().map(|battery| battery.to_digit(10).unwrap()).collect())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one_two(&banks, 2));
    println!("Result for puzzle 2: {}", puzzle_one_two(&banks, 12));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(banks: &[Vec<u32>], battery_count: usize) -> usize {
    let mut output_joltage = 0;
    let bank_len = banks[0].len();

    for bank in banks {
        let mut first_idx = 0;
        let mut joltage: usize = 0;

        for batteries_left in (0..battery_count).rev() {
            let mut value = 0;

            for (idx, v) in bank.iter().enumerate().take(bank_len - batteries_left).skip(first_idx) {
                if value < *v {
                    first_idx = idx + 1;
                    value = *v;

                    if value == 9 {
                        break;
                    }
                }
            }

            joltage = joltage * 10 + value as usize;
        }

        output_joltage += joltage;
    }

    output_joltage
}
