// ---------------------------------------------------
//  main.swift
//  Day 25: Combo Breaker
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let input = """
18356117
5909654
"""

let lines = input.components(separatedBy: .newlines)
let keys = (card: Int(lines[0])!, door: Int(lines[1])!)


// --------------------
//   MARK: - Puzzle 1
// --------------------

var value = 1
var loopSize = 0

while value != keys.card {
    loopSize += 1
    value = (value * 7) % 20201227
}

value = 1

for _ in 1...loopSize {
    value = (value * keys.door) % 20201227
}

print("Result for puzzle 1: \(value)")


// ------------------------------------------
//   There was no second puzzle on this day
// ------------------------------------------
