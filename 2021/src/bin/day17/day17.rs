// ---------------------------------------------------
//  Advent of Code 2021
//  Day 17: Trick Shot
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

// INPUT: "target area: x=155..182, y=-117..-67";

const MIN_X: isize = 155;
const MAX_X: isize = 182;
const MIN_Y: isize = -117;
const MAX_Y: isize = -67;

pub fn run() {
    let mut max_height = 0;
    let mut hits_count = 0;

    for vx in 1..=MAX_X {
        for vy in MIN_Y..-MIN_Y {
            if let Some(h) = hits_target_area(vx, vy) {
                max_height = max_height.max(h);
                hits_count += 1;
            }
        }
    }

    println!("Result for puzzle 1: {}", max_height);
    println!("Result for puzzle 2: {}", hits_count);
}

fn hits_target_area(mut vx: isize, mut vy: isize) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut height = 0;

    while x <= MAX_X && y >= MIN_Y {
        if x >= MIN_X && y <= MAX_Y {
            return Some(height);
        }

        x += vx;
        y += vy;
        height = height.max(y);

        if vx > 0 { vx -= 1; }
        vy -= 1;
    }

    None
}
