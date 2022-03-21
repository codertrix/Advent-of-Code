// ---------------------------------------------------
//  Advent of Code 2021
//  Day 20: Trench Map
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day20;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------");
    println!(" Day 20: Trench Map");
    println!("--------------------");

    day20::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
