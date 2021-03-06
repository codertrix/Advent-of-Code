// ---------------------------------------------------
//  main.swift
//  Day 5: Alchemical Reduction
//
//  Copyright © 2018 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let polymer = input.utf8.map { Int8($0) }


// --------------------
//   MARK: - Puzzle 1
// --------------------

func react(_ polymer: [Int8]) -> Int {
    var polymer = polymer
    var reactionOccurred = false

    repeat {
        var newPolymer = [Int8]()
        var index = 0
        reactionOccurred = false

        while index < polymer.count - 1 {
            if abs(polymer[index] - polymer[index + 1]) == 32 {
                reactionOccurred = true
                index += 2
            } else {
                newPolymer.append(polymer[index])
                index += 1
            }
        }

        if index == polymer.count - 1 {
            newPolymer.append(polymer[index])
        }

        polymer = newPolymer
    } while reactionOccurred

    return polymer.count
}

print("Result for puzzle 1: \(react(polymer))")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var minLength =  polymer.count

for unit in 65...90 {
    let reducedPolymer = polymer.filter { $0 != unit && $0 != unit + 32 }
    let count = react(reducedPolymer)
    if count < minLength { minLength = count }
}

print("Result for puzzle 2: \(minLength)")
