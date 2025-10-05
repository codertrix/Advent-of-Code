// ---------------------------------------------------
//  Advent of Code 2024
//  Day 14: Restroom Redoubt
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day14.in");
const TILES_WIDTH: i32 = 101;
const TILES_HEIGHT: i32 = 103;

pub fn run() {
    let robots: Vec<Robot> = INPUT
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            let (px, py) = p[2..].split_once(',').unwrap();
            let (vx, vy) = v[2..].split_once(',').unwrap();

            Robot {
                p: (px.parse().unwrap(), py.parse().unwrap()),
                v: (vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&robots, 100));
    println!("Result for puzzle 2: {}", puzzle_two(&robots));
}

struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(robots: &[Robot], seconds: i32) -> usize {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in robots {
        let mut x = (robot.p.0 + robot.v.0 * seconds) % TILES_WIDTH;
        let mut y = (robot.p.1 + robot.v.1 * seconds) % TILES_HEIGHT;

        if x < 0 {
            x += TILES_WIDTH;
        }

        if y < 0 {
            y += TILES_HEIGHT;
        }

        if x < TILES_WIDTH / 2 && y < TILES_HEIGHT / 2 {
            q1 += 1;
        }

        if x > TILES_WIDTH / 2 && y < TILES_HEIGHT / 2 {
            q2 += 1;
        }

        if x < TILES_WIDTH / 2 && y > TILES_HEIGHT / 2 {
            q3 += 1;
        }

        if x > TILES_WIDTH / 2 && y > TILES_HEIGHT / 2 {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(robots: &[Robot]) -> i32 {
    let mut count = 1;
    let mut n = 1;

    while count + n + 2 <= robots.len() {
        n += 2;
        count += n;
    }

    let tree_height = n.div_ceil(2);

    for seconds in 1.. {
        let mut aligned_robots = HashMap::new();

        for robot in robots {
            let mut x = (robot.p.0 + robot.v.0 * seconds) % TILES_WIDTH;
            let mut y = (robot.p.1 + robot.v.1 * seconds) % TILES_HEIGHT;

            if x < 0 {
                x += TILES_WIDTH;
            }

            if y < 0 {
                y += TILES_HEIGHT;
            }

            aligned_robots.entry(x).or_insert(Vec::new()).push(y);
        }

        for (_, mut y) in aligned_robots {
            if y.len() >= tree_height {
                let mut n = 1;

                y.sort_unstable();

                for yy in y.windows(2) {
                    if yy[1] - yy[0] == 1 {
                        n += 1;

                        if n == tree_height {
                            return seconds;
                        }
                    } else {
                        n = 1;
                    }
                }
            }
        }
    }

    0
}
