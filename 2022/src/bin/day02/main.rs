// ---------------------------------------------------
//  Advent of Code 2022
//  Day 2: Rock Paper Scissors
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day02;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------");
    println!(" Day 2: Rock Paper Scissors");
    println!("----------------------------");

    day02::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
