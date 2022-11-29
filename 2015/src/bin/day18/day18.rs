// ---------------------------------------------------
//  Advent of Code 2015
//  Day 18: Like a GIF For Your Yard
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::mem::swap;

const INPUT: &str = include_str!("day18.in");

pub fn run() {
    let grid: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| (c == '#').into())
                .collect()
        })
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one_two(&grid, false));
    println!("Result for puzzle 2: {}", puzzle_one_two(&grid, true));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn neighbors_on_count(grid: &[Vec<usize>], x: usize, y: usize, x_max: usize, y_max: usize) -> usize {
    let x1 = if x > 0 { x - 1 } else { x };
    let y1 = if y > 0 { y - 1 } else { y };
    let x2 = x_max.min(x + 1);
    let y2 = y_max.min(y + 1);
    let mut count = 0;

    for (yy, row) in grid.iter().enumerate().take(y2 + 1).skip(y1) {
        for (xx, value) in row.iter().enumerate().take(x2 + 1).skip(x1) {
            if xx == x && yy == y { continue; }
            count += value;
        }
    }

    count
}

fn set_corners_on(grid: &mut [Vec<usize>], x_max: usize, y_max: usize) {
    grid[0][0] = 1;
    grid[0][x_max] = 1;
    grid[y_max][0] = 1;
    grid[y_max][x_max] = 1;
}

fn puzzle_one_two(grid: &[Vec<usize>], corners_stuck_on: bool) -> usize {
    let x_max = grid[0].len() - 1;
    let y_max = grid.len() - 1;
    let mut grid = grid.to_owned();
    let mut grid_new = grid.clone();

    if corners_stuck_on {
        set_corners_on(&mut grid, x_max, y_max);
    }

    for _ in 0..100 {
        for y in 0..=y_max {
            for x in 0..=x_max {
                match neighbors_on_count(&grid, x, y, x_max, y_max) {
                    2 => grid_new[y][x] = grid[y][x],
                    3 => grid_new[y][x] = 1,
                    _ => grid_new[y][x] = 0
                }
            }
        }

        swap(&mut grid, &mut grid_new); 

        if corners_stuck_on {
            set_corners_on(&mut grid, x_max, y_max);
        }
    }

    grid
        .iter()
        .map(|row| row.iter().sum::<usize>())
        .sum()
}
