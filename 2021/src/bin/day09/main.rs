// ---------------------------------------------------
//  Advent of Code 2021
//  Day 9: Smoke Basin
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day09;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------");
    println!(" Day 9: Smoke Basin");
    println!("--------------------");

    day09::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
