// ---------------------------------------------------
//  Advent of Code 2016
//  Day 13: A Maze of Twisty Little Cubicles
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashSet, VecDeque};

const INPUT: isize = 1358;
const TARGET: (isize, isize) = (31, 39);
const MAX_STEPS: usize = 50;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------------------");
    println!(" Day 13: A Maze of Twisty Little Cubicles");
    println!("------------------------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut result_two = None;

    queue.push_back((1, 1, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if result_two.is_none() && steps == MAX_STEPS + 1 {
            result_two = Some(seen.len());
        }

        if (x, y) == TARGET {
            return (steps, result_two.unwrap());
        }

        if seen.contains(&(x, y)) {
            continue;
        }

        seen.insert((x, y));

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (xx, yy) = (x + dx, y + dy);

            if xx < 0 || yy < 0 {
                continue;
            }

            let mut a = xx * xx + 3 * xx + 2 * xx * yy + yy + yy * yy + INPUT;
            let mut number_of_bits = 0;

            while a > 0 {
                number_of_bits += a & 1;
                a >>= 1;
            }

            if number_of_bits & 1 == 0 {
                queue.push_back((xx, yy, steps + 1));
            }
        }
    }

    unreachable!("Target not reachable!");
}
