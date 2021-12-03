// ---------------------------------------------------
//  main.swift
//  Day 15: Rambunctious Recitation
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let input = [8, 13, 1, 0, 18, 9]

func spokenNumber(forTurn lastTurn: Int, startingNumbers: [Int]) -> Int {
    var previousTurns = [Int: Int]()

    for idx in 0..<startingNumbers.count - 1 {
        previousTurns[startingNumbers[idx]] = idx
    }

    var numberToConsider = startingNumbers.last!
    var turn = previousTurns.count

    while turn < lastTurn - 1 {
        var nextNumber = 0

        if let previousTurn = previousTurns[numberToConsider] {
            nextNumber = turn - previousTurn
        }

        previousTurns[numberToConsider] = turn
        numberToConsider = nextNumber
        turn += 1
    }

    return numberToConsider
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

print("Result for puzzle 1: \(spokenNumber(forTurn: 2020, startingNumbers: input))")


// --------------------
//   MARK: - Puzzle 2
// --------------------

print("Result for puzzle 2: \(spokenNumber(forTurn: 30000000, startingNumbers: input))")
