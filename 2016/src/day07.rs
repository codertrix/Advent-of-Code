// ---------------------------------------------------
//  Advent of Code 2016
//  Day 7: Internet Protocol Version 7
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day07.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------------");
    println!(" Day 7: Internet Protocol Version 7");
    println!("------------------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn contains_abba(sequence: &str) -> bool {
    for abba in sequence.chars().collect::<Vec<_>>().windows(4) {
        if abba[0] == abba[3] && abba[1] == abba[2] && abba[0] != abba[1] {
            return true;
        }
    }

    false
}

fn puzzle_one_two() -> (usize, usize) {
    let mut tls_support_count = 0;
    let mut ssl_support_count = 0;

    'outer:
    for line in INPUT.lines() {
        let mut supernet = Vec::new();
        let mut hypernet = Vec::new();

        for (idx, part) in line.split(|c| c == '[' || c == ']').enumerate() {
            if idx % 2 == 0 {
                supernet.push(part);
            } else {
                hypernet.push(part);
            }
        }

        'abba:
        for sequence in supernet.iter() {
            if contains_abba(sequence) {
                for sequence in hypernet.iter() {
                    if contains_abba(sequence) {
                        break 'abba;
                    }
                }

                tls_support_count += 1;
                break;
            }
        }

        for super_sequence in supernet {
            for aba in super_sequence.chars().collect::<Vec<_>>().windows(3) {
                if aba[0] == aba[2] && aba[0] != aba[1] {
                    if let Ok(bab) = String::from_utf8(vec![aba[1] as u8, aba[0] as u8, aba[1] as u8]) {
                        for hyper_sequence in hypernet.iter() {
                            if hyper_sequence.contains(&bab) {
                                ssl_support_count += 1;
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        }
    }

    (tls_support_count, ssl_support_count)
}
