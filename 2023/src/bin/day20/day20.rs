// ---------------------------------------------------
//  Advent of Code 2023
//  Day 20: Pulse Propagation
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("day20.in");

pub fn run() {
    let mut flip_flops = HashMap::new();
    let mut conjunctions = HashMap::new();
    let mut broadcaster = Vec::new();
    let mut all_outputs = Vec::new();

    for line in INPUT.lines() {
        let (module, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<_> = outputs.split(", ").collect();
        all_outputs.push((module, outputs.clone()));

        if let Some(module) = module.strip_prefix('%') {
            flip_flops.insert(
                module,
                FlipFlop {
                    name: module,
                    state: false,
                    outputs,
                },
            );
        } else if let Some(module) = module.strip_prefix('&') {
            conjunctions.insert(
                module,
                Conjunction {
                    name: module,
                    states: HashMap::new(),
                    outputs,
                },
            );
        } else {
            broadcaster = outputs;
        }
    }

    for (source, outputs) in all_outputs {
        let source = if source == "broadcaster" {
            source
        } else {
            &source[1..]
        };

        for output in outputs {
            conjunctions.entry(output).and_modify(|conjunction| {
                conjunction.states.insert(source, false);
            });
        }
    }

    println!( "Result for puzzle 1: {}", puzzle_one(&broadcaster, &flip_flops, &conjunctions));
    println!( "Result for puzzle 2: {}", puzzle_two(&broadcaster, &mut flip_flops, &mut conjunctions));
}

#[derive(Clone)]
struct FlipFlop {
    name: &'static str,
    state: bool,
    outputs: Vec<&'static str>,
}

impl FlipFlop {
    fn process_pulse(&mut self, pulse: bool, queue: &mut VecDeque<(&str, &str, bool)>) {
        if !pulse {
            self.state = !self.state;
            for output in &self.outputs {
                queue.push_back((self.name, output, self.state));
            }
        }
    }
}

#[derive(Clone)]
struct Conjunction {
    name: &'static str,
    states: HashMap<&'static str, bool>,
    outputs: Vec<&'static str>,
}

impl Conjunction {
    fn process_pulse(
        &mut self,
        pulse: bool,
        source: &'static str,
        queue: &mut VecDeque<(&str, &str, bool)>,
    ) {
        self.states.insert(source, pulse);

        let pulse = !self.states.values().fold(true, |a, b| a & b);

        for output in &self.outputs {
            queue.push_back((self.name, output, pulse));
        }
    }
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one( broadcaster: &[&str], flip_flops: &HashMap<&str, FlipFlop>, conjunctions: &HashMap<&str, Conjunction>,) -> usize {
    let mut flip_flops = flip_flops.clone();
    let mut conjunctions = conjunctions.clone();
    let mut queue = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        low_count += 1;

        for destination in broadcaster {
            queue.push_back(("broadcaster", *destination, false));
        }

        while let Some((source, destination, pulse)) = queue.pop_front() {
            if pulse {
                high_count += 1;
            } else {
                low_count += 1;
            }

            flip_flops
                .entry(destination)
                .and_modify(|e| e.process_pulse(pulse, &mut queue));
            conjunctions
                .entry(destination)
                .and_modify(|e| e.process_pulse(pulse, source, &mut queue));
        }
    }

    low_count * high_count
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two<'a>( broadcaster: &[&'a str], flip_flops: &mut HashMap<&'a str, FlipFlop>, conjunctions: &mut HashMap<&'a str, Conjunction>,) -> usize {
    let mut queue = VecDeque::new();
    let mut intervals = Vec::new();
    let mut button_presses = 0;

    if let Some((_, before_rx)) = conjunctions.iter().find(|(_, cj)| cj.outputs[0] == "rx") {
        let monitored_modules: Vec<&str> = before_rx.states.keys().copied().collect();

        while intervals.len() < monitored_modules.len() {
            button_presses += 1;

            for destination in broadcaster {
                queue.push_back(("broadcaster", *destination, false));
            }

            while let Some((source, destination, pulse)) = queue.pop_front() {
                if pulse && monitored_modules.contains(&source) {
                    intervals.push(button_presses);
                }

                flip_flops
                    .entry(destination)
                    .and_modify(|e| e.process_pulse(pulse, &mut queue));
                conjunctions
                    .entry(destination)
                    .and_modify(|e| e.process_pulse(pulse, source, &mut queue));
            }
        }
    }

    intervals.into_iter().product()
}
