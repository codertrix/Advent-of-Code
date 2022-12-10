// ---------------------------------------------------
//  Advent of Code 2022
//  Day 10: Cathode-Ray Tube
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day10.in");

pub fn run() {
    let input: Vec<_> = INPUT
        .lines()
        .map(|line| if line == "noop" {
            (1, 0)
        } else {
            let Some((_, value)) = line.split_once(' ') else { panic!("Incorrect input!") };
            (2, value.parse().unwrap())
        })
        .collect();

    let (result_one, result_two) = puzzle_one_two(&input);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn execute_cycle(cycle: &mut isize, x: isize) -> (isize, char) {
    let checkpoints = [20, 60, 100, 140, 180, 220];
    let sprite = x - 1..=x + 1;
    let pixel = if sprite.contains(&(*cycle % 40)) { '#' } else { ' ' };

    *cycle += 1;

    (if checkpoints.contains(cycle) { x * *cycle } else { 0 }, pixel)
}

fn puzzle_one_two(input: &[(isize, isize)]) -> (isize, String) {
    let mut sum = 0;
    let mut crt = String::from("\n\n");
    let mut cycle = 0;
    let mut x = 1;

    for command in input {
        let (signal_strength, pixel) = execute_cycle(&mut cycle, x);

        sum += signal_strength;
        crt.push(pixel);

        if command.0 == 2 {
            if cycle % 40 == 0 {
                crt.push('\n');
            }

            let (signal_strength, pixel) = execute_cycle(&mut cycle, x);

            sum += signal_strength;
            crt.push(pixel);
            x += command.1;
        }

        if cycle % 40 == 0 {
            crt.push('\n');
        }
    }

    (sum, crt)
}
