// ---------------------------------------------------
//  Advent of Code 2016
//  Day 1: No Time for a Taxicab
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day01.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------");
    println!(" Day 1: No Time for a Taxicab");
    println!("------------------------------");

    let (result_one, result_two) = puzzle_one_two(INPUT);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

fn puzzle_one_two(instructions: &str) -> (isize, isize) {
    let mut direction = NORTH;
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut locations = HashSet::new();
    let mut result_two = None;

    for instruction in instructions.trim().split(", ") {
        direction = if instruction[0..1] == *"R" { direction + 1 } else { direction + 3 } % 4;
        let steps = instruction[1..].parse().unwrap_or(0);

        for _ in 0..steps {
            match direction {
                NORTH => y += 1,
                EAST => x += 1,
                SOUTH => y -= 1,
                WEST => x -= 1,
                _ => panic!("There is something wrong!")
            }

            if result_two.is_none() {
                if locations.contains(&(x, y)) {
                    result_two = Some(x.abs() + y.abs());
                }

                locations.insert((x, y));
            }
        }
    }

    (x.abs() + y.abs(), result_two.unwrap_or(0))
}
