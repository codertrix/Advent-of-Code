// ---------------------------------------------------
//  Advent of Code 2021
//  Day 16: Packet Decoder
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod day16;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------");
    println!(" Day 16: Packet Decoder");
    println!("------------------------");

    day16::run();

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}
