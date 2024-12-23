// ---------------------------------------------------
//  Advent of Code 2024
//  Day 13: Claw Contraption
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day13.in");

pub fn run() {
    let mut machines = Vec::new();
    let mut a_x = 0;
    let mut a_y = 0;
    let mut b_x = 0;
    let mut b_y = 0;
    let mut prize_x;
    let mut prize_y;

    for line in INPUT.lines() {
        let Some((prefix, values)) = line.split_once(": ") else {
            continue;
        };

        let (x, y) = values.split_once(", ").unwrap();

        match prefix {
            "Button A" => {
                a_x = x[2..].parse().unwrap();
                a_y = y[2..].parse().unwrap();
            }
            "Button B" => {
                b_x = x[2..].parse().unwrap();
                b_y = y[2..].parse().unwrap();
            }
            _ => {
                prize_x = x[2..].parse().unwrap();
                prize_y = y[2..].parse().unwrap();
                machines.push(ClawMachine {
                    a_x,
                    a_y,
                    b_x,
                    b_y,
                    prize_x,
                    prize_y,
                });
            }
        }
    }

    println!("Result for puzzle 1: {}", puzzle_one_two(&machines, 0));
    println!(
        "Result for puzzle 2: {}",
        puzzle_one_two(&machines, 10_000_000_000_000)
    );
}

struct ClawMachine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(machines: &[ClawMachine], offset: i64) -> i64 {
    machines.iter().fold(0, |tokens, m| {
        let prize_x = m.prize_x + offset;
        let prize_y = m.prize_y + offset;

        let b = (prize_x * m.a_y - prize_y * m.a_x) / (m.b_x * m.a_y - m.b_y * m.a_x);
        let a = (prize_x - b * m.b_x) / m.a_x;

        if m.a_x * a + m.b_x * b == prize_x && m.a_y * a + m.b_y * b == prize_y {
            tokens + a * 3 + b
        } else {
            tokens
        }
    })
}
