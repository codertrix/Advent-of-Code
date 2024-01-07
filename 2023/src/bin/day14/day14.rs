// ---------------------------------------------------
//  Advent of Code 2023
//  Day 14: Parabolic Reflector Dish
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day14.in");

pub fn run() {
    let mut platform: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (result_one, result_two) = puzzle_one_two(&mut platform);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}

fn roll_rocks(platform: &mut Vec<Vec<char>>) -> usize {
    let max_load = platform.len();
    let mut result = 0;

    for col in 0..platform[0].len() {
        let mut space = 0;

        for row in 0..max_load - 1 {
            match (platform[row][col], platform[row + 1][col]) {
                ('.', 'O') => {
                    platform[space][col] = 'O';
                    platform[row + 1][col] = '.';
                    result += max_load - space;
                    space += 1;
                }
                ('#', _) | ('.', '#') => space = row + 1,
                ('O', _) => {
                    result += max_load - row;
                    space = row + 1;
                }
                _ => (),
            }
        }

        if platform[max_load - 1][col] == 'O' {
            result += 1;
        }
    }

    result
}


// --------------
//  Puzzle 1 + 2
// --------------

const CYCLES: usize = 1_000_000_000;

fn puzzle_one_two(platform: &mut Vec<Vec<char>>) -> (usize, usize) {
    let result_one = roll_rocks(platform);
    let mut result_two = 0;

    let platform_size = platform.len();
    let mut seen = HashMap::new();
    let mut loads = Vec::new();

    for cycle in 0..CYCLES {
        for _ in 0..4 {
            roll_rocks(platform);

            let last_rotation = platform.to_vec();

            for (row_idx, mut row) in last_rotation.into_iter().enumerate() {
                for (col_idx, c) in row.iter_mut().enumerate() {
                    platform[col_idx][platform_size - 1 - row_idx] = *c;
                }
            }
        }

        let mut load = 0;

        for (row_idx, row) in platform.iter().enumerate() {
            for c in row {
                if *c == 'O' {
                    load += platform_size - row_idx;
                }
            }
        }

        if let Some(first_cycle) = seen.get(platform) {
            let interval = cycle - first_cycle;
            let idx = CYCLES - (CYCLES - first_cycle) / interval * interval - 1;

            result_two = loads[idx];
            break;
        }

        loads.push(load);
        seen.insert(platform.clone(), cycle);
    }

    (result_one, result_two)
}
