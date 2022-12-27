// ---------------------------------------------------
//  Advent of Code 2022
//  Day 25: Full of Hot Air
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day25.in");

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> String {
    let mut sum: isize = INPUT
        .lines()
        .map(|line| {
            let mut number = 0;

            for c in line.chars() {
                number *= 5;

                match c {
                    '2' => number += 2,
                    '1' => number += 1,
                    '-' => number -= 1,
                    '=' => number -= 2,
                    _ => ()
                }
            }

            number
        })
        .sum();

    let mut snafu = Vec::new();

    while sum > 0 {
        let mut digit = sum % 5;

        if digit > 2 {
            digit -= 5;
        }

        sum -= digit;
        sum /= 5;

        snafu.push(digit);
    }

    snafu
        .iter()
        .rev()
        .map(|n| match n {
            2 => '2',
            1 => '1',
            -1 => '-',
            -2 => '=',
            _ => '0'
        })
        .collect()
}

// ----------------------------------------
//  There was no second puzzle on this day
// ----------------------------------------
