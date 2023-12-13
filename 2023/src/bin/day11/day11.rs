// ---------------------------------------------------
//  Advent of Code 2023
//  Day 11: Cosmic Expansion
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day11.in");

pub fn run() {
    let mut galaxies = Vec::new();
    let mut rows_to_expand = Vec::new();
    let mut cols_to_expand = Vec::new();

    for (idx_row, line) in INPUT.lines().enumerate() {
        if !line.contains('#') {
            rows_to_expand.push(idx_row);
        }

        for (idx_col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((idx_row, idx_col));
            }
        }
    }

    let cols_with_galaxy: Vec<usize> = galaxies.iter().map(|galaxy| galaxy.1).collect();

    for col in 0..*cols_with_galaxy.iter().max().unwrap() {
        if !cols_with_galaxy.contains(&col) {
            cols_to_expand.push(col);
        }
    }

    let (result_one, result_two) = puzzle_one_two(&galaxies, &rows_to_expand, &cols_to_expand);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(galaxies: &[(usize, usize)], rows_to_expand: &[usize], cols_to_expand: &[usize]) -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;

    for (idx, (row_a, col_a)) in galaxies.iter().enumerate() {
        for (row_b, col_b) in galaxies.iter().skip(idx) {
            let row_min = row_a.min(row_b);
            let row_max = row_a.max(row_b);
            let col_min = col_a.min(col_b);
            let col_max = col_a.max(col_b);
            let shortest_path = row_max - row_min + col_max - col_min;

            result_one += shortest_path;
            result_two += shortest_path;

            for row in rows_to_expand {
                if row_min < row && row < row_max {
                    result_one += 1;
                    result_two += 999999;
                }
            }

            for col in cols_to_expand {
                if col_min < col && col < col_max {
                    result_one += 1;
                    result_two += 999999;
                }
            }
        }
    }

    (result_one, result_two)
}
