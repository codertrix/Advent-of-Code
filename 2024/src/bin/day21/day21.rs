// ---------------------------------------------------
//  Advent of Code 2024
//  Day 21: Keypad Conundrum
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("day21.in");
const KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

pub fn run() {
    let codes: Vec<&str> = INPUT.lines().collect();

    println!("Result for puzzle 1: {}", puzzle_one_two(&codes, 2));
    println!("Result for puzzle 2: {}", puzzle_one_two(&codes, 25));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(codes: &[&str], depth: u32) -> usize {
    let keypad = keypad_sequences();
    let mut sum_complexities = 0;
    let mut seen = HashMap::new();

    for &code in codes {
        let mut last_digit = 'A';

        sum_complexities += code.chars().fold(0, |len_a, next_digit| {
            let sequences = keypad.get(&(last_digit, next_digit)).unwrap();
            last_digit = next_digit;

            sequences.iter().fold(usize::MAX, |len_b, next_sequence| {
                let mut last_move = 'A';

                len_b.min(next_sequence.chars().fold(0, |len_c, next_move| {
                    let len = move_robots(last_move, next_move, depth, &mut seen);
                    last_move = next_move;
                    len_c + len
                }))
            }) + len_a
        }) * code[0..3].parse::<usize>().unwrap();
    }

    sum_complexities
}

fn move_robots(a: char, b: char, depth: u32, seen: &mut HashMap<(char, char, u32), usize>) -> usize {
    if let Some(len) = seen.get(&(a, b, depth)) {
        return *len;
    }

    let sequences = dirpad_sequences(a, b);

    if depth == 1 {
        sequences[0].len()
    } else {
        let mut shortest_len = usize::MAX;

        for sequence in sequences {
            let mut out_len = 0;
            let mut last = 'A';

            for key in sequence.chars() {
                out_len += move_robots(last, key, depth - 1, seen);
                last = key;
            }

            if out_len > 0 {
                shortest_len = shortest_len.min(out_len);
            }
        }

        seen.insert((a, b, depth), shortest_len);
        shortest_len
    }
}

fn dirpad_sequences(a: char, b: char) -> [&'static str; 2] {
    match (a, b) {
        ('A', 'A') => ["A", ""],
        ('A', '^') => ["<A", ""],
        ('A', '<') => ["<v<A", "v<<A"],
        ('A', 'v') => ["<vA", "v<A"],
        ('A', '>') => ["vA", ""],
        ('^', 'A') => [">A", ""],
        ('^', '^') => ["A", ""],
        ('^', '<') => ["v<A", ""],
        ('^', 'v') => ["vA", ""],
        ('^', '>') => ["v>A", ">vA"],
        ('<', 'A') => [">^>A", ">>^A"],
        ('<', '^') => [">^A", ""],
        ('<', '<') => ["A", ""],
        ('<', 'v') => [">A", ""],
        ('<', '>') => [">>A", ""],
        ('v', 'A') => ["^>A", ">^A"],
        ('v', '^') => ["^A", ""],
        ('v', '<') => ["<A", ""],
        ('v', 'v') => ["A", ""],
        ('v', '>') => [">A", ""],
        ('>', 'A') => ["^A", ""],
        ('>', '^') => ["^<A", "<^A"],
        ('>', '<') => ["<<A", ""],
        ('>', 'v') => ["<A", ""],
        ('>', '>') => ["A", ""],
        _ => panic!("Ups!"),
    }
}

fn keypad_sequences() -> HashMap<(char, char), Vec<String>> {
    let positions: Vec<_> = [0, 1, 2]
        .into_iter()
        .flat_map(|x| [0, 1, 2, 3].map(|y| (x, y)))
        .filter(|(x, y)| (*x, *y) != (0, 3))
        .collect();
    let mut sequences = HashMap::new();

    for &(ax, ay) in &positions {
        let ka = KEYPAD[ay][ax];

        for &(bx, by) in &positions {
            let kb = KEYPAD[by][bx];

            if ka == kb {
                sequences.insert((ka, kb), vec!["A".to_string()]);
                continue;
            }

            let mut distance = usize::MAX;
            let mut moves = Vec::new();
            let mut queue = VecDeque::new();

            queue.push_back((ax, ay, "".to_string()));

            'w: while let Some((x, y, m)) = queue.pop_front() {
                for (dx, dy, nm) in [(0, -1, "^"), (1, 0, ">"), (0, 1, "v"), (-1, 0, "<")] {
                    let xx = x as i32 + dx;
                    let yy = y as i32 + dy;
                    let m = m.clone() + nm;

                    if xx < 0 || yy < 0 || xx > 2 || yy > 3 || (xx, yy) == (0, 3) {
                        continue;
                    }

                    if (xx as usize, yy as usize) == (bx, by) {
                        if distance < m.len() {
                            break 'w;
                        }

                        distance = m.len();
                        moves.push(m.clone() + "A");
                    }

                    if distance > m.len() {
                        queue.push_back((xx as usize, yy as usize, m));
                    }
                }
            }

            sequences.insert((ka, kb), moves);
        }
    }

    sequences
}
