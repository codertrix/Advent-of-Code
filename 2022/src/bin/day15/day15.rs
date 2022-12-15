// ---------------------------------------------------
//  Advent of Code 2022
//  Day 15: Beacon Exclusion Zone
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day15.in");
const ROW: isize = 2000000;
const MAX_XY: isize = 4000000;

pub fn run() {
    let mut sb_pairs: Vec<((isize, isize), isize)> = Vec::new();

    for line in INPUT.lines() {
        let mut numbers = Vec::new();
        let mut number = 0;
        let mut sign = 1;

        for c in line.chars() {
            match c {
                '0'..='9' => number = number * 10 + c.to_digit(10).unwrap(),
                '-' => sign = -1,
                ',' | ':' => {
                    numbers.push(sign * number as isize);
                    number = 0;
                    sign = 1;
                }
                _ => ()
            }
        }

        numbers.push(sign * number as isize);
        sb_pairs.push(((numbers[0], numbers[1]), (numbers[0] - numbers[2]).abs() + (numbers[1] - numbers[3]).abs()));
        numbers.clear();
    }

    let (result_one, result_two) = puzzle_one_two(&sb_pairs);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(sb_pairs: &[((isize, isize), isize)]) -> (isize, isize) {
    let mut result_one = None;
    let mut result_two = None;

    for y in 0..=MAX_XY {
        let mut ranges = Vec::new();
        let mut x = 0;

        for (sensor, distance) in sb_pairs {
            let dx = distance - (sensor.1 - y).abs();

            if dx < 0 {
                continue;
            }

            ranges.push(sensor.0 - dx..=sensor.0 + dx);
        }

        ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()));

        for range in &ranges {
            if x < *range.start() {
                let result = (x + 1) * 4000000 + y;

                if let Some(result_one) = result_one {
                    return (result_one, result);
                }

                result_two = Some(result);
            }

            x = x.max(*range.end());
        }

        if y == ROW {
            let result = ranges
                .iter()
                .map(|r| r.end())
                .max()
                .unwrap() - ranges[0].start();

            if let Some(result_two) = result_two {
                return (result, result_two);
            }

            result_one = Some(result);
        }
    }

    unreachable!("Distress beacon not found!");
}
