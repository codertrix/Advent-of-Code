// ---------------------------------------------------
//  Advent of Code 2023
//  Day 17: Clumsy Crucible
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("day17.in");

pub fn run() {
    let map: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as usize).collect())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one_two(&map, 1, 3));
    println!("Result for puzzle 2: {}", puzzle_one_two(&map, 4, 10));
}


// --------------
//  Puzzle 1 + 2
// --------------

#[derive(Eq)]
struct State {
    hl: usize,
    row: i32,
    col: i32,
    dir: usize,
    step: u8,
}

impl State {
    fn new(hl: usize, row: i32, col: i32, dir: usize, step: u8) -> Self {
        Self { hl, row, col, dir, step, }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.hl.cmp(&self.hl)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.hl == other.hl
    }
}

fn puzzle_one_two(map: &[Vec<usize>], steps_min: u8, steps_max: u8) -> usize {
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let mut queue = BinaryHeap::new();
    let mut heat_losses = vec![vec![usize::MAX; max_col + 1]; max_row + 1];
    let mut seen = HashSet::new();

    queue.push(State::new(0, 0, 0, usize::MAX, 0));

    while let Some(State { hl, row, col, dir, step }) = queue.pop() {
        if seen.contains(&(row, col, dir, step)) {
            continue;
        }

        seen.insert((row, col, dir, step));
        heat_losses[row as usize][col as usize] = hl.min(heat_losses[row as usize][col as usize]);

        'outer: for (new_dir, (d_row, d_col)) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().enumerate() {
            if new_dir ^ 1 == dir || new_dir == dir && step == steps_max {
                continue;
            }

            let new_step = if new_dir == dir { step + 1 } else { steps_min };
            let step_count = if new_step == steps_min { steps_min } else { 1 };
            let mut d_hl = 0;
            let (mut r, mut c) = (row, col);

            for _ in 0..step_count {
                r += d_row;
                c += d_col;

                if !(0..=max_row as i32).contains(&r) || !(0..=max_col as i32).contains(&c) {
                    continue 'outer;
                }

                d_hl += map[r as usize][c as usize];
            }

            queue.push(State::new(hl + d_hl, r, c, new_dir, new_step));
        }
    }

    heat_losses[max_row][max_col]
}
