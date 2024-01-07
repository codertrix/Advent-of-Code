// ---------------------------------------------------
//  Advent of Code 2023
//  Day 13: Point of Incidence
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day13.in");

pub fn run() {
    let patterns: Vec<_> = INPUT.split("\n\n").collect();
    let (result_one, result_two) = puzzle_one_two(patterns);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn difference_count(row_a: &str, row_b: &str) -> u32 {
    row_a
        .chars()
        .zip(row_b.chars())
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum()
}

fn reflections_for(pattern: &[String]) -> (usize, usize) {
    let mut mirror_one = 0;
    let mut mirror_two = 0;

    for (idx, rows) in pattern.windows(2).enumerate() {
        if rows[0] == rows[1] || difference_count(&rows[0], &rows[1]) == 1 {
            let mut differences = 0;

            for offset in 0..(idx + 1).min(pattern.len() - idx - 1) {
                differences += difference_count(&pattern[idx - offset], &pattern[idx + 1 + offset]);
            }

            if differences == 0 {
                mirror_one = idx + 1;
            }

            if differences == 1 {
                mirror_two = idx + 1;
            }
        }
    }

    (mirror_one, mirror_two)
}

fn puzzle_one_two(patterns: Vec<&str>) -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;

    for pattern in &patterns {
        let pattern: Vec<String> = pattern.lines().map(|line| line.to_owned()).collect();
        let (r_one, r_two) = reflections_for(&pattern);

        result_one += r_one * 100;
        result_two += r_two * 100;

        let mut rotated_pattern = Vec::new();

        for idx in 0..pattern[0].len() {
            let mut new_row = String::with_capacity(pattern.len());

            for row in pattern.iter().rev() {
                new_row.push(row.chars().nth(idx).unwrap());
            }

            rotated_pattern.push(new_row);
        }

        let (r_one, r_two) = reflections_for(&rotated_pattern);

        result_one += r_one;
        result_two += r_two;
    }

    (result_one, result_two)
}
