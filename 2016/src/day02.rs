// ---------------------------------------------------
//  Advent of Code 2016
//  Day 2: Bathroom Security
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day02.in");
const KEYPAD_ONE: [[char; 7]; 7] = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', '1', '2', '3', ' ', ' '],
    [' ', ' ', '4', '5', '6', ' ', ' '],
    [' ', ' ', '7', '8', '9', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ']
];
const KEYPAD_TWO: [[char; 7]; 7] = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', '1', ' ', ' ', ' '],
    [' ', ' ', '2', '3', '4', ' ', ' '],
    [' ', '5', '6', '7', '8', '9', ' '],
    [' ', ' ', 'A', 'B', 'C', ' ', ' '],
    [' ', ' ', ' ', 'D', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ']
];

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------");
    println!(" Day 2: Bathroom Security");
    println!("--------------------------");

    println!("Result for puzzle 1: {}", puzzle_one_two(&KEYPAD_ONE, (3, 3)));
    println!("Result for puzzle 2: {}", puzzle_one_two(&KEYPAD_TWO, (1, 3)));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(keypad: &[[char; 7]; 7], start_key: (usize, usize)) -> String {
    let mut x = start_key.0;
    let mut y = start_key.1;
    let mut code = String::new();

    for line in INPUT.lines() {
        for direction in line.chars() {
            match direction {
                'U' => if keypad[y - 1][x] != ' ' { y -= 1 }
                'D' => if keypad[y + 1][x] != ' ' { y += 1 }
                'L' => if keypad[y][x - 1] != ' ' { x -= 1 }
                _ => if keypad[y][x + 1] != ' ' { x += 1 }
            }
        }

        code.push(keypad[y][x]);
    }

    code
}
