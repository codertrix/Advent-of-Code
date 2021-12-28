// ---------------------------------------------------
//  Advent of Code 2021
//  Day 15: Chiton
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const INPUT: &str = include_str!("day15.in");

pub fn run() {
    let grid: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|line| line
             .bytes()
             .map(|b| (b - 48) as usize)
             .collect())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&grid));
    println!("Result for puzzle 2: {}", puzzle_two(&grid));
}

#[derive(Eq)]
struct Node {
    x: usize,
    y: usize,
    cost: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost
            .cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn adjacents(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut adjacents = Vec::new();

    if x > 0 { adjacents.push((x - 1, y)); }
    if y > 0 { adjacents.push((x, y - 1)); }
    if x < x_max { adjacents.push((x + 1, y)); }
    if y < y_max { adjacents.push((x, y + 1)); }

    adjacents
}

fn shortest_path(grid: &[Vec<usize>]) -> Option<usize> {
    let dest_x = grid[0].len() - 1;
    let dest_y = grid.len() - 1;
    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();

    distances.insert((0, 0), 0);
    queue.push(Node { x: 0, y: 0, cost: 0 });

    while let Some(Node { x, y, cost }) = queue.pop() {
        if x == dest_x && y == dest_y {
            return Some(cost);
        }

        if let Some(distance) = distances.get(&(x, y)) {
            if distance < &cost { continue; }
        }

        for (xx, yy) in adjacents(x, y, dest_x, dest_y) {
            if distances.get(&(xx, yy)).is_none() {
                let risk_level = grid[yy][xx];

                queue.push(Node { x: xx, y: yy, cost: cost + risk_level });
                distances.insert((xx, yy), cost + risk_level);
            }
        }
    }

    None
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(grid: &[Vec<usize>]) -> usize {
    shortest_path(grid)
        .expect("No path to destination!")
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(grid: &[Vec<usize>]) -> usize {
    let mut big_grid = Vec::new();

    for yy in 0..5 {
        for y in 0..grid.len() {
            let mut row = Vec::new();

            for xx in 0..5 {
                for x in 0..grid[0].len() {
                    let value = grid[y][x] + xx + yy;

                    row.push(if value <= 9 { value } else { value - 9 });
                }
            }

            big_grid.push(row);
        }
    }

    shortest_path(&big_grid)
        .expect("No path to destination!")
}
