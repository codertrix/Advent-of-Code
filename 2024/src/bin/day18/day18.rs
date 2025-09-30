// ---------------------------------------------------
//  Advent of Code 2024
//  Day 18: RAM Run
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("day18.in");
const XY_MAX: i32 = 70;

pub fn run() {
    let bytes: Vec<(usize, usize)> = INPUT
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&bytes));
    println!("Result for puzzle 2: {}", puzzle_two(&bytes));
}

fn try_to_exit(grid: &[[char; XY_MAX as usize + 1]; XY_MAX as usize + 1]) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((0, 0, 0));

    while let Some((x, y, step)) = queue.pop_front() {
        if x == XY_MAX && y == XY_MAX {
            return step;
        }

        if seen.contains(&(x, y)) {
            continue;
        }

        seen.insert((x, y));

        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let x = x + dx;
            let y = y + dy;

            if x < 0 || y < 0 || x > XY_MAX || y > XY_MAX || grid[y as usize][x as usize] == '#' {
                continue;
            }

            queue.push_back((x, y, step + 1));
        }
    }

    0
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(bytes: &[(usize, usize)]) -> usize {
    let mut grid = [['.'; XY_MAX as usize + 1]; XY_MAX as usize + 1];

    for idx in 0..1024 {
        grid[bytes[idx].1][bytes[idx].0] = '#';
    }

    try_to_exit(&grid)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(bytes: &[(usize, usize)]) -> String {
    let mut grid = [['.'; XY_MAX as usize + 1]; XY_MAX as usize + 1];

    for idx in 0..bytes.len() {
        grid[bytes[idx].1][bytes[idx].0] = '#';

        if idx >= 1024 && try_to_exit(&grid) == 0 {
            return format!("{},{}", bytes[idx].0, bytes[idx].1);
        }
    }

    "Exit still reachable".to_string()
}
