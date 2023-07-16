// ---------------------------------------------------
//  main.swift
//  Day 19: A Series of Tubes
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let diagram = input
    .components(separatedBy: .newlines)
    .map { Array($0) }


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

var x = diagram[0].firstIndex(of: "|")!
var y = 0
var dx = 0
var dy = 1
var letters = ""
var notAtEnd = true
var steps = 0

while notAtEnd {
    steps += 1
    x += dx
    y += dy

    switch diagram[y][x] {
        case "+":
            if dx == 0 {
                dx = diagram[y][x - 1] == " " ? 1 : -1
                dy = 0
            } else {
                dx = 0
                dy = diagram[y - 1][x] == " " ? 1 : -1
            }
        case "A"..."Z":
            letters.append(diagram[y][x])
        case " ":
            notAtEnd = false
        default:
            continue
    }
}

print("Result for puzzle 1: \(letters)")
print("Result for puzzle 2: \(steps)")
