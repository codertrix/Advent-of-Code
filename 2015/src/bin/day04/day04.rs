// ---------------------------------------------------
//  Advent of Code 2015
//  Day 4: The Ideal Stocking Stuffer
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use md5_rs::Context;

const INPUT: &str = "bgvyzdsv";

pub fn run() {
    let results = puzzle_one_two();

    println!("Result for puzzle 1: {}", results.0);
    println!("Result for puzzle 2: {}", results.1);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut number = 1;
    let mut puzzle_one = None;
    let mut puzzle_two = None;

    loop {
        if let Some(result_one) = puzzle_one {
            if let Some(result_two) = puzzle_two {
                return (result_one, result_two);
            }
        }

        let message = format!("{}{}", INPUT, number);
        let mut context = Context::new();

        context.read(message.as_bytes());

        let digest = context.finish();

        if digest[0] == 0 && digest[1] == 0 {
            if puzzle_one.is_none() && (digest[2] & 0xf0) == 0 {
                puzzle_one = Some(number);
            }

            if puzzle_two.is_none() && digest[2] == 0 {
                puzzle_two = Some(number);
            }
        }

        number += 1;
    }
}
