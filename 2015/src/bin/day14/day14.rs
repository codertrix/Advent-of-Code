// ---------------------------------------------------
//  Advent of Code 2015
//  Day 14: Reindeer Olympics
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day14.in");
const TIME_LIMIT: usize = 2503;

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}

struct Reindeer {
    speed: usize,
    duration: usize,
    interval: usize,
    distance: usize,
    score: usize
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut max_distance = 0;
    let mut reindeers: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .split_whitespace()
                .collect();
            let speed = parts[3].parse().unwrap_or(0);
            let duration = parts[6].parse().unwrap_or(0);
            let interval = parts[13].parse().unwrap_or(0) + duration;

            Reindeer { speed, duration, interval, distance: 0, score: 0 }
        })
        .collect();

    for time in 0..TIME_LIMIT {
        max_distance = 0;

        for reindeer in reindeers.iter_mut() {
            if time % reindeer.interval < reindeer.duration {
                reindeer.distance += reindeer.speed;
            }

            max_distance = reindeer.distance.max(max_distance);
        }

        for reindeer in reindeers.iter_mut() {
            if reindeer.distance == max_distance {
                reindeer.score += 1;
            }
        }
    }

    (max_distance, reindeers.iter().map(|r| r.score).max().unwrap_or(0))
}
