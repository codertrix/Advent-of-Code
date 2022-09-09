// ---------------------------------------------------
//  Advent of Code 2015
//  Day 25: Let It Snow
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

// Input: To continue, please consult the code grid in the manual.  Enter the code at row 2978, column 3083.

const INPUT_COL: usize = 3083;
const INPUT_ROW: usize = 2978;

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> usize {
    let mut start_row = 1;
    let mut col = 1;
    let mut row = 1;
    let mut code = 20151125;

    loop {
        if row == INPUT_ROW && col == INPUT_COL {
            return code;
        }

        if row == 1 {
            start_row += 1;
            row = start_row;
            col = 1;
        } else {
            row -= 1;
            col += 1;
        }

        code = (code * 252533) % 33554393;
    }
}


// ----------------------------------------
//  There was no second puzzle on this day
// ----------------------------------------
