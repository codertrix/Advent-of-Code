// ---------------------------------------------------
//  Advent of Code 2015
//  Day 2: I Was Told There Would Be No Math
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day02;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------------------");
    println!(" Day 2: I Was Told There Would Be No Math");
    println!("------------------------------------------");

    day02::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
