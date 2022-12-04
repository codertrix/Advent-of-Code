// ---------------------------------------------------
//  Advent of Code 2022
//  Day 4: Camp Cleanup
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day04.in");

pub fn run() {
    let results = INPUT
        .lines()
        .map(|line| {
            let sections = line.split_once(',').unwrap();
            let s1 = sections.0.split_once('-').unwrap();
            let s2 = sections.1.split_once('-').unwrap();

            let s1min: usize = s1.0.parse().unwrap();
            let s1max: usize = s1.1.parse().unwrap();
            let s2min: usize = s2.0.parse().unwrap();
            let s2max: usize = s2.1.parse().unwrap();

            let includes = s1min <= s2min && s1max >= s2max || s2min <= s1min && s2max >= s1max;
            let overlaps = !(s1max < s2min || s2max < s1min);

            (includes.into(), overlaps.into())
        })
        .fold((0, 0), |a, b: (usize, usize)| (a.0 + b.0, a.1 + b.1));

    println!("Result for puzzle 1: {}", results.0);
    println!("Result for puzzle 2: {}", results.1);
}
