// ---------------------------------------------------
//  main.swift
//  Day 1: Chronal Calibration
//
//  Copyright © 2018 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let inputValues = input
    .components(separatedBy: .newlines)
    .compactMap { Int($0) }


// --------------------
//   MARK: - Puzzle 1
// --------------------

print("Result for puzzle 1: \(inputValues.reduce(0) { $0 + $1 })")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var frequencies: Set = [0]
var frequency = 0

endlessLoop: while true {
    for value in inputValues {
        frequency += value
        if frequencies.contains(frequency) {
            break endlessLoop
        }
        frequencies.insert(frequency)
    }
}

print("Result for puzzle 2: \(frequency)")
