// ---------------------------------------------------
//  Advent of Code 2023
//  Day 14: Parabolic Reflector Dish
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day14;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------------");
    println!(" Day 14: Parabolic Reflector Dish");
    println!("----------------------------------");

    day14::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
