// ---------------------------------------------------
//  Advent of Code 2021
//  Day 25: Sea Cucumber
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day25.in");

pub fn run() {
    let mut seafloor = Vec::new();

    for line in INPUT.lines() {
        let row: Vec<char> = line
            .chars()
            .collect();

        seafloor.push(row);
    }

    println!("Result for puzzle 1: {}", puzzle_one(&seafloor));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(seafloor: &[Vec<char>]) -> usize {
    let mut seafloor = seafloor.to_owned();
    let mut cucumbers_moved = true;
    let mut step = 0;
    let len_col = seafloor[0].len();
    let len_row = seafloor.len();

    while cucumbers_moved {
        cucumbers_moved = false;
        step += 1;

        let mut next_seafloor = Vec::new();

        for row in seafloor {
            let mut next_row = row.clone();

            for col in 0..len_col {
                let next_col = (col + 1) % len_col;

                if row[col] == '>' && row[next_col] == '.' {
                    next_row[col] = '.';
                    next_row[next_col] = '>';
                    cucumbers_moved = true;
                }
            }

            next_seafloor.push(next_row);
        }

        seafloor = next_seafloor.clone();

        for row in 0..len_row {
            let next_row = (row + 1) % len_row;

            for col in 0..len_col {
                if seafloor[row][col] == 'v' && seafloor[next_row][col] == '.' {
                    next_seafloor[row][col] = '.';
                    next_seafloor[next_row][col] = 'v';
                    cucumbers_moved = true;
                }
            }
        }

        seafloor = next_seafloor;
    }

    step
}


// ----------------------------------------
//  There was no second puzzle on this day
// ----------------------------------------
