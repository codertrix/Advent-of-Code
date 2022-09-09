// ---------------------------------------------------
//  Advent of Code 2015
//  Day 6: Probably a Fire Hazard
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day06.in");

pub fn run() {
    let results = puzzle_one_two();

    println!("Result for puzzle 1: {}", results.0);
    println!("Result for puzzle 2: {}", results.1);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (u32, u32) {
    let mut lights_one = [[false; 1000]; 1000];
    let mut lights_two = [[0; 1000]; 1000];

    for line in INPUT.lines() {
        let line_parts: Vec<&str> = line
            .split(' ')
            .collect();

        let instruction = if line_parts.len() == 4 { 2 } else if line_parts[1] == "on" { 0 } else { 1 };

        let top_left: (usize, usize) = line_parts[line_parts.len() - 3]
            .split_once(',')
            .map(|s| (s.0.parse().unwrap(), s.1.parse().unwrap()))
            .unwrap();
        let bottom_right: (usize, usize) = line_parts[line_parts.len() - 1]
            .split_once(',')
            .map(|s| (s.0.parse().unwrap(), s.1.parse().unwrap()))
            .unwrap();

        for row in top_left.1..=bottom_right.1 {
            for col in top_left.0..=bottom_right.0 {
                match instruction {
                    0 => {
                        lights_one[row][col] = true;
                        lights_two[row][col] += 1;
                    }
                    1 => {
                        lights_one[row][col] = false;
                        if lights_two[row][col] > 0 {
                            lights_two[row][col] -= 1;
                        }
                    }
                    2 => { lights_one[row][col] = !lights_one[row][col];
                        lights_two[row][col] += 2;
                    }
                    _ => unreachable!()
                }
            }
        }
    }

    let result_one = lights_one
        .iter()
        .map(|row| row
              .iter()
              .fold(0, |aa, bb| if *bb { aa + 1 } else { aa }))
        .sum();

    let result_two = lights_two
        .iter()
        .map(|row| row
             .iter()
             .sum::<u32>())
        .sum();

    (result_one, result_two)
}
