// ---------------------------------------------------
//  Advent of Code 2022
//  Day 24: Blizzard Basin
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day24;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------");
    println!(" Day 24: Blizzard Basin");
    println!("------------------------");

    day24::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
