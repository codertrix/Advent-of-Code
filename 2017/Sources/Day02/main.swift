// ---------------------------------------------------
//  main.swift
//  Day 2: Corruption Checksum
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let spreadSheet = input
    .components(separatedBy: .newlines)
    .map { $0
        .components(separatedBy: "\t")
        .compactMap { Int($0) }
        .sorted { $1 < $0 }
    }


// --------------------
//   MARK: - Puzzle 1
// --------------------

print("Result for puzzle 1: \(spreadSheet.reduce(0) { $0 + $1[0] - $1[$1.count - 1] })")


// --------------------
//   MARK: - Puzzle 2
// --------------------

let sum = spreadSheet.reduce(0) {
    for idx1 in 0..<$1.count - 1 {
        for idx2 in idx1 + 1..<$1.count {
            if $1[idx1] % $1[idx2] == 0 {
                return $0 + $1[idx1] / $1[idx2]
            }
        }
    }

    return $0
}

print("Result for puzzle 2: \(sum)")
