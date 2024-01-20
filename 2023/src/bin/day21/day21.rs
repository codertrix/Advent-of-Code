// ---------------------------------------------------
//  Advent of Code 2023
//  Day 21: Step Counter
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("day21.in");

pub fn run() {
    let map: Vec<Vec<_>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (result_one, result_two) = puzzle_one_two(&map);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(map: &[Vec<char>]) -> (usize, isize) {
    let map_len = map.len() as isize;
    let mut reachables = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((map_len / 2, map_len / 2, 0, true));

    while let Some((x, y, step, is_even)) = queue.pop_front() {
        if reachables.contains_key(&(x, y)) {
            continue;
        }

        reachables.insert((x, y), (step, is_even));

        if step == 327 {
            continue;
        }

        for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let mut xx = x % map_len;
            let mut yy = y % map_len;

            if xx < 0 {
                xx += map_len;
            }

            if yy < 0 {
                yy += map_len;
            }

            if map[yy as usize][xx as usize] != '#' {
                queue.push_back((x, y, step + 1, !is_even));
            }
        }
    }

    let result_one = reachables
        .values()
        .filter(|(step, is_even)| *is_even && *step <= 64)
        .count();

    let y1 = reachables
        .values()
        .filter(|(step, is_even)| !*is_even && *step <= 65)
        .count() as isize;
    let y2 = reachables
        .values()
        .filter(|(step, is_even)| *is_even && *step <= 196)
        .count() as isize;
    let y3 = reachables
        .values()
        .filter(|(_, is_even)| !*is_even)
        .count() as isize;

    let a = (y3 - 2 * y2 + y1) / 2;
    let b = y2 - y1 - a;
    let c = y1;
    let n = (26501365 - 65) / 131;

    (result_one, a * n * n + b * n + c)
}
