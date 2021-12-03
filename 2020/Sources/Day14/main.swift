// ---------------------------------------------------
//  main.swift
//  Day 14: Docking Data
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let program = input.components(separatedBy: .newlines)


// --------------------
//   MARK: - Puzzle 1
// --------------------

var and = 1
var or = 0
var memory = [Int: Int]()

for line in program {
    if line.hasPrefix("mask = ") {
        let mask = Array(line)
        or = 0
        and = 1

        for idx in 7..<mask.count {
            or <<= 1
            and <<= 1
            if mask[idx] == "X" { and += 1 }
            if mask[idx] == "1" { and += 1; or += 1 }
        }
    } else {
        let mem = line.split(whereSeparator: { "[] =".contains($0) })
        memory[Int(mem[1])!] = Int(mem[2])! & and | or
    }

}

print("Result for puzzle 1: \(memory.reduce(0) { $0 + $1.value })")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var masks = [Int]()
memory = [Int: Int]()

for line in program {
    if line.hasPrefix("mask = ") {
        let mask = Array(line)
        var bitValue = 0x1000000000
        masks = [0]
        or = 0
        and = 1

        for idx in 7..<mask.count {
            or <<= 1
            and <<= 1
            bitValue >>= 1
            if mask[idx] == "0" { and += 1 }
            if mask[idx] == "1" { or += 1 }
            if mask[idx] == "X" { masks += masks.map { $0 + bitValue } }
        }
    } else {
        let mem = line.split(whereSeparator: { "[] =".contains($0) })

        for mask in masks {
            memory[Int(mem[1])! & and | or | mask] = Int(mem[2])!
        }
    }

}

print("Result for puzzle 2: \(memory.reduce(0) { $0 + $1.value })")
