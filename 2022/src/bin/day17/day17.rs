// ---------------------------------------------------
//  Advent of Code 2022
//  Day 17: Pyroclastic Flow
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

// Warning: This solution works well with my input.
//          With the example input it runs very, very long! (I don't know if it would sovle the example)
//          So it's not a good general solution.

use std::collections::HashSet;

const INPUT: &str = include_str!("day17.in");
const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;

type Rock = [(i64, i64); 5];

const ROCK_A: Rock = [(0, 0), (3, 0), (1, 0), (2, 0), (0, 0)];
const ROCK_B: Rock = [(0, 1), (2, 1), (1, 2), (1, 0), (1, 1)];
const ROCK_C: Rock = [(0, 0), (2, 0), (2, 2), (1, 0), (2, 1)];
const ROCK_D: Rock = [(0, 1), (0, 2), (0, 3), (0, 0), (0, 0)];
const ROCK_E: Rock = [(0, 0), (1, 1), (0, 1), (1, 0), (0, 0)];

const ROCKS: [Rock; 5] = [ROCK_A, ROCK_B, ROCK_C, ROCK_D, ROCK_E];

pub fn run() {
    let jet_pattern: Vec<_> = INPUT
        .trim()
        .chars()
        .collect();

    let (result_one, result_two) = puzzle_one_two(&jet_pattern);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn check_dx(rock: &[(i64, i64)], rested_rocks: &HashSet<(i64, i64)>, dx: i64) -> i64 {
    if dx == -1 {
        if rock[LEFT].0 == 0 {
            return 0;
        }
    } else if rock[RIGHT].0 == 6 {
        return 0;
    }

    for (x, y) in rock {
        if rested_rocks.contains(&(*x + dx, *y)) {
            return 0;
        }
    }

    dx
}

fn puzzle_one_two(jet_pattern: &[char]) -> (i64, i64) {
    let mut result_one = 0;
    let result_two;

    let mut jet_idx = 0;
    let mut rock_idx = 0;
    let mut rock_count = 0;
    let mut tower_top = 0;
    let mut tops = [0; 7];
    let mut rested_rocks = HashSet::from([(0, 0),(1, 0),(2, 0),(3, 0),(4, 0),(5, 0),(6, 0)]);

    let mut max_rock_count: i64 = 1_000_000_000_000;
    let mut mem_jet_idx = 0;
    let mut mem_rock_idx = 0;
    let mut mem_rock_count = 0;
    let mut mem_tower_top = 0;

    let mut interval = 0;
    let mut tower_top_interval = 0;

    loop {
        rock_count += 1;
        let mut rock = Vec::from(ROCKS[rock_idx]);

        for r in rock.iter_mut() {
            r.0 += 2;
            r.1 += tower_top + 4;
        }

        'fall: loop {
            let dx = match jet_pattern[jet_idx] {
                '<' => check_dx(&rock, &rested_rocks, -1),
                '>' => check_dx(&rock, &rested_rocks, 1),
                _ => 0
            };

            for (x, y) in rock.iter_mut() {
                *x += dx;
                *y -= 1;
            }

            jet_idx = (jet_idx + 1) % jet_pattern.len();

            for r in &rock {
                if rested_rocks.contains(r) {
                    break 'fall;
                }
            }
        }

        for (x, y) in &rock {
            tops[*x as usize] = tops[*x as usize].max(*y + 1);
            rested_rocks.insert((*x, *y + 1));
        }

        rock_idx = (rock_idx + 1) % 5;
        tower_top = tower_top.max(rock[TOP].1 + 1);

        if rock_count == 2022 {
            result_one = tower_top;
        }

        if rock_count == max_rock_count {
            result_two = tower_top + (interval - 1) * tower_top_interval;
            break;
        }

        let (_, is_flat_bottom) = tops
            .iter()
            .fold((tower_top, true), |a, b| (a.0, a.1 && a.0 == *b));

        if is_flat_bottom {
            if rock_idx == mem_rock_idx && jet_idx == mem_jet_idx {
                let rock_count_interval = rock_count - mem_rock_count;

                interval= (max_rock_count - mem_rock_count) / rock_count_interval;
                max_rock_count = max_rock_count - (interval * rock_count_interval + mem_rock_count) + rock_count;
                tower_top_interval = tower_top - mem_tower_top;
            } else {
                mem_jet_idx = jet_idx;
                mem_rock_idx = rock_idx;
                mem_rock_count = rock_count;
                mem_tower_top = tower_top;
            }
        }
    }

    (result_one, result_two)
}
