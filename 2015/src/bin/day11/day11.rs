// ---------------------------------------------------
//  Advent of Code 2015
//  Day 11: Corporate Policy
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = "hepxcrrq";

pub fn run() {
    let result_one = puzzle_one_two(INPUT);
    let result_two = puzzle_one_two(&result_one);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(old_pw: &str) -> String {
    let bad_chars = [b'i', b'l', b'o'];
    let mut new_pw: Vec::<_> = old_pw
        .bytes()
        .collect();

    'outer:
    loop {
        for c in new_pw.iter_mut().rev() {
            *c += 1;
            if *c <= b'z' {
                break;
            }

            *c = b'a';
        }

        let mut valid = false;
        let mut pairs = [INPUT.len(), 0];

        if bad_chars.contains(&new_pw[0]) || bad_chars.contains(&new_pw[1]) {
            continue 'outer;
        }

        for (idx, ccc) in new_pw.windows(3).enumerate() {
            if bad_chars.contains(&ccc[2]) {
                continue 'outer;
            }

            if pairs[0] == INPUT.len() && ccc[0] == ccc[1] {
                pairs[0] = idx;
            }

            if ccc[1] == ccc[2] {
                pairs[1] = idx + 1;
            }

            if ccc[0] + 1 == ccc[1] && ccc[0] + 2 == ccc[2] {
                valid = true;
            }
        }

        if valid && pairs[0] + 1 < pairs[1] {
            return new_pw
                .iter()
                .map(|c| char::from(*c))
                .collect()
        }
    }
}
