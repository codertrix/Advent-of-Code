// ---------------------------------------------------
//  Advent of Code 2016
//  Day 18: Like a Rogue
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::mem::swap;

const INPUT: &str = "^^.^..^.....^..^..^^...^^.^....^^^.^.^^....^.^^^...^^^^.^^^^.^..^^^^.^^.^.^.^.^.^^...^^..^^^..^.^^^^";

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------");
    println!(" Day 18: Like a Rogue");
    println!("----------------------");

    println!("Result for puzzle 1: {}", puzzle_one_two(40));
    println!("Result for puzzle 2: {}", puzzle_one_two(400000));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(row_count: usize) -> usize {
    let mut current_row: Vec<_> = INPUT
        .chars()
        .map(|c| c == '.')
        .collect();
    let mut safe_tiles_count = current_row
        .iter()
        .fold(0, |a, b| if *b { a + 1 } else { a });
    let mut next_row = Vec::new();
    let last_idx = current_row.len() - 1;

    for _ in 0..row_count - 1 {
        let mut tile;

        next_row.clear();

        for idx in 0..=last_idx {
            if idx == 0 {
                tile = current_row[idx + 1];
            } else if idx == last_idx {
                tile = current_row[idx - 1];
            } else {
                tile = current_row[idx - 1] == current_row[idx + 1];
            }

            next_row.push(tile);

            if tile {
                safe_tiles_count += 1;
            }
        }

        swap(&mut current_row, &mut next_row);
    }

    safe_tiles_count
}
