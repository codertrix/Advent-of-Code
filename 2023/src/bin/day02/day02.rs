// ---------------------------------------------------
//  Advent of Code 2023
//  Day 2: Cube Conundrum
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day02.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;

    for line in INPUT.lines() {
        let (game, sets) = line.split_once(": ").unwrap();
        let (_, id) = game.split_once(' ').unwrap();
        let id: usize = id.parse().unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in sets.split("; ") {
            for colors in set.split(", ") {
                let (n, color) = colors.split_once(' ').unwrap();
                let n: usize = n.parse().unwrap();

                match color {
                    "red" => red = red.max(n),
                    "green" => green = green.max(n),
                    "blue" => blue = blue.max(n),
                    _ => panic!("There something wrong with the input."),
                }
            }
        }

        if red <= 12 && green <= 13 && blue <= 14 {
            result_one += id;
        }

        result_two += red * green * blue;
    }

    (result_one, result_two)
}
