// ---------------------------------------------------
//  Advent of Code 2024
//  Day 3: Mull It Over
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day03.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut idx = 0;
    let mut result_one = 0;
    let mut result_two = 0;
    let mut enabled = true;

    while idx < INPUT.len() {
        let Some(mul_idx) = INPUT[idx..].find("mul(") else {
            break;
        };

        let do_idx = INPUT[idx..idx + mul_idx].find("do()");
        let dont_idx = INPUT[idx..idx + mul_idx].find("don't()");

        idx += mul_idx + 4;

        match (do_idx, dont_idx) {
            (Some(do_idx), None) => {
                if do_idx < mul_idx {
                    enabled = true;
                }
            }
            (None, Some(dont_idx)) => {
                if dont_idx < mul_idx {
                    enabled = false;
                }
            }
            (Some(do_idx), Some(dont_idx)) => enabled = do_idx > dont_idx,
            (None, None) => (),
        }

        let Some(end_idx) = INPUT[idx..].find(')') else {
            break;
        };

        if let Some((a, b)) = INPUT[idx..idx + end_idx].split_once(',') {
            let Ok(a) = a.parse::<usize>() else {
                continue;
            };
            let Ok(b) = b.parse::<usize>() else {
                continue;
            };

            idx += end_idx + 1;
            result_one += a * b;

            if enabled {
                result_two += a * b;
            }
        }
    }

    (result_one, result_two)
}
