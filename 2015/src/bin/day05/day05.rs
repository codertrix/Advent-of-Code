// ---------------------------------------------------
//  Advent of Code 2015
//  Day 5: Doesn't He Have Intern-Elves For This?
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day05.in");

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> usize {
    let mut nice_count = 0;

    'line:
    for line in INPUT.lines() {
        let mut left;
        let mut right = ' ';
        let mut vowels = 0;
        let mut double_letters = false;

        for c in line.chars() {
            left = right;
            right = c;

            if ["ab", "cd", "pq", "xy"].contains(&format!("{}{}", left, right).as_str()) {
                continue 'line;
            }

            if ['a', 'e', 'i', 'o', 'u'].contains(&right) {
                vowels += 1;
            }

            double_letters |= left == right;
        }

        if double_letters && vowels >= 3 {
            nice_count += 1;
        }
    }

    nice_count
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two() -> usize {
    let mut nice_count = 0;

    for line in INPUT.lines() {
        let mut letter_repeats = false;

        for idx in 0..line.len() - 2 {
            if line.get(idx..=idx) == line.get(idx + 2..=idx + 2) {
                letter_repeats = true;
                break;
            }
        }

        if letter_repeats {
            for idx in 0..line.len() - 1 {
                if let Some(pair) = line.get(idx..=idx + 1) {
                    if line[idx + 2..].contains(pair) {
                        nice_count += 1;
                        break;
                    }
                }
            }
        }
    }

    nice_count
}
