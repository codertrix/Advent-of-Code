// ---------------------------------------------------
//  Advent of Code 2022
//  Day 5: Supply Stacks
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day05.in");

pub fn run() {
    let Some(input) = INPUT.split_once("\n\n") else { panic!() };
    let mut stacks_one = Vec::new();

    for line in input.0.lines().rev() {
        for (idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                stacks_one.push(Vec::new());
            }
            
            if c.is_ascii_uppercase() {
                stacks_one[(idx - 1) / 4].push(c);
            }
        }
    }

    let mut stacks_two = stacks_one.clone();

    input.1
        .lines()
        .for_each(|line| {
            let step: Vec<_> = line.split(' ').collect();
            let count = step[1].parse::<usize>().unwrap();
            let from = step[3].parse::<usize>().unwrap() - 1;
            let to = step[5].parse::<usize>().unwrap() - 1;

            for _ in 0..count {
                let c = stacks_one[from].pop().unwrap();
                stacks_one[to].push(c);
            }

            let len = stacks_two[from].len();
            let mut temp_stack: Vec<_> = stacks_two[from]
                .drain((len - count)..)
                .collect();
            stacks_two[to].append(&mut temp_stack);
        });

    let result_one: String = stacks_one
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect();

    let result_two: String = stacks_two
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}
