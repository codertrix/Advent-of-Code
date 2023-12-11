// ---------------------------------------------------
//  Advent of Code 2023
//  Day 10: Pipe Maze
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day10.in");

pub fn run() {
    let mut start = (0, 0);
    let mut grid: Vec<Vec<char>> = INPUT
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = (row, col);
                    }

                    c
                })
                .collect()
        })
        .collect();

    let (result_one, result_two) = puzzle_one_two(&mut grid, start);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(grid: &mut [Vec<char>], start: (usize, usize)) -> (usize, usize) {
    let (mut row, mut col) = start;
    let mut pipe_tiles = HashSet::new();
    let mut start_tile = 0;

    if row > 0 && "|7F".contains(grid[row - 1][col]) {
        start_tile += 1;
    }
    if col > 0 && "-LF".contains(grid[row][col - 1]) {
        start_tile += 2;
    }
    if row < grid.len() - 1 && "|JL".contains(grid[row + 1][col]) {
        start_tile += 4;
    }
    if col < grid[0].len() - 1 && "-J7".contains(grid[row][col + 1]) {
        start_tile += 8;
    }

    grid[row][col] = "   J |7  L- F".chars().nth(start_tile).unwrap();
    let mut last_corner = 'S';
    let mut direction = 'u';

    loop {
        let tile = grid[row][col];

        pipe_tiles.insert((row, col));

        if tile == '-' {
            match last_corner {
                'L' | 'F' => col += 1,
                'J' | '7' => col -= 1,
                _ => (),
            }

            continue;
        }

        if tile == '|' {
            if direction == 'u' {
                row -= 1;
            } else {
                row += 1;
            }

            continue;
        }

        match (last_corner, tile, direction) {
            ('S', '|', _) => row -= 1,
            ('S', '-', _) => col += 1,
            ('S', 'L', _) => row -= 1,
            ('S', 'J', _) => row -= 1,
            ('S', '7', _) => {
                direction = 'd';
                row += 1;
            }
            ('S', 'F', _) => col += 1,
            ('J', 'L', _) => {
                direction = 'u';
                row -= 1;
            }
            ('7', 'L', 'u') => row -= 1,
            ('7', 'L', 'd') => col += 1,
            ('F', 'L', _) => col += 1,
            ('L', 'J', _) => {
                direction = 'u';
                row -= 1;
            }
            ('7', 'J', _) => col -= 1,
            ('F', 'J', 'u') => row -= 1,
            ('F', 'J', 'd') => col -= 1,
            ('L', '7', 'u') => col -= 1,
            ('L', '7', 'd') => row += 1,
            ('J', '7', _) => col -= 1,
            ('F', '7', _) => {
                direction = 'd';
                row += 1;
            }
            ('L', 'F', _) => col += 1,
            ('J', 'F', 'u') => col += 1,
            ('J', 'F', 'd') => row += 1,
            ('7', 'F', _) => {
                direction = 'd';
                row += 1;
            }
            _ => {}
        }

        if (row, col) != start {
            last_corner = tile;
            continue;
        }

        break;
    }

    let mut result_two = 0;

    for (idx_row, row) in grid.iter().enumerate() {
        let mut counter = 0;

        for (col, c) in row.iter().enumerate() {
            if pipe_tiles.contains(&(idx_row, col)) {
                if *c != '-' && *c == 'F' || *c == '|' || *c == '7' {
                    counter += 1;
                }
            } else if counter & 1 == 1 {
                result_two += 1;
            }
        }
    }

    (pipe_tiles.len() / 2, result_two)
}
