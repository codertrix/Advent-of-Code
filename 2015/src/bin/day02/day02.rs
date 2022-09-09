// ---------------------------------------------------
//  Advent of Code 2015
//  Day 2: I Was Told There Would Be No Math
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day02.in");

pub fn run() {
    let mut paper = 0;
    let mut ribbon = 0;

    for line in INPUT.lines() {
        let mut lwh: Vec<usize> = line
            .split('x')
            .map(|a| a.parse().unwrap())
            .collect();
        lwh.sort_unstable();

        let lw = lwh[0] * lwh[1];
        let wh = lwh[1] * lwh[2];
        let hl = lwh[2] * lwh[0];

        paper += 2 * (lw + wh + hl) + lw.min(wh).min(hl);
        ribbon += 2 * (lwh[0] + lwh[1]) + lw * lwh[2];
    }

    println!("Result for puzzle 1: {}", paper);
    println!("Result for puzzle 2: {}", ribbon);
}
