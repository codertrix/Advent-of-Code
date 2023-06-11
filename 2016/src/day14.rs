// ---------------------------------------------------
//  Advent of Code 2016
//  Day 14: One-Time Pad
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::VecDeque;

use md5_rs::Context;

const INPUT: &str = "cuanljph";

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------");
    println!(" Day 14: One-Time Pad");
    println!("----------------------");

    println!("Result for puzzle 1: {}", puzzle_one_two(0));
    println!("Result for puzzle 2: {}", puzzle_one_two(2016));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn md5_hash_for(key: &str, strech_count: usize) -> String {
    let mut hash = key.to_owned();

    for _ in 0..strech_count + 1 {
        let mut context = Context::new();
        
        context.read(hash.as_bytes());
        hash = context
            .finish()
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .collect();
    }

    hash
}

fn puzzle_one_two(stretch_count: usize) -> usize {
    let mut hashes = VecDeque::new();
    let mut key_count = 0;

    for idx in 0..1001 {
        hashes.push_back((idx, md5_hash_for(&format!("{}{}", INPUT, idx), stretch_count)));
    }

    loop {
        let (idx, hash) = hashes.pop_front().unwrap();
        let mut c = 0;

        for cc in hash.as_bytes().windows(3) {
            if cc[0] == cc[1] && cc[1] == cc[2] {
                c = cc[0];
                break;
            }
        }

        if c > 0 {
            'outer:
            for (_, hash) in &hashes {
                for cc in hash.as_bytes().windows(5) {
                    if cc[0] == c && cc[1] == c && cc[2] == c && cc[3] == c && cc[4] == c {
                        key_count += 1;

                        if key_count == 64 {
                            return idx;
                        }

                        break 'outer;
                    }
                }
            }
        }

        hashes.push_back((idx + 1001, md5_hash_for(&format!("{}{}", INPUT, idx + 1001), stretch_count)));
    }
}
