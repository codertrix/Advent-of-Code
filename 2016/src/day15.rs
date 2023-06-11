// ---------------------------------------------------
//  Advent of Code 2016
//  Day 15: Timing is Everything
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day15.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------");
    println!(" Day 15: Timing is Everything");
    println!("------------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut time = 0;
    let mut interval = 1;
    let mut disc_nr: usize = 0;

    for line in INPUT.lines() {
        let parts: Vec<_> = line
            .split_whitespace()
            .collect();
        let positions: usize = parts[3].parse().unwrap();
        let pos_at_0: usize = parts[11][..parts[11].len() - 1].parse().unwrap();

        disc_nr = parts[1][1..].parse().unwrap();

        while (time + disc_nr + pos_at_0) % positions != 0 {
            time += interval;
        }

        interval *= positions;
    }

    let time_one = time;
    disc_nr += 1;

    while (time + disc_nr) % 11 != 0 {
        time += interval;
    }

    (time_one, time)
}
