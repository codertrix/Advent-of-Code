// ---------------------------------------------------
//  Advent of Code 2021
//  Day 11: Dumbo Octopus
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use core::cmp::{min, max};

const INPUT: &str = include_str!("day11.in");

pub fn run() {
    let mut grid: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|line| line
             .bytes()
             .map(|b| (b - 48) as usize)
             .collect())
        .collect();

    let x_max = (grid[0].len() - 1) as isize;
    let y_max = (grid.len() - 1) as isize;
    let mut flash_count = 0;
    let mut step = 1;

    loop {
        for row in grid.iter_mut() {
            for col in row.iter_mut() {
                *col += 1;
            }
        }

        loop {
            let mut ready = true;

            for y in 0..=(y_max as usize) {
                for x in 0..=(x_max as usize) {
                    if grid[y][x] > 9 {
                        grid[y][x] = 0;
                        ready = false;
                        let x = x as isize;
                        let y = y as isize;

                        for yy in max(0, y - 1)..=min(y + 1, y_max) {
                            for xx in max(0, x - 1)..=min(x + 1, x_max) {
                                let x = xx as usize;
                                let y = yy as usize;

                                if grid[y][x] > 0 {
                                    grid[y][x] += 1;
                                }
                            }
                        }

                        if step <= 100 {
                            flash_count += 1;
                        }

                    }
                }
            }

            if ready {
                break;
            }
        }

        if grid.iter().flatten().sum::<usize>() == 0 {
            break;
        }

        step += 1;
    }

    println!("Result for puzzle 1: {}", flash_count);
    println!("Result for puzzle 2: {}", step);
}
