// ---------------------------------------------------
//  main.swift
//  Day 11: Hex Ed
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

var q = 0
var r = 0
var fewest = 0
var furthest = 0

for step in input.split(separator: ",") {
    switch step {
        case "n":
            r -= 1
        case "ne":
            q += 1
            r -= 1
        case "se":
            q += 1
        case "s":
            r += 1
        case "sw":
            q -= 1
            r += 1
        case "nw":
            q -= 1
        default:
            fatalError()
    }

    fewest = max(abs(q), abs(r))
    furthest = max(furthest, fewest)
}

print("Result for puzzle 1: \(fewest)")
print("Result for puzzle 2: \(furthest)")
