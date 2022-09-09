// ---------------------------------------------------
//  Advent of Code 2015
//  Day 5: Doesn't He Have Intern-Elves For This?
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day05;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------------------------");
    println!(" Day 5: Doesn't He Have Intern-Elves For This?");
    println!("-----------------------------------------------");

    day05::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
