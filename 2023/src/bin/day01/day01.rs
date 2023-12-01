// ---------------------------------------------------
//  Advent of Code 2023
//  Day 1: Trebuchet?!
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day01.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn digit_from(s: &str) -> Option<u32> {
    for (idx, d) in DIGITS.into_iter().enumerate() {
        if s.starts_with(d) {
            return Some(idx as u32 + 1);
        }
    }

    None
}

fn puzzle_one_two() -> (u32, u32) {
    let mut result_one = 0;
    let mut result_two = 0;

    for line in INPUT.lines() {
        let mut first_one = None;
        let mut last_one = None;
        let mut first_two = None;
        let mut last_two = 0;

        for (idx, c) in line.chars().enumerate() {
            let d = match c {
                '0'..='9' => {
                    let digit = c.to_digit(10);

                    if first_one.is_none() {
                        first_one = digit;
                    }

                    last_one = digit;
                    digit
                },
                'o' | 't' | 'f' | 's' | 'e' | 'n' => digit_from(&line[idx..]),
                _ => None
            };

            if let Some(a) = d {
                if first_two.is_none() {
                    first_two = Some(a);
                }

                last_two = a;
            }
        }

        result_one += first_one.unwrap() * 10 + last_one.unwrap();
        result_two += first_two.unwrap() * 10 + last_two;
    }

    (result_one, result_two)
}
