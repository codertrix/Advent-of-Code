// ---------------------------------------------------
//  Advent of Code 2023
//  Day 15: Lens Library
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day15.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (u32, usize) {
    let mut result_one = 0;
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for s in INPUT.trim().split(',') {
        let mut hash = 0;
        let mut label = "";
        let mut box_nr = 0;
        let mut operation = 0;
        let mut focal_length = 0;

        for (idx, b) in s.bytes().enumerate() {
            if b == b'-' || b == b'=' {
                label = &s[..idx];
                box_nr = hash as usize;
                operation = b;
            }

            if b.is_ascii_digit() {
                focal_length = (b - b'0') as usize;
            }

            hash += b as u32;
            hash *= 17;
            hash &= 255;
        }

        result_one += hash;

        let idx = boxes[box_nr].iter().position(|&lense| lense.0 == label);

        if operation == b'-' {
            if let Some(idx) = idx {
                boxes[box_nr].remove(idx);
            }
        } else if let Some(idx) = idx {
            boxes[box_nr][idx].1 = focal_length;
        } else {
            boxes[box_nr].push((label, focal_length));
        }
    }

    let mut result_two = 0;

    for (box_nr, lenses) in boxes.into_iter().enumerate() {
        for (slot, (_, focal_length)) in lenses.into_iter().enumerate() {
            result_two += (box_nr + 1) * (slot + 1) * focal_length;
        }
    }

    (result_one, result_two)
}
