// ---------------------------------------------------
//  main.swift
//  Day 4: High-Entropy Passphrases
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let lines = input
    .components(separatedBy: .newlines)
    .map { $0.components(separatedBy: .whitespaces) }

// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

var validCountOne = 0
var validCountTwo = 0

for words in lines {
    var valid = 1;

    for (idx, word) in words.enumerated() {
        if words[(idx + 1)...].contains(word) {
            valid = 0
            break
        }
    }

    validCountOne += valid

    let sortedWords = words.map { $0.sorted() }
    valid = 1

    for (idx, word) in sortedWords.enumerated() {
        if sortedWords[(idx + 1)...].contains(word) {
            valid = 0
            break
        }
    }

    validCountTwo += valid
}

print("Result for puzzle 1: \(validCountOne)")
print("Result for puzzle 2: \(validCountTwo)")
