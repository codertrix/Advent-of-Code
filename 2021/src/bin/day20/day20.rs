// ---------------------------------------------------
//  Advent of Code 2021
//  Day 20: Trench Map
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day20.in");

pub fn run() {
    let mut lines = INPUT.lines();
    let algorithm: Vec<char> = lines
        .next()
        .unwrap()
        .chars()
        .collect();

    lines.next();

    let input_image: Vec<&str> = lines.collect();
    let mut image = HashSet::new();

    for (y, line) in input_image.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((x as isize, y as isize));
            }
        }
    }

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = input_image[0].len() as isize;
    let mut max_y = input_image.len() as isize;
    let mut result_one = 0;

    for step in 0..50 {
        let mut enhanced_image = HashSet::new();

        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let mut idx = 0;

                for yy in y - 1..=y + 1 {
                    for xx in x - 1..=x + 1 {
                        idx <<= 1;
                        if xx < min_x || xx > max_x || yy < min_y || yy > max_y {
                            idx += step % 2;
                        } else {
                            idx += if image.contains(&(xx, yy)) { 1 } else { 0 }
                        }
                    }
                }

                if algorithm[idx] == '#' {
                    enhanced_image.insert((x, y));
                }
            }
        }

        image = enhanced_image;
        min_x -= 1;
        max_x += 1;
        min_y -= 1;
        max_y += 1;

        if step == 1 {
            result_one = image.len();
        }
    }

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", image.len());
}
