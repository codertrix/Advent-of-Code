// ---------------------------------------------------
//  Advent of Code 2015
//  Day 8: Matchsticks
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day08.in");

pub fn run() {
    let results = puzzle_one_two();

    println!("Result for puzzle 1: {}", results.0);
    println!("Result for puzzle 2: {}", results.1);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;

    for line in INPUT.lines() {
        result_one += line.len();
        result_two += 2;

        let mut chars = line.chars();

        while let Some(c) = chars.next() {
            if c == '\\' {
                result_two += 1;

                if let Some(c) = chars.next() {
                    match c {
                        'x' => {
                            chars.next();
                            chars.next();
                            result_two += 3;
                        },
                        '"' | '\\' => result_two += 2,
                        _ => panic!("Invalid input")
                    }
                }
            }

            if c == '"' {
                result_two += 1;
            } else {
                result_one -= 1;
            }

            result_two += 1;
        }

        result_two -= line.len();

    }

    (result_one, result_two)
}
