// ---------------------------------------------------
//  Advent of Code 2021
//  Day 8: Seven Segment Search
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day08.in");

pub fn run() {
    let mut notes: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    INPUT
        .lines()
        .map(|s| s.split_once(" | ").unwrap())
        .for_each(|(p, o)| {
            notes.push((p.split_whitespace().collect::<Vec<&str>>(), o.split_whitespace().collect::<Vec<&str>>()));
        });

    println!("Result for puzzle 1: {}", puzzle_one(&notes));
    println!("Result for puzzle 2: {}", puzzle_two(&notes));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(notes: &[(Vec<&str>, Vec<&str>)]) -> usize {
    notes
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .map(|out_digit| match out_digit.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}


// ----------
//  Puzzle 2
// ----------

fn digit_x<'a>(digit: u8, condition: usize, patterns: &mut Vec<&'a str>, digits: &mut [&str; 10]) -> &'a str {
    let mut index = None;

    for (idx, pattern) in patterns.iter().enumerate() {
        if pattern.chars().filter(|&c| !digits[digit as usize].contains(c)).count() == condition {
            index = Some(idx);
        }
    };

    if let Some(idx) = index {
        patterns.remove(idx)
    } else {
        panic!("No match found!");
    }

}

fn sorted(from: &str) -> String {
    let mut chars: Vec<char> = from
        .chars()
        .collect();
    chars.sort_unstable();

    String::from_iter(chars)
}

fn puzzle_two(notes: &[(Vec<&str>, Vec<&str>)]) -> usize {
    notes
        .iter()
        .map(|(patterns, output)| {
            let mut digits = [""; 10];
            let mut out235 = Vec::new();
            let mut out069 = Vec::new();

            for &p in patterns {
                match p.len() {
                    2 => digits[1] = p,
                    3 => digits[7] = p,
                    4 => digits[4] = p,
                    7 => digits[8] = p,
                    5 => out235.push(p),
                    6 => out069.push(p),
                    _ => {}
                }
            }

            digits[3] = digit_x(1, 3, &mut out235, &mut digits);
            digits[2] = digit_x(4, 3, &mut out235, &mut digits);
            digits[5] = out235[0];

            digits[6] = digit_x(7, 4, &mut out069, &mut digits);
            digits[0] = digit_x(3, 2, &mut out069, &mut digits);
            digits[9] = out069[0];

            let digit_map: HashMap<String, usize> = digits
                .iter()
                .enumerate()
                .map(|(n, digit)| (sorted(digit), n))
                .collect();

            output
                .iter()
                .fold(0, |a, digit| a * 10 + digit_map[&sorted(digit)])
        })
        .sum()
}
