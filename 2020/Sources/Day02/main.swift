// ---------------------------------------------------
//  main.swift
//  Day 2: Password Philosophy
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

var puzzle1Count = 0
var puzzle2Count = 0

for line in input.split(separator: "\n") {
    let parts = line.split(whereSeparator: { $0 == " " || $0 == "-" || $0 == ":" }).map { String($0) }
    let a = Int(parts[0])!
    let b = Int(parts[1])!
    let character = Character(parts[2])
    let pw = parts[3]

    // Puzzle 1
    let occurrences = pw.filter { $0 == character }.count

    if occurrences >= a && occurrences <= b {
        puzzle1Count += 1
    }

    // Puzzle 2
    let startIdx = pw.startIndex
    let idx1 = pw.index(startIdx, offsetBy: a - 1)
    let idx2 = pw.index(startIdx, offsetBy: b - 1)

    if (pw[idx1] == character) != (pw[idx2] == character) {
        puzzle2Count += 1
    }
}

print("Result for puzzle 1: \(puzzle1Count)")
print("Result for puzzle 2: \(puzzle2Count)")
