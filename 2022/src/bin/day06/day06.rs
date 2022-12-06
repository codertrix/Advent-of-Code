// ---------------------------------------------------
//  Advent of Code 2022
//  Day 6: Tuning Trouble
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day06.in");

pub fn run() {
    let input: Vec<_> = INPUT
        .chars()
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one_two(&input, 4));
    println!("Result for puzzle 2: {}", puzzle_one_two(&input, 14));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(stream: &[char], len: usize) -> usize {
    let mut marker_chars: HashMap<char, isize> = HashMap::new();

    for (idx, &c) in stream.iter().enumerate() {
        *marker_chars.entry(c).or_default() += 1;

        if idx >= len {
            let c = stream[idx - len];
            let Some(cc) = marker_chars.get_mut(&c) else { panic!(); };

            *cc -= 1;

            if *cc == 0 {
                marker_chars.remove(&c);
            }

            if marker_chars.len() == len {
                return idx + 1;
            }
        }
    }

    panic!("No message in stream for length {}!", len);
}
