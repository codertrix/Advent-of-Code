// ---------------------------------------------------
//  Advent of Code 2023
//  Day 10: Pipe Maze
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day10;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------");
    println!(" Day 10: Pipe Maze");
    println!("-------------------");

    day10::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
