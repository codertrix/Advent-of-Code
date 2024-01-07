// ---------------------------------------------------
//  Advent of Code 2023
//  Day 16: The Floor Will Be Lava
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("day16.in");

pub fn run() {
    let contraption: Vec<Vec<_>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    println!("Result for puzzle 1: {}", puzzle_one(&contraption));
    println!("Result for puzzle 2: {}", puzzle_two(&contraption));
}

use Direction::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn energized_tiles(grid: &[Vec<char>], row: usize, col: usize, dir: Direction) -> usize {
    let grid_size = grid.len() - 1;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((row, col, dir));

    while let Some((row, col, dir)) = queue.pop_front() {
        if seen.contains(&(row, col, dir)) {
            continue;
        }

        seen.insert((row, col, dir));

        match (dir, grid[row][col]) {
            (Up, '.') | (Up, '|') => {
                if row > 0 {
                    queue.push_back((row - 1, col, dir));
                }
            }
            (Up, '/') | (Down, '\\') => {
                if col < grid_size {
                    queue.push_back((row, col + 1, Right));
                }
            }
            (Up, '\\') | (Down, '/') => {
                if col > 0 {
                    queue.push_back((row, col - 1, Left));
                }
            }
            (Down, '.') | (Down, '|') => {
                if row < grid_size {
                    queue.push_back((row + 1, col, dir));
                }
            }
            (Up, '-') | (Down, '-') => {
                if col > 0 {
                    queue.push_back((row, col - 1, Left));
                }
                if col < grid_size {
                    queue.push_back((row, col + 1, Right));
                }
            }
            (Left, '.') | (Left, '-') => {
                if col > 0 {
                    queue.push_back((row, col - 1, dir));
                }
            }
            (Left, '/') | (Right, '\\') => {
                if row < grid_size {
                    queue.push_back((row + 1, col, Down));
                }
            }
            (Left, '\\') | (Right, '/') => {
                if row > 0 {
                    queue.push_back((row - 1, col, Up));
                }
            }
            (Right, '.') | (Right, '-') => {
                if col < grid_size {
                    queue.push_back((row, col + 1, dir));
                }
            }
            (Right, '|') | (Left, '|') => {
                if row > 0 {
                    queue.push_back((row - 1, col, Up));
                }
                if row < grid_size {
                    queue.push_back((row + 1, col, Down));
                }
            }
            _ => {}
        }
    }

    seen.into_iter()
        .map(|(row, col, _)| (row, col))
        .collect::<HashSet<_>>()
        .len()
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(grid: &[Vec<char>]) -> usize {
    energized_tiles(grid, 0, 0, Right)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(grid: &[Vec<char>]) -> usize {
    let grid_size = grid.len() - 1;
    let mut max_energized_tiles = 0;

    for idx in 0..grid_size {
        max_energized_tiles = max_energized_tiles.max(energized_tiles(grid, idx, 0, Right));
        max_energized_tiles = max_energized_tiles.max(energized_tiles(grid, idx, grid_size, Left));
        max_energized_tiles = max_energized_tiles.max(energized_tiles(grid, 0, idx, Down));
        max_energized_tiles = max_energized_tiles.max(energized_tiles(grid, grid_size, idx, Up));
    }

    max_energized_tiles
}
