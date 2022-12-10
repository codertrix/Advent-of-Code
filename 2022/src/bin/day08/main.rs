// ---------------------------------------------------
//  Advent of Code 2022
//  Day 8: Treetop Tree House
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day08;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------");
    println!(" Day 8: Treetop Tree House");
    println!("---------------------------");

    day08::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
