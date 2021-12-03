// ---------------------------------------------------
//  main.swift
//  Day 8: Handheld Halting
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var program: [(cmd: String, arg: Int)] = input.components(separatedBy: .newlines).map{ Array($0) }.map { (String($0[...2]), Int(String($0[4...]))!) }

func run(_ program: [(cmd: String, arg: Int)]) -> (accumulator: Int, finished: Bool) {
    var accumulator = 0
    var idx = 0
    let terminateIdx = program.count
    var seen = Set<Int>()

    while !seen.contains(idx) && idx != terminateIdx {
        seen.insert(idx)

        switch program[idx].cmd {
            case "acc": accumulator += program[idx].arg; idx += 1
            case "jmp": idx += program[idx].arg
            case "nop": idx += 1
            default: fatalError("Illegal instruction!")
        }
    }

    return (accumulator, idx == terminateIdx)
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var result = run(program)

print("Result for puzzle 1: \(result.accumulator)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

for idx in 0..<program.count {
    let cmd = program[idx].cmd

    switch cmd {
        case "jmp": program[idx].cmd = "nop"
        case "nop": program[idx].cmd = "jmp"
        default: continue
    }

    result = run(program)
    program[idx].cmd = cmd

    if result.finished { break }
}

print("Result for puzzle 2: \(result.accumulator)")
