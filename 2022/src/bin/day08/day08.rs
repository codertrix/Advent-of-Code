// ---------------------------------------------------
//  Advent of Code 2022
//  Day 8: Treetop Tree House
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day08.in");

pub fn run() {
    let grid: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line
             .bytes()
             .map(|b| b - b'0')
             .collect())
        .collect();

    let (result_one, result_two) = puzzle_one_two(&grid);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(grid: &[Vec<u8>]) -> (usize, usize) {
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    let directions = [(-1isize, 0), (1, 0), (0, -1isize), (0, 1)];
    let mut visible_trees = HashSet::new();
    let mut highest_score = 0;

    for (y, row) in grid.iter().enumerate().skip(1).take(max_y - 1) {
        for (x, tree) in row.iter().enumerate().skip(1).take(max_x - 1) {
            let mut is_visible = [true; 4];
            let mut scenic_score = 1;

            for (idx, (dx, dy)) in directions.iter().enumerate() {
                let mut xx = x as isize;
                let mut yy = y as isize;
                let mut distance = 0;

                while xx > 0 && xx < max_x as isize && yy > 0 && yy < max_y as isize {
                    xx += *dx;
                    yy += *dy;
                    distance += 1;

                    if grid[yy as usize][xx as usize] >= *tree {
                        is_visible[idx] = false;
                        break;
                    }
                }

                scenic_score *= distance;
                highest_score = highest_score.max(scenic_score);
            }

            if is_visible.contains(&true) {
                visible_trees.insert((x, y));
            }
        }
    }

    (visible_trees.len() + 2 * (max_x + max_y), highest_score)
}
