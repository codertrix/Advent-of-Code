// ---------------------------------------------------
//  Advent of Code 2015
//  Day 20: Infinite Elves and Infinite Houses
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day20;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n--------------------------------------------");
    println!(" Day 20: Infinite Elves and Infinite Houses");
    println!("--------------------------------------------");

    day20::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
