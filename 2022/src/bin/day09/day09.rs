// ---------------------------------------------------
//  Advent of Code 2022
//  Day 9: Rope Bridge
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day09.in");

pub fn run() {
    let motions: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let (dir, count) = line.split_once(' ').unwrap();
            (dir, count.parse::<usize>().unwrap())
        })
        .collect();

    let (result_one, result_two) = puzzle_one_two(&motions);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn check_knots(knots: &mut [(isize, isize); 10], direction: (isize, isize)) {
    knots[0].0 += direction.0;
    knots[0].1 += direction.1;

    for idx in 0..9 {
        let dx = knots[idx].0 - knots[idx + 1].0;
        let dy = knots[idx].1 - knots[idx + 1].1;

        match (dx.abs(), dy.abs()) {
            (2, 2) => {
                knots[idx + 1].0 += dx / 2;
                knots[idx + 1].1 += dy / 2;
            }
            (_, 2) => {
                knots[idx + 1].0 += dx;
                knots[idx + 1].1 += dy / 2;
            }
            (2, _) => {
                knots[idx + 1].0 += dx / 2;
                knots[idx + 1].1 += dy;
            }
            _ => break
        }
    }
}

fn puzzle_one_two(motions: &[(&str, usize)]) -> (usize, usize) {
    let mut positions_one = HashSet::new();
    let mut positions_two = HashSet::new();
    let mut knots = [(0, 0); 10];

    for (direction, count) in motions {
        for _ in 0..*count {
            match *direction {
                "R" => check_knots(&mut knots, (1, 0)),
                "L" => check_knots(&mut knots, (-1, 0)),
                "U" => check_knots(&mut knots, (0, -1)),
                _ => check_knots(&mut knots, (0, 1)),
            }

            positions_one.insert(knots[1]);
            positions_two.insert(knots[9]);

        }
    }

    (positions_one.len(), positions_two.len())
}
