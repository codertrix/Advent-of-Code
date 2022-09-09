// ---------------------------------------------------
//  Advent of Code 2015
//  Day 3: Perfectly Spherical Houses in a Vacuum
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day03.in");

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one_two(1));
    println!("Result for puzzle 2: {}", puzzle_one_two(2));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(puzzle: usize) -> usize {
    let mut houses: HashSet<(isize, isize)> = HashSet::new();
    let mut locations = [[0; 2]; 2];

    houses.insert((0, 0));

    INPUT
        .trim()
        .chars()
        .enumerate()
        .for_each(|(idx, c)| {
            let idx = idx % puzzle;
            match c {
                '^' => locations[idx][1] -= 1,
                'v' => locations[idx][1] += 1,
                '>' => locations[idx][0] += 1,
                '<' => locations[idx][0] -= 1,
                _ => unreachable!()
            };

            houses.insert((locations[idx][0], locations[idx][1]));
        });

    houses.len()
}
