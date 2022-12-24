// ---------------------------------------------------
//  Advent of Code 2022
//  Day 23: Unstable Diffusion
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day23.in");

const NORTH: u8 = 0;
const SOUTH: u8 = 1;
const WEST: u8 = 2;
const EAST: u8 = 3;

pub fn run() {
    let mut elves = HashSet::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    let (result_one, result_two) = puzzle_one_two(&elves);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn adjacents_are_empty(elf: (isize, isize), elves: &HashSet<(isize, isize)>) -> bool {
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if elves.contains(&(elf.0 + dx, elf.1 + dy)) {
                return false;
            }
        }
    }

    true
}

fn propose_direction(elf: (isize, isize), elves: &HashSet<(isize, isize)>, adjacents: [(isize, isize); 3]) -> Option<(isize, isize)> {
    for (dx, dy) in adjacents {
        if elves.contains(&(elf.0 + dx, elf.1 + dy)) {
            return None;
        }
    }

    Some((elf.0 + adjacents[1].0, elf.1 + adjacents[1].1))
}

fn propose_move(elf: (isize, isize), elves: &HashSet<(isize, isize)>, first_direction: u8) -> (isize, isize) {
    let mut direction = first_direction;

    for _ in 0..4 {
        if let Some(proposed) = match direction {
            NORTH => propose_direction(elf, elves, [(-1, -1), (0, -1), (1, -1)]),
            SOUTH => propose_direction(elf, elves, [(-1, 1), (0, 1), (1, 1)]),
            WEST => propose_direction(elf, elves, [(-1, -1), (-1, 0), (-1, 1)]),
            EAST => propose_direction(elf, elves, [(1, -1), (1, 0), (1, 1)]),
            _ => unreachable!()
        } {
            return proposed
        };

        direction = (direction + 1) % 4;
    }

    elf
}

fn puzzle_one_two(elves: &HashSet<(isize, isize)>) -> (isize, usize) {
    let mut elves = elves.clone();
    let mut first_direction = NORTH;
    let mut result_one = 0;
    let mut result_two = 0;

    loop {
        let mut not_moved = HashSet::new();
        let mut proposed: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();

        result_two += 1;

        for elf in elves.clone() {
            if adjacents_are_empty(elf, &elves) {
                not_moved.insert(elf);
                continue;
            }

            proposed
                .entry(propose_move(elf, &elves, first_direction))
                .or_default()
                .push(elf);
        }

        if elves.len() == not_moved.len() {
            break;
        }

        first_direction = (first_direction + 1) % 4;
        elves = not_moved;

        for (elf, current) in &proposed {
            if current.len() == 1 {
                elves.insert(*elf);
            } else {
                for elf in current {
                    elves.insert(*elf);
                }
            }
        }

        if result_two == 10 {
            let (min_x, max_x, min_y, max_y) = elves
                .iter()
                .fold((isize::MAX, isize::MIN, isize::MAX, isize::MIN), |(min_x, max_x, min_y, max_y), (x, y)| {
                    (*x.min(&min_x), *x.max(&max_x), *y.min(&min_y), *y.max(&max_y))
                });

            result_one = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as isize;
        }
    }

    (result_one, result_two)
}
