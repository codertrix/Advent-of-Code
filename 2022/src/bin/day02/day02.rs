// ---------------------------------------------------
//  Advent of Code 2022
//  Day 2: Rock Paper Scissors
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day02.in");

pub fn run() {
    let scores = INPUT
        .lines()
        .fold((0, 0), |a, b| {
            let round_scores = match b {
                "A X" => (4, 3),
                "B Y" => (5, 5),
                "C Z" => (6, 7),
                "A Z" => (3, 8),
                "B X" => (1, 1),
                "C Y" => (2, 6),
                "A Y" => (8, 4),
                "B Z" => (9, 9),
                "C X" => (7, 2),
                _ => panic!()
            };

            (a.0 + round_scores.0, a.1 + round_scores.1)
        });

    println!("Result for puzzle 1: {}", scores.0);
    println!("Result for puzzle 2: {}", scores.1);
}
