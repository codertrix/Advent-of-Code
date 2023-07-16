// ---------------------------------------------------
//  main.swift
//  Day 17: Spinlock
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

let steps = 355


// --------------------
//   MARK: - Puzzle 1
// --------------------

var buffer = [0]
var idx = 0

for value in 1...2017 {
    idx = (idx + steps) % value + 1
    buffer.insert(value, at: idx)
}

print("Result for puzzle 1: \(buffer[(idx + 1) % buffer.count])")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var valueAfterZero = 0

for value in 2018...50_000_000 {
    idx = (idx + steps) % value + 1

    if idx == 1 {
        valueAfterZero = value
    }
}

print("Result for puzzle 2: \(valueAfterZero)")
