// ---------------------------------------------------
//  main.swift
//  Day 9: Encoding Error
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let numbers = input.components(separatedBy: .newlines).map { Int($0)! }


// --------------------
//   MARK: - Puzzle 1
// --------------------

var invalidNumber = 0

for idx in 0..<numbers.count - 26 {
    var isValid = false
    invalidNumber = numbers[idx + 25]

    loop:
    for number1 in numbers[idx...idx + 23] {
        for number2 in numbers[idx + 1...idx + 24] {
            if number1 + number2 == invalidNumber {
                isValid = true
                break loop
            }
        }
    }

    if !isValid { break }
}

print("Result for puzzle 1: \(invalidNumber)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var smallest = 0
var largest = 0

for idx1 in 0..<numbers.count {
    var sum = numbers[idx1]
    var idx2 = idx1

    while sum < invalidNumber {
        idx2 += 1
        sum += numbers[idx2]
    }

    if sum == invalidNumber {
        let range = numbers[idx1...idx2]
        smallest = range.min()!
        largest = range.max()!
        break
    }
}

print("Result for puzzle 2: \(smallest + largest)")
