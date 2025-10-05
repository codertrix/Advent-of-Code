// ---------------------------------------------------
//  Advent of Code 2024
//  Day 24: Crossed Wires
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

type Gates = HashMap<&'static str, (char, &'static str, &'static str)>;
type Wires = HashMap<&'static str, usize>;

const INPUT: &str = include_str!("day24.in");

pub fn run() {
    let mut wires = HashMap::new();
    let mut gates = HashMap::new();
    let mut lines = INPUT.lines();

    for line in lines.by_ref() {
        if let Some((a, b)) = line.split_once(": ") {
            wires.insert(a, b.parse::<usize>().unwrap());
        } else {
            break;
        }
    }

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();

        match parts[1] {
            "AND" => gates.insert(parts[4], ('&', parts[0], parts[2])),
            "OR" => gates.insert(parts[4], ('|', parts[0], parts[2])),
            _ => gates.insert(parts[4], ('^', parts[0], parts[2])),
        };
    }

    println!("Result for puzzle 1: {}", puzzle_one(&wires, &gates));
    println!("Result for puzzle 2: {}", puzzle_two(&gates));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(wires: &Wires, gates: &Gates) -> usize {
    let mut wires = wires.to_owned();
    let mut decimal_output = 0;
    let mut outputs: Vec<_> = gates
        .keys()
        .filter_map(|k| if k.starts_with('z') { Some(*k) } else { None })
        .collect();

    outputs.sort_unstable_by(|a, b| b.cmp(a));

    for output in outputs {
        decimal_output <<= 1;
        decimal_output += resolve_output(output, &mut wires, gates)
    }

    decimal_output
}

fn resolve_output(output: &'static str, wires: &mut Wires, gates: &Gates) -> usize {
    if let Some(value) = wires.get(output) {
        return *value;
    }

    let Some(&(opr, a, b)) = gates.get(output) else {
        return 0;
    };
    let value_a = resolve_output(a, wires, gates);
    let value_b = resolve_output(b, wires, gates);

    let value = match opr {
        '&' => value_a & value_b,
        '|' => value_a | value_b,
        '^' => value_a ^ value_b,
        _ => panic!("Unknown operator!"),
    };

    wires.insert(output, value);
    value
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(gates: &Gates) -> String {
    let mut gates = gates.clone();
    let wires: Vec<&str> = gates.keys().copied().collect();
    let mut swapped_wires = Vec::new();

    for _ in 0..4 {
        let current_progress = progress(&gates);

        'swapping: for a in &wires {
            for b in &wires {
                if a == b {
                    continue;
                }

                let gate_a = *gates.get(a).unwrap();
                let gate_b = *gates.get(b).unwrap();

                gates.entry(a).and_modify(|v| *v = gate_b);
                gates.entry(b).and_modify(|v| *v = gate_a);

                if progress(&gates) > current_progress {
                    swapped_wires.push(a.to_string());
                    swapped_wires.push(b.to_string());
                    break 'swapping;
                }

                gates.entry(a).and_modify(|v| *v = gate_a);
                gates.entry(b).and_modify(|v| *v = gate_b);
            }
        }
    }

    swapped_wires.sort_unstable();
    swapped_wires.join(",")
}

fn progress(gates: &Gates) -> u32 {
    for bit in 0..45 {
        if !verify_output(gates, bit) {
            return bit;
        }
    }

    45
}

fn verify_output(gates: &Gates, bit: u32) -> bool {
    let output = format!("z{:02}", bit);
    let Some((op, a, b)) = gates.get(output.as_str()) else {
        return false;
    };

    if *op != '^' {
        return false;
    }

    if bit == 0 {
        &a[1..] == "00" && &b[1..] == "00"
    } else {
        verify_xor_xy(gates, bit, a) && verify_carry_bit(gates, bit, b)
            || verify_xor_xy(gates, bit, b) && verify_carry_bit(gates, bit, a)
    }
}

fn verify_carry_bit(gates: &Gates, bit: u32, wire: &str) -> bool {
    let Some((op, a, b)) = gates.get(wire) else {
        return false;
    };

    if bit == 1 {
        return *op == '&' && &a[1..] == "00" && &b[1..] == "00";
    }

    if *op != '|' {
        false
    } else {
        verify_prev_carry(gates, bit - 1, a) && verify_recursive_carry(gates, bit - 1, b)
            || verify_prev_carry(gates, bit - 1, b) && verify_recursive_carry(gates, bit - 1, a)
    }
}

fn verify_prev_carry(gates: &Gates, bit: u32, wire: &str) -> bool {
    let input = format!("{:02}", bit);
    let Some((op, a, b)) = gates.get(wire) else {
        return false;
    };

    *op == '&' && a[1..] == input && b[1..] == input
}

fn verify_recursive_carry(gates: &Gates, bit: u32, wire: &str) -> bool {
    let Some((op, a, b)) = gates.get(wire) else {
        return false;
    };

    if *op != '&' {
        false
    } else {
        verify_xor_xy(gates, bit, a) && verify_carry_bit(gates, bit, b)
            || verify_xor_xy(gates, bit, b) && verify_carry_bit(gates, bit, a)
    }
}

fn verify_xor_xy(gates: &Gates, bit: u32, wire: &str) -> bool {
    let input = format!("{:02}", bit);
    let Some((op, a, b)) = gates.get(wire) else {
        return false;
    };

    *op == '^' && a[1..] == input && b[1..] == input
}
