// ---------------------------------------------------
//  Advent of Code 2023
//  Day 3: Gear Ratios
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day03.in");

pub fn run() {
    let es: Vec<Vec<_>> = INPUT.lines().map(|line| line.bytes().collect()).collect();
    let (result_one, result_two) = puzzle_one_two(&es);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn number_is_part( es: &[Vec<u8>], row: usize, col: usize, n_len: usize,) -> (bool, Option<(usize, usize)>) {
    let start_row = if row > 0 { row - 1 } else { row };
    let stop_row = if row < es.len() - 1 { row + 1 } else { row };
    let start_col = if col - n_len > 0 { col - n_len - 1 } else { col - n_len };

    for (r, row) in es.iter().enumerate().take(stop_row + 1).skip(start_row) {
        for (c, symbol) in row.iter().enumerate().take(col + 1).skip(start_col) {
            match symbol {
                b'0'..=b'9' | b'.' => {}
                b'*' => return (true, Some((r, c))),
                _ => return (true, None),
            }
        }
    }

    (false, None)
}

fn puzzle_one_two(es: &[Vec<u8>]) -> (usize, usize) {
    let mut result_one = 0;
    let mut possible_gears = Vec::new();
    let mut n = 0;
    let mut n_len = 0;

    for (row, line) in es.iter().enumerate() {
        for (col, symbol) in line.iter().enumerate() {
            let is_digit = symbol.is_ascii_digit();

            if is_digit {
                n = n * 10 + (symbol - b'0') as usize;
                n_len += 1;
            }

            if n_len > 0 && !is_digit || col == line.len() - 1 {
                let (is_part, pos) = number_is_part(es, row, col, n_len);

                if is_part {
                    result_one += n;
                }

                if let Some(pos) = pos {
                    possible_gears.push((pos, n));
                }

                n = 0;
                n_len = 0;
            }
        }
    }

    possible_gears.sort_unstable();

    let result_two = possible_gears
        .windows(2)
        .map(|possible_gear| if possible_gear[0].0 == possible_gear[1].0 {
            possible_gear[0].1 * possible_gear[1].1
        } else {
            0
        })
        .sum();

    (result_one, result_two)
}
