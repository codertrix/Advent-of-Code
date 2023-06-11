// ---------------------------------------------------
//  Advent of Code 2016
//  Day 21: Scrambled Letters and Hash
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::VecDeque;

const INPUT: &str = include_str!("day21.in");
const PASSWORD: &str = "abcdefgh";
const SCRAMBLED: &str = "fbgdceah";

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------------");
    println!(" Day 21: Scrambled Letters and Hash");
    println!("------------------------------------");

    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}

fn usize(s: &str) -> usize {
    s.parse().unwrap()
}

fn move_positions(scrambled: &mut VecDeque<char>, x: usize, y: usize) {
    let letter = scrambled.remove(x).unwrap();

    scrambled.insert(y, letter)
}

fn reverse(scrambled: &mut VecDeque<char>, x: usize, y: usize) {
    let mut x = x;
    let mut y = y;

    while x < y {
        scrambled.swap(x, y);
        x += 1;
        y -= 1;
    }
}

fn rotate_based(scrambled: &mut VecDeque<char>, letter: &str, reverse: bool) {
    let left = [1, 1, 6, 2, 7, 3, 0, 4];
    let right = [1, 2, 3, 4, 6, 7, 0, 1];
    let c = letter.chars().next().unwrap();
    let idx = scrambled.iter().position(|cc| *cc == c).unwrap();

    if reverse {
        scrambled.rotate_left(left[idx]);
    } else {
        scrambled.rotate_right(right[idx]);
    }
}

fn swap_letter(scrambled: &mut VecDeque<char>, letter_x: &str, letter_y: &str) {
    let cx = letter_x.chars().next().unwrap();
    let cy = letter_y.chars().next().unwrap();
    let idx_x = scrambled.iter().position(|c| *c == cx).unwrap();
    let idx_y = scrambled.iter().position(|c| *c == cy).unwrap();

    scrambled.swap(idx_x, idx_y);
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> String {
    let mut scrambled: VecDeque<_> = PASSWORD
        .chars()
        .collect();

    for line in INPUT.lines() {
        let parts: Vec<_> = line
            .split_whitespace()
            .collect();

        match parts[0] {
            "swap" => match parts[1] {
                "position" => scrambled.swap(usize(parts[2]), usize(parts[5])),
                "letter" => swap_letter(&mut scrambled, parts[2], parts[5]),
                _ => panic!("Wrong operation!")
            }
            "rotate" => match parts[1] {
                "left" => scrambled.rotate_left(usize(parts[2])),
                "right" => scrambled.rotate_right(usize(parts[2])),
                "based" => rotate_based(&mut scrambled, parts[6], false),
                _ => panic!("Wrong operation!")
            }
            "reverse" => reverse(&mut scrambled, usize(parts[2]), usize(parts[4])),
            "move" => move_positions(&mut scrambled, usize(parts[2]), usize(parts[5])),
            _ => panic!("Wrong operation!")
        }
    }

    scrambled
        .into_iter()
        .collect()
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two() -> String {
    let mut password: VecDeque<_> = SCRAMBLED
        .chars()
        .collect();

    for line in INPUT.lines().rev() {
        let parts: Vec<_> = line
            .split_whitespace()
            .collect();

        match parts[0] {
            "swap" => match parts[1] {
                "position" => password.swap(usize(parts[2]), usize(parts[5])),
                "letter" => swap_letter(&mut password, parts[2], parts[5]),
                _ => panic!("Wrong operation!")
            }
            "rotate" => match parts[1] {
                "left" => password.rotate_right(usize(parts[2])),
                "right" => password.rotate_left(usize(parts[2])),
                "based" => rotate_based(&mut password, parts[6], true),
                _ => panic!("Wrong operation!")
            }
            "reverse" => reverse(&mut password, usize(parts[2]), usize(parts[4])),
            "move" => move_positions(&mut password, usize(parts[5]), usize(parts[2])),
            _ => panic!("Wrong operation!")
        }
    }

    password
        .into_iter()
        .collect()
}
