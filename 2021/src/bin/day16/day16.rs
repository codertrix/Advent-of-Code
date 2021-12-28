// ---------------------------------------------------
//  Advent of Code 2021
//  Day 16: Packet Decoder
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day16.in");

pub fn run() {
    let mut bits = Vec::new();

    for octed in INPUT.trim().bytes() {
        let octed = if octed > 64 { octed - 55 } else { octed - 48 };

        for mask in [8, 4, 2, 1] {
            bits.push(if octed & mask > 0 { 1 } else { 0 });
        }
    }

    let mut packets = HashMap::new();
    let mut pos = 0;

    while pos < bits.len() - 7 {
        parse_packet(&bits, &mut pos, &mut 0, &mut packets);
    }

    let result_one = packets
        .values()
        .fold(0, |a, packet| a + packet.version);
    let result_two = packets
        .get(&0)
        .unwrap()
        .value;

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}

struct Packet {
    version: usize,
    value: usize,
}

fn parse_packet(bits: &[usize], pos: &mut usize, packet_number: &mut usize, packets: &mut HashMap<usize, Packet>) {
    let version = bits[*pos..*pos + 3].iter().fold(0, |a, b| a * 2 + b);
    let type_id = bits[*pos + 3..*pos + 6].iter().fold(0, |a, b| a * 2 + b);
    let mut value = 0;

    *pos += 6;

    match type_id {
        4 => {
            loop {
                *pos += 5;
                value = value * 16 + bits[*pos - 4..*pos].iter().fold(0, |a, b| a * 2 + b);

                if bits[*pos - 5] == 0 {
                    packets.insert(*packet_number, Packet { version, value });
                    *packet_number += 1;
                    return;
                }
            }
        }
        _ => {
            let mut sub_packets = Vec::new();
            let this_packet_number = *packet_number;
            *packet_number += 1;

            if bits[*pos] == 0 {
                *pos += 16;

                let end_of_sub_packets = *pos + bits[*pos - 15..*pos].iter().fold(0, |a, b| a * 2 + *b);

                while *pos < end_of_sub_packets {
                    sub_packets.push(*packet_number);
                    parse_packet(bits, pos, packet_number, packets);
                    *packet_number += 1;
                }
            } else {
                *pos += 12;

                let sub_packets_count = bits[*pos - 11..*pos].iter().fold(0, |a, b| a * 2 + *b);

                for _ in 0..sub_packets_count {
                    sub_packets.push(*packet_number);
                    parse_packet(bits, pos, packet_number, packets);
                    *packet_number += 1;
                }
            }

            let mut values = sub_packets
                .iter()
                .map(|p| packets.get(p).unwrap().value);

            value = match type_id {
                0 => values.sum(),
                1 => values.product(),
                2 => values.min().expect("Missing sub-packet(s)!"),
                3 => values.max().expect("Missing sub-packet(s)!"),
                _ => {
                    let a = values
                        .next()
                        .expect("Missing sub-packet!");
                    let b = values
                        .next()
                        .expect("Missing sub-packet!");

                    match type_id {
                        5 => if a > b { 1 } else { 0 },
                        6 => if a < b { 1 } else { 0 },
                        7 => if a == b { 1 } else { 0 },
                        _ => panic!("Unkown type id!")
                    }
                }
            };

            packets.insert(this_packet_number, Packet { version, value });
        }
    }
}
