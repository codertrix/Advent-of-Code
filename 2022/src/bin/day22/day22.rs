// ---------------------------------------------------
//  Advent of Code 2022
//  Day 22: Monkey Map
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day22.in");
const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;
const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn run() {
    let mut max_x = 0;
    let mut board = vec![vec![]];
    let mut path = Vec::new();

    for line in INPUT.lines() {
        let mut row: Vec<_> = line.chars().collect();

        if row.is_empty() {
            board.push(row);
            continue;
        }

        if row[0] > '.' {
            path = row;
        } else {
            row.insert(0, ' ');
            row.push(' ');
            max_x = max_x.max(row.len());
            board.push(row);
        }
    }

    for row in board.iter_mut() {
        if row.len() < max_x {
            for _ in 0..max_x - row.len() {
                row.push(' ');
            }
        }
    }

    println!("Result for puzzle 1: {}", puzzle_one_two(&board, &path, false));
    println!("Result for puzzle 2: {}", puzzle_one_two(&board, &path, true));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn calculate_cube() -> HashMap<(usize, usize), (usize, usize, usize)> {
    let mut cube_map = HashMap::new();

    for n in 1..=50 {
        cube_map.insert((n, 100), (51, 50 + n, RIGHT));
        cube_map.insert((50, 50 + n), (n, 101, DOWN));
        cube_map.insert((50, n), (1, 151 - n, RIGHT));
        cube_map.insert((0, 151 - n), (51, n, RIGHT));
        cube_map.insert((50 + n, 0), (1, 150 + n, RIGHT));
        cube_map.insert((0, 150 + n), (50 + n, 1, DOWN));
        cube_map.insert((100 + n, 0), (n, 200, UP));
        cube_map.insert((n, 201), (100 + n, 1, DOWN));
        cube_map.insert((151, n), (100, 151 - n, LEFT));
        cube_map.insert((101, 151 - n), (150, n, LEFT));
        cube_map.insert((100 + n, 51), (100, 50 + n, LEFT));
        cube_map.insert((101, 50 + n), (100 + n, 50, UP));
        cube_map.insert((50 + n, 151), (50, 150 + n, LEFT));
        cube_map.insert((51, 150 + n), (50 + n, 150, UP));
    }

    cube_map
}

fn puzzle_one_two(board: &[Vec<char>], path: &[char], is_cube: bool) -> usize {
    let mut col = 1;
    let mut row = 1;
    let mut facing = RIGHT;
    let mut rotation = None;
    let mut step_count = 0;
    let mut path = path.iter();
    let cube_map = calculate_cube();

    while board[row][col] != '.' { col += 1; }

    loop {
        let c = path.next();

        if let Some(step) = c {
            if step.is_ascii_digit() {
                step_count *= 10;
                step_count += step.to_digit(10).unwrap();
            } else {
                rotation = Some(step);
            }
        }

        if c.is_none() || rotation.is_some() {
            for _ in 0..step_count {
                let mut next_col = (col as isize + DIRECTIONS[facing].0) as usize;
                let mut next_row = (row as isize + DIRECTIONS[facing].1) as usize;

                if is_cube {
                    if board[next_row][next_col] == ' ' {
                        let (mapped_col, mapped_row, mapped_facing) = cube_map.get(&(next_col, next_row)).unwrap();

                        if board[*mapped_row][*mapped_col] == '#' {
                            break;
                        }

                        next_col = *mapped_col;
                        next_row = *mapped_row;
                        facing = *mapped_facing;
                    }
                } else if board[next_row][next_col] == ' ' {
                    loop {
                        let prev_col = (next_col as isize - DIRECTIONS[facing].0) as usize;
                        let prev_row = (next_row as isize - DIRECTIONS[facing].1) as usize;

                        if board[prev_row][prev_col] == ' ' {
                            break;
                        }

                        next_col = prev_col;
                        next_row = prev_row;
                    }
                }

                if board[next_row][next_col] == '#' { break; }

                col = next_col;
                row = next_row;
            }

            if c.is_none() { break; }

            match rotation.unwrap() {
                'R' => facing = (facing + 1) % 4,
                'L' => facing = (facing + 3) % 4,
                _ => unreachable!("There is something wrong with the path!")
            }

            step_count = 0;
            rotation = None;
        }
    }

    1000 * row + 4 * col + facing
}
