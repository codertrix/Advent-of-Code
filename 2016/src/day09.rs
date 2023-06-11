// ---------------------------------------------------
//  Advent of Code 2016
//  Day 9: Explosives in Cyberspace
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day09.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------------");
    println!(" Day 9: Explosives in Cyberspace");
    println!("---------------------------------");

    println!("Result for puzzle 1: {}", puzzle_one(INPUT));
    println!("Result for puzzle 2: {}", puzzle_two(INPUT));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


fn decompress(file: &str, full: bool) -> usize {
    let mut decompressed_length = 0;
    let mut num_start = 0;
    let mut length = 0;
    let mut read_marker = false;
    let mut input = file.chars().enumerate();

    while let Some((idx, c)) = input.next() {
        match (c, read_marker) {
            ('(', false) => {
                read_marker = true;
                num_start = idx + 1;
            }
            ('x', true) => {
                length = file[num_start..idx].parse().unwrap();
                num_start = idx + 1;
            }
            (')', true) => {
                let mut end = idx;
                let mut is_compressed = false;
                read_marker = false;

                for _ in 0..length {
                    if let Some((idx, c)) = input.next() {
                        if c == '(' {
                            is_compressed = true;
                        }
                        end = idx;
                    }
                }

                if full && is_compressed {
                    length = decompress(&file[idx + 1..=end], full);
                }

                decompressed_length += length * file[num_start..idx].parse::<usize>().unwrap();
            }
            (_, false) => decompressed_length += 1,
            _ => ()
        }
    }

    decompressed_length
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(file: &str) -> usize {
    decompress(file.trim(), false)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(file: &str) -> usize {
    decompress(file.trim(), true)
}
