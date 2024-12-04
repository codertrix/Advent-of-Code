// ---------------------------------------------------
//  Advent of Code 2024
//  Day 4: Ceres Search
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day04.in");

pub fn run() {
    let mut grid = Vec::new();

    for line in INPUT.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    println!("Result for puzzle 1: {}", puzzle_one(&grid));
    println!("Result for puzzle 2: {}", puzzle_two(&grid));
}


// ----------
//  Puzzle 1
// ----------

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

fn puzzle_one(grid: &[Vec<char>]) -> usize {
    let mut xmas_count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                for (dx, dy) in DIRECTIONS {
                    let mut letter_count = 1;

                    for (n, c) in ['M', 'A', 'S'].into_iter().enumerate() {
                        let xx = dx * (n as isize + 1) + x as isize;
                        let yy = dy * (n as isize + 1) + y as isize;

                        if xx < 0
                            || yy < 0
                            || xx >= grid[0].len() as isize
                            || yy >= grid.len() as isize
                        {
                            continue;
                        }

                        if grid[yy as usize][xx as usize] == c {
                            letter_count += 1;
                        }
                    }

                    if letter_count == 4 {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    xmas_count
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(grid: &[Vec<char>]) -> usize {
    let mut mas_count = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] == 'A' {
                let x_mas: String = [
                    grid[y - 1][x - 1],
                    grid[y - 1][x + 1],
                    grid[y + 1][x - 1],
                    grid[y + 1][x + 1],
                ]
                .iter()
                .collect();

                if x_mas == "MSMS" || x_mas == "SMSM" || x_mas == "MMSS" || x_mas == "SSMM" {
                    mas_count += 1;
                }
            }
        }
    }

    mas_count
}
