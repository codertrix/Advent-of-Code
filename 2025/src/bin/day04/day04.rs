// ---------------------------------------------------
//  Advent of Code 2025
//  Day 4: Printing Department
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day04.in");

pub fn run() {
    let mut diagram: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (result_one, result_two) = puzzle_one_two(&mut diagram);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(diagram: &mut [Vec<char>]) -> (usize, usize) {
    let len_x = diagram[0].len() as isize;
    let len_y = diagram.len() as isize;
    let mut rolls_one = 0;
    let mut rolls_two = 0;
    let mut to_remove = Vec::new();

    loop {
        to_remove.clear();

        for (y, row) in diagram.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if diagram[y][x] == '@' {
                    let xx = x as isize;
                    let yy = y as isize;
                    let mut count = 0;

                    for (dx, dy) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                        let x = xx + dx;
                        let y = yy + dy;

                        if x < 0 || x == len_x || y < 0 || y == len_y {
                            continue;
                        }

                        if diagram[y as usize][x as usize] == '@' {
                            count += 1;
                        }
                    }

                    if count < 4 {
                        rolls_two += 1;
                        to_remove.push((x, y));
                    }
                }
            }
        }

        for (x, y) in &to_remove {
            diagram[*y][*x] = '.';
        }

        if rolls_one == 0 {
            rolls_one = rolls_two;
        }

        if to_remove.is_empty() {
            break;
        }
    }

    (rolls_one, rolls_two)
}
