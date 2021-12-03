// ---------------------------------------------------
//  main.swift
//  Day 1: Report Repair
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let inputCount = input.count


// --------------------
//   MARK: - Puzzle 1
// --------------------

puzzleOne:
for idx1 in 0..<inputCount {
    let value = input[idx1]
    for idx2 in idx1..<inputCount {
        if value + input[idx2] == 2020 {
            print("Result for puzzle 1: \(value * input[idx2])")
            break puzzleOne
        }
    }
}


// --------------------
//   MARK: - Puzzle 2
// --------------------

puzzleTwo:
for idx1 in 0..<inputCount {
    let value1 = input[idx1]
    for idx2 in idx1..<inputCount {
        let value2 = input[idx2]
        for idx3 in idx2..<inputCount {
            if value1 + value2 + input[idx3] == 2020 {
                print("Result for puzzle 2: \(value1 * value2 * input[idx3])")
                break puzzleTwo
            }
        }
    }
}
