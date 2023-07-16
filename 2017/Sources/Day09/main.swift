// ---------------------------------------------------
//  main.swift
//  Day 9: Stream Processing
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

var totalScore = 0
var groupScore = 0
var garbageCount = 0
var garbage = false
var skip = false

for c in input {
    if skip {
        skip = false
        continue
    }

    if garbage {
        switch c {
            case ">":
                garbage = false
            case "!":
                skip = true
            default:
                garbageCount += 1
                continue
        }
    }

    switch c {
        case "{":
            groupScore += 1
        case "}":
            totalScore += groupScore
            groupScore -= 1
        case "<":
            garbage = true
        case "!":
            skip = true
        default:
            continue
    }
}

print("Result for puzzle 1: \(totalScore)")
print("Result for puzzle 2: \(garbageCount)")
