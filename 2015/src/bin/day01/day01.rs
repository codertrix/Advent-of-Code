// ---------------------------------------------------
//  Advent of Code 2015
//  Day 1: Not Quite Lisp
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day01.in");

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> isize {
    INPUT
        .trim()
        .chars()
        .fold(0, |a, c| a + match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        })
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two() -> usize {
    let mut floor = 0;

    for (idx, c) in INPUT.trim().chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        };

        if floor == -1 {
            return idx + 1;
        }
    }

    0
}
