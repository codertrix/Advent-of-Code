// ---------------------------------------------------
//  Advent of Code 2021
//  Day 2: Dive!
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day02.in");

pub fn run() {
    let (aim, depth, position) = INPUT
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .fold((0, 0, 0), |(aim, depth, pos), (instruction, value)| {
            let amount: usize = value.parse().unwrap();

            match instruction {
                "forward" => (aim, depth + aim * amount, pos + amount),
                "down" => (aim + amount, depth, pos),
                "up" => (aim - amount, depth, pos),
                _ => panic!("Bad input data!")
            }
        });

    println!("Result for puzzle 1: {}", position * aim);
    println!("Result for puzzle 2: {}", position * depth);
}
