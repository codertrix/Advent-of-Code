// ---------------------------------------------------
//  Advent of Code 2016
//  Day 16: Dragon Checksum
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = "01111001100111011";

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-------------------------");
    println!(" Day 16: Dragon Checksum");
    println!("-------------------------");

    println!("Result for puzzle 1: {}", puzzle_one_two(272));
    println!("Result for puzzle 2: {}", puzzle_one_two(35651584));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(disc_size: usize) -> String {
    let mut data: Vec<_> = INPUT
        .chars()
        .map(|c| c == '0')
        .collect();

    while data.len() < disc_size {
        data.push(false);

        for idx in (0..data.len() - 1).rev() {
            data.push(!data[idx]);
        }
    }

    data.truncate(disc_size);

    while data.len() & 1 == 0 {
        let mut checksum = Vec::new();
        let mut bits = data.into_iter();

        while let Some(bit1) = bits.next() {
            if let Some(bit2) = bits.next() {
                checksum.push(bit1 == bit2);
            }
        }

        data = checksum;
    }

    data
        .into_iter()
        .map(|bit| if bit { '1' } else { '0' })
        .collect()
}
