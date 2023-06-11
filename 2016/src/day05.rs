// ---------------------------------------------------
//  Advent of Code 2016
//  Day 5: How About a Nice Game of Chess?
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use md5_rs::Context;

const INPUT: &str = "cxdnnyjw";

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------------------");
    println!(" Day 5: How About a Nice Game of Chess?");
    println!("----------------------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (String, String) {
    let mut password_one = String::new();
    let mut password_two = [' '; 8];
    let mut index = 0;
    let mut counter = 0;

    while counter < 8 {
        let mut context = Context::new();
        context.read(format!("{}{}",INPUT, index).as_bytes());
        let digest = context.finish();

        if digest[2] < 16 && digest[0] == 0 && digest[1] == 0 {
            if password_one.len() < 8 {
                password_one.push_str(&format!("{:x}", digest[2]));
            }

            if digest[2] < 8 && password_two[digest[2] as usize] == ' ' {
                let b = digest[3] / 16;
                password_two[digest[2] as usize] = (if b <= 9 { b + 48 } else { b + 87 }) as char;
                counter += 1;
            }
        }

        index += 1;
    }

    (password_one, password_two.into_iter().collect())
}
