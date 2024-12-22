// ---------------------------------------------------
//  Advent of Code 2024
//  Day 8: Resonant Collinearity
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day08.in");

pub fn run() {
    let mut antennas = HashMap::new();
    let x_len = INPUT.lines().next().unwrap().len() as isize;
    let y_len = INPUT.lines().count() as isize;

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((x as isize, y as isize));
            }
        }
    }

    println!("Result for puzzle 1: {}", puzzle_one(&antennas, x_len, y_len));
    println!("Result for puzzle 2: {}", puzzle_two(&antennas, x_len, y_len));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(antennas: &HashMap<char, Vec<(isize, isize)>>, x_len: isize, y_len: isize) -> usize {
    let mut antinodes = HashSet::new();

    for positions in antennas.values() {
        for idx1 in 0..positions.len() {
            let (x1, y1) = positions[idx1];

            for idx2 in (idx1..positions.len()).skip(1) {
                let (x2, y2) = positions[idx2];
                let dx = x2 - x1;
                let dy = y2 - y1;

                let ax = x1 - dx;
                let ay = y1 - dy;

                if ax >= 0 && ax < x_len && ay >= 0 && ay < y_len {
                    antinodes.insert((ax, ay));
                }

                let ax = x2 + dx;
                let ay = y2 + dy;

                if ax >= 0 && ax < x_len && ay >= 0 && ay < y_len {
                    antinodes.insert((ax, ay));
                }
            }
        }
    }

    antinodes.len()
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(antennas: &HashMap<char, Vec<(isize, isize)>>, x_len: isize, y_len: isize) -> usize {
    let mut antinodes = HashSet::new();

    for positions in antennas.values() {
        for idx1 in 0..positions.len() {
            let (x1, y1) = positions[idx1];

            for idx2 in (idx1..positions.len()).skip(1) {
                let (x2, y2) = positions[idx2];
                let dx = x2 - x1;
                let dy = y2 - y1;

                let mut ax;
                let mut ay;

                for n in 0.. {
                    ax = x1 - n * dx;
                    ay = y1 - n * dy;

                    if ax >= 0 && ax < x_len && ay >= 0 && ay < y_len {
                        antinodes.insert((ax, ay));
                    } else {
                        break;
                    }
                }

                for n in 0.. {
                    ax = x2 + n * dx;
                    ay = y2 + n * dy;

                    if ax >= 0 && ax < x_len && ay >= 0 && ay < y_len {
                        antinodes.insert((ax, ay));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len()
}
