// ---------------------------------------------------
//  Advent of Code 2022
//  Day 11: Monkey in the Middle
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day11;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------");
    println!(" Day 11: Monkey in the Middle");
    println!("------------------------------");

    day11::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
