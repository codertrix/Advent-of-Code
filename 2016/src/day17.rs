// ---------------------------------------------------
//  Advent of Code 2016
//  Day 17: Two Steps Forward
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::VecDeque;

use md5_rs::Context;

const INPUT: &str = "pgflpeqp";

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------");
    println!(" Day 17: Two Steps Forward");
    println!("---------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn door_states_for(path: &str) -> [bool; 4] {
    let mut context = Context::new();
    let mut door_states = [false; 4];
    let mut passcode = INPUT.to_owned();

    passcode.push_str(path);
    context.read(passcode.as_bytes());
    context
        .finish()
        .into_iter()
        .enumerate()
        .take(2)
        .for_each(|(idx, x)| {
            door_states[idx * 2] = x >> 4 > 10;
            door_states[idx * 2 + 1] = x & 0x0f > 10;
        });

    door_states
}

fn puzzle_one_two() -> (String, usize) {
    let mut queue = VecDeque::new();
    let mut result_one: Option<String> = None;
    let mut result_two = 0;

    queue.push_back((1, 1, String::new()));

    while let Some((x, y, path)) = queue.pop_front() {
        if x == 4 && y == 4 {
            if result_one.is_none() {
                result_one = Some(path.clone());
            }

            result_two = path.len();
            continue;
        }

        let door_states = door_states_for(&path);

        for (idx, (dx, dy)) in [(0, -1), (0, 1), (-1, 0), (1, 0)].into_iter().enumerate() {
            let (xx, yy) = (x + dx, y + dy);

            if door_states[idx] && xx > 0 && xx < 5 && yy > 0 && yy < 5 {
                let mut next_path = path.clone();

                next_path.push(['U', 'D', 'L', 'R'][idx]);
                queue.push_back((xx, yy, next_path));
            }
        }
    }

    (result_one.unwrap(), result_two)
}
