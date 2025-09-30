// ---------------------------------------------------
//  Advent of Code 2024
//  Day 20: Race Condition
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day20.in");

pub fn run() {
    let mut start_x = 0;
    let mut start_y = 0;
    let track_map: Vec<Vec<char>> = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_x = x;
                        start_y = y;
                    }
                    c
                })
                .collect()
        })
        .collect();

    let (cheats_one, cheats_two) = puzzle_one_two(&track_map, start_x, start_y);

    println!("Result for puzzle 1: {}", cheats_one);
    println!("Result for puzzle 2: {}", cheats_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(track_map: &[Vec<char>], mut x: usize, mut y: usize) -> (usize, usize) {
    let mut track = Vec::new();
    let mut pico_seconds = vec![vec![-1; track_map[0].len()]; track_map.len()];
    let mut ps = 0;
    let mut cheats_one = 0;
    let mut cheats_two = HashSet::new();

    track.push((x, y));
    pico_seconds[y][x] = ps;

    while track_map[y][x] != 'E' {
        for (xx, yy) in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
            if track_map[yy][xx] != '#' && pico_seconds[yy][xx] == -1 {
                x = xx;
                y = yy;
                ps += 1;
                pico_seconds[y][x] = ps;
                track.push((x, y));
                break;
            }
        }
    }

    for (idx, (x1, y1)) in track.iter().enumerate() {
        for (x2, y2) in track.iter().skip(idx) {
            let distance = (*x2 as i32 - *x1 as i32).abs() + (*y2 as i32 - *y1 as i32).abs();

            if distance <= 20 && pico_seconds[*y2][*x2] - pico_seconds[*y1][*x1] - distance >= 100 {
                if distance == 2 {
                    cheats_one += 1;
                }

                cheats_two.insert((*x1, *y1, *x2, *y2));
            }
        }
    }

    (cheats_one, cheats_two.len())
}
