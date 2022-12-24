// ---------------------------------------------------
//  Advent of Code 2022
//  Day 23: Unstable Diffusion
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day23;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------");
    println!(" Day 23: Unstable Diffusion");
    println!("----------------------------");

    day23::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
