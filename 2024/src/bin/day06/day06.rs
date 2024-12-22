// ---------------------------------------------------
//  Advent of Code 2024
//  Day 6: Guard Gallivant
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day06.in");

pub fn run() {
    let mut area: Vec<Vec<char>> = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for (yy, line) in INPUT.lines().enumerate() {
        area.push(
            line.chars()
                .enumerate()
                .map(|(xx, c)| {
                    if c == '^' {
                        x = xx as isize;
                        y = yy as isize;
                    }
                    c
                })
                .collect(),
        );
    }

    let (positions_one, positions_two) = puzzle_one_two(&area, x, y);

    println!("Result for puzzle 1: {}", positions_one);
    println!("Result for puzzle 1: {}", positions_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(area: &[Vec<char>], mut x: isize, mut y: isize) -> (usize, usize) {
    let start = (x, y);
    let x_max = area[0].len() as isize - 1;
    let y_max = area.len() as isize - 1;
    let (mut dx, mut dy) = (0, -1);
    let mut seen = HashSet::new();
    let mut obstructions = HashSet::new();

    seen.insert((x, y, dx, dy));

    while x > 0 && x < x_max && y > 0 && y < y_max {
        if area[(y + dy) as usize][(x + dx) as usize] == '#' {
            (dx, dy) = (-dy, dx);
        } else {
            if is_loop(area, start.0, start.1, x + dx, y + dy) {
                obstructions.insert((x + dx, y + dy));
            }

            x += dx;
            y += dy;
            seen.insert((x, y, dx, dy));
        }
    }

    obstructions.remove(&start);

    (
        seen.iter()
            .map(|pos| (pos.0, pos.1))
            .collect::<HashSet<_>>()
            .len(),
        obstructions.len(),
    )
}

fn is_loop(area: &[Vec<char>], mut x: isize, mut y: isize, ox: isize, oy: isize) -> bool {
    let x_max = area[0].len() as isize - 1;
    let y_max = area.len() as isize - 1;
    let (mut dx, mut dy) = (0, -1);
    let mut seen = HashSet::new();

    while x > 0 && x < x_max && y > 0 && y < y_max {
        seen.insert((x, y, dx, dy));

        if area[(y + dy) as usize][(x + dx) as usize] == '#' || (x + dx, y + dy) == (ox, oy) {
            (dx, dy) = (-dy, dx);
        } else {
            x += dx;
            y += dy;
        }

        if seen.contains(&(x, y, dx, dy)) {
            return true;
        }
    }

    false
}
