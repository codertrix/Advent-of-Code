// ---------------------------------------------------
//  Advent of Code 2022
//  Day 3: Rucksack Reorganization
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day03.in");

pub fn run() {
    let rucksacks: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line
             .bytes()
             .map(|b| if b < 91 { b - 38 } else { b - 96 })
             .collect())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&rucksacks));
    println!("Result for puzzle 2: {}", puzzle_two(&rucksacks));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(rucksacks: &[Vec<u8>]) -> usize {
    rucksacks
        .iter()
        .map(|rucksack| {
            let compartments = rucksack.split_at(rucksack.len() / 2);

            *compartments.0
                .iter()
                .find(|item| compartments.1.contains(item))
                .unwrap() as usize
        })
        .sum()
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(rucksacks: &[Vec<u8>]) -> usize {
    rucksacks
        .chunks(3)
        .map(|group| {
            *group[0]
                .iter()
                .find(|item| group[1].contains(item) && group[2].contains(item))
                .unwrap() as usize
        })
        .sum()
}
