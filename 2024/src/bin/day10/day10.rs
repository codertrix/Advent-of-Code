// ---------------------------------------------------
//  Advent of Code 2024
//  Day 10: Hoof It
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day10.in");

pub fn run() {
    let map: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (result_one, result_two) = puzzle_one_two(&map);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(map: &[Vec<u32>]) -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                let mut tails = Vec::new();

                trail_tails(map, x, y, *height, &mut tails);
                result_one += tails.iter().collect::<HashSet<&(usize, usize)>>().len();
                result_two += tails.len();
            }
        }
    }

    (result_one, result_two)
}

fn trail_tails(
    map: &[Vec<u32>],
    x: usize,
    y: usize,
    height: u32,
    nine_pos: &mut Vec<(usize, usize)>,
) {
    if height == 9 {
        nine_pos.push((x, y));
        return;
    }

    let height = height + 1;

    if x > 0 && map[y][x - 1] == height {
        trail_tails(map, x - 1, y, height, nine_pos);
    }

    if y > 0 && map[y - 1][x] == height {
        trail_tails(map, x, y - 1, height, nine_pos);
    }

    if x < map[0].len() - 1 && map[y][x + 1] == height {
        trail_tails(map, x + 1, y, height, nine_pos);
    }

    if y < map.len() - 1 && map[y + 1][x] == height {
        trail_tails(map, x, y + 1, height, nine_pos);
    }
}
