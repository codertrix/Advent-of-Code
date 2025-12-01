// ---------------------------------------------------
//  Advent of Code 2025
//  Day 1: Secret Entrance
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day01.in");

pub fn run() {
    let (password1, password2) = puzzle_one_two();

    println!("Result for puzzle 1: {}", password1);
    println!("Result for puzzle 2: {}", password2);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (i32, i32) {
    let mut pointer = 50;
    let mut password1 = 0;
    let mut password2 = 0;

    for line in INPUT.lines() {
        let mut amount: i32 = line[1..].parse().unwrap();

        password2 += amount / 100;
        amount %= 100;

        if line.starts_with("L") {
            if pointer > 0 && pointer <= amount {
                password2 += 1;
            }

            pointer -= amount;

            if pointer < 0 {
                pointer += 100;
            }
        } else {
            if 100 - pointer <= amount {
                password2 += 1;
            }

            pointer = (pointer + amount) % 100;
        }

        if pointer == 0 {
            password1 += 1;
        }
    }

    (password1, password2)
}
