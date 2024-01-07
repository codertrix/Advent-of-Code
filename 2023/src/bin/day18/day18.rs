// ---------------------------------------------------
//  Advent of Code 2023
//  Day 18: Lavaduct Lagoon
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day18.in");

pub fn run() {
    let input: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&input));
    println!("Result for puzzle 2: {}", puzzle_two(&input));
}

fn cubic_meters(instructions: &[(char, isize)]) -> isize {
    let mut x = 0;
    let mut y = 0;
    let mut trench = 0;
    let mut lagoon = vec![(x, y)];

    for (direction, count) in instructions {
        match direction {
            'U' => y -= count,
            'D' => y += count,
            'L' => x -= count,
            _ => x += count,
        }

        trench += count;
        lagoon.push((x, y));
    }

    lagoon
        .windows(2)
        .map(|segment| (segment[0].0 * segment[1].1 - segment[1].0 * segment[0].1))
        .sum::<isize>() / 2 + (trench / 2 + 1)
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(input: &[Vec<&str>]) -> isize {
    let instructions: Vec<(char, isize)> = input
        .iter()
        .map(|parts| (parts[0].chars().next().unwrap(), parts[1].parse().unwrap()))
        .collect();

    cubic_meters(&instructions)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(input: &[Vec<&str>]) -> isize {
    let directions = ['R', 'D', 'L', 'U'];
    let instructions: Vec<(char, isize)> = input
        .iter()
        .map(|parts| {
            let idx: usize = parts[2][7..8].parse().unwrap();
            let mut distance = 0_isize;

            for c in parts[2][2..7].chars() {
                distance <<= 4;
                distance += c.to_digit(16).unwrap() as isize;
            }

            (directions[idx], distance)
        })
        .collect();

    cubic_meters(&instructions)
}
