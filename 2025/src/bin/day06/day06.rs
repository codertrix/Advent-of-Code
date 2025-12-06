// ---------------------------------------------------
//  Advent of Code 2025
//  Day 6: Trash Compactor
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day06.in");

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());
}

// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> u64 {
    let mut lines: Vec<Vec<&str>> = INPUT.lines().map(|line| line.split_whitespace().collect()).collect();
    let operators = lines.pop().unwrap();
    let mut numbers = Vec::new();
    let mut result = 0;

    for (idx, operator) in operators.iter().enumerate() {
        numbers.clear();

        for line in &lines {
            numbers.push(line[idx].parse().unwrap());
        }

        match *operator {
            "+" => result += numbers.iter().sum::<u64>(),
            "*" => result += numbers.iter().product::<u64>(),
            _ => (),
        }
    }

    result
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two() -> u64 {
    let mut lines: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let operators = lines.pop().unwrap();
    let mut idx = operators.len();
    let mut numbers = Vec::new();
    let mut result = 0;

    while idx > 0 {
        let mut number = 0;
        idx -= 1;

        for line in &lines {
            if let Some(digit) = line[idx].to_digit(10) {
                number = number * 10 + digit as u64;
            }
        }

        if number > 0 {
            numbers.push(number);
        } else {
            numbers.clear()
        }

        match operators[idx] {
            '+' => result += numbers.iter().sum::<u64>(),
            '*' => result += numbers.iter().product::<u64>(),
            _ => (),
        }
    }

    result
}
