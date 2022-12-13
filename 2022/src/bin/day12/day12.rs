// ---------------------------------------------------
//  Advent of Code 2022
//  Day 12: Hill Climbing Algorithm
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("day12.in");

pub fn run() {
    let mut start = (0, 0);
    let mut end  = (0, 0);
    let mut grid: Vec<_> = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| line
             .bytes()
             .enumerate()
             .map(|(x, c)| {
                 match c {
                     b'S' => start = (x, y),
                     b'E' => end = (x, y),
                     _ => ()
                 }
                 c
             })
             .collect::<Vec<_>>())
        .collect();

    grid[start.1][start.0] = b'a';
    grid[end.1][end.0] = b'z';

    println!("Result for puzzle 1: {}", puzzle_one_two(&grid, start, end, false));
    println!("Result for puzzle 2: {}", puzzle_one_two(&grid, start, end, true));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn adjacents(x: usize, y: usize, grid: &[Vec<u8>], reverse: bool) -> Vec<(usize, usize)> {
    let mut adjacents = Vec::new();

    if x > 0 { adjacents.push((x - 1, y)); }
    if y > 0 { adjacents.push((x, y - 1)); }
    if x < grid[0].len() - 1 { adjacents.push((x + 1, y)); }
    if y < grid.len() - 1 { adjacents.push((x, y + 1)); }

    adjacents
        .into_iter()
        .filter(|(xx, yy)| (grid[*yy][*xx] as i8 - grid[y][x] as i8) * if reverse { -1 } else { 1 } <= 1)
        .collect()
}

fn puzzle_one_two(grid: &[Vec<u8>], start: (usize, usize), end: (usize, usize), reverse: bool) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    if reverse {
        queue.push_back((0, end.0, end.1));
    } else {
        queue.push_back((0, start.0, start.1));
    }

    while let Some((distance, x, y)) = queue.pop_front() {
        if reverse {
            if grid[y][x] == b'a' { return distance; }
        } else if end == (x, y) {
            return distance;
        }

        if seen.contains(&(x, y)) { continue; }
        seen.insert((x, y));

        for (xx, yy) in adjacents(x, y, grid, reverse) {
            queue.push_back((distance + 1, xx, yy));
        }
    }

    unreachable!("Oops! There is no path up the hill.")
}
