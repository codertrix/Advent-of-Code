// ---------------------------------------------------
//  Advent of Code 2024
//  Day 12: Garden Groups
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("day12.in");

pub fn run() {
    let map: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    let (price_one, price_two) = puzzle_one_two(&map);

    println!("Result for puzzle 1: {}", price_one);
    println!("Result for puzzle 2: {}", price_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(map: &[Vec<char>]) -> (usize, usize) {
    let mut plots = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            plots.insert((x as isize, y as isize), *c);
        }
    }

    let mut price_one = 0;
    let mut price_two = 0;

    for y in 0..map.len() as isize {
        for x in 0..map[0].len() as isize {
            if let Some(plot) = plots.remove(&(x, y)) {
                let mut area = HashSet::new();
                let mut positions = VecDeque::new();

                area.insert((x, y));
                positions.push_back((x, y));

                while let Some((x, y)) = positions.pop_front() {
                    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                        let xx = x + dx;
                        let yy = y + dy;

                        if plots.get(&(xx, yy)) == Some(&plot) {
                            positions.push_back((xx, yy));
                            area.insert((xx, yy));
                            plots.remove(&(xx, yy));
                        }
                    }
                }

                let mut top = HashMap::new();
                let mut right = HashMap::new();
                let mut bottom = HashMap::new();
                let mut left = HashMap::new();

                for (x, y) in &area {
                    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                        if !area.contains(&(x + dx, y + dy)) {
                            match (dx, dy) {
                                (0, -1) => top.entry(y).or_insert(Vec::new()).push(x),
                                (1, 0) => right.entry(x).or_insert(Vec::new()).push(y),
                                (0, 1) => bottom.entry(y).or_insert(Vec::new()).push(x),
                                _ => left.entry(x).or_insert(Vec::new()).push(y),
                            }
                        }
                    }
                }

                let mut perimeter_one = 0;
                let mut perimeter_two = 0;

                for edge in [top, right, bottom, left] {
                    for (_, mut e) in edge {
                        e.sort_unstable();
                        perimeter_one += e.len();
                        perimeter_two += 1;

                        for ee in e.windows(2) {
                            if ee[1] - ee[0] > 1 {
                                perimeter_two += 1;
                            }
                        }
                    }
                }

                price_one += area.len() * perimeter_one;
                price_two += area.len() * perimeter_two;
            }
        }
    }

    (price_one, price_two)
}
