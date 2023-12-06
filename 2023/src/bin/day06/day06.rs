// ---------------------------------------------------
//  Advent of Code 2023
//  Day 6: Wait For It
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day06.in");

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());
}

fn ways_to_win(time: usize, distance: usize) -> usize {
    let mut ms = 1;

    let start_to_win = loop {
        if (time - ms) * ms > distance {
            break ms;
        }

        ms += 1;
    };

    ms = time;
    let end_to_win = loop {
        if (time - ms) * ms > distance {
            break ms;
        }

        ms -= 1;
    };

    end_to_win - start_to_win + 1
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> usize {
    let mut result = 1;
    let mut lines = INPUT.lines();

    let times: Vec<usize> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<usize> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();

    for race in 0..times.len() {
        result *= ways_to_win(times[race], distances[race]);
    }

    result
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two() -> usize {
    ways_to_win(41968894, 214178911271055)
}
