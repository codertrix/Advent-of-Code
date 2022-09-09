// ---------------------------------------------------
//  Advent of Code 2015
//  Day 9: All in a Single Night
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day09;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------");
    println!(" Day 9: All in a Single Night");
    println!("------------------------------");

    day09::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
