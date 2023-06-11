// ---------------------------------------------------
//  Advent of Code 2016
//  Day 8: Two-Factor Authentication
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day08.in");
const SCREEN_WDITH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------------");
    println!(" Day 8: Two-Factor Authentication");
    println!("----------------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2:\n\n{}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, String) {
    let mut screen = [[' '; SCREEN_WDITH]; SCREEN_HEIGHT];
    let mut buffer = [' '; SCREEN_WDITH];

    for line in INPUT.lines() {
        let parts: Vec<&str> = line
            .split_whitespace()
            .collect();

        match (parts[0], parts[1]) {
            ("rect", _) => {
                let (x, y) = parts[1].split_once('x').unwrap();

                for row in screen.iter_mut().take(y.parse().unwrap()) {
                    for pixel in row.iter_mut().take(x.parse().unwrap()) {
                        *pixel = '#';
                    }
                }
            }
            ("rotate", "row") => {
                let row: usize = parts[2][2..].parse().unwrap();
                let shift = parts[4].parse().unwrap();

                buffer[..shift].copy_from_slice(&screen[row][SCREEN_WDITH - shift..]);
                screen[row].copy_within(0..SCREEN_WDITH - shift, shift);
                screen[row][..shift].copy_from_slice(&buffer[..shift]);
            }
            ("rotate", "column") => {
                let col: usize = parts[2][2..].parse().unwrap();
                let shift = parts[4].parse().unwrap();

                for idx in 0..shift {
                    buffer[idx] = screen[SCREEN_HEIGHT - shift + idx][col];
                }

                for idx in (0..SCREEN_HEIGHT - shift).rev() {
                    screen[idx + shift][col] = screen[idx][col];
                }

                for idx in 0..shift {
                    screen[idx][col] = buffer[idx];
                }
            }
            _ => panic!("Unknown instruction!")
        }
    }

    let result_two = screen
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    (result_two.chars().fold(0, |a, c| if c == '#' { a + 1 } else { a }), result_two)
}
