// ---------------------------------------------------
//  Advent of Code 2022
//  Day 1: Calorie Counting
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day01.in");

pub fn run() {
    let mut elves_calories = Vec::new();
    let mut calories = 0;

    for line in INPUT.lines() {
        if let Ok(c) = line.parse::<usize>() {
            calories += c;
        } else {
            elves_calories.push(calories);
            calories = 0;
        }
    }

    elves_calories.push(calories);
    elves_calories.sort_unstable_by(|a, b| b.cmp(a));

    println!("Result for puzzle 1: {}", elves_calories[0]);
    println!("Result for puzzle 2: {}", elves_calories.into_iter().take(3).sum::<usize>());
}
