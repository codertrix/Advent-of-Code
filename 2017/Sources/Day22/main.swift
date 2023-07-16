// ---------------------------------------------------
//  main.swift
//  Day 22: Sporifica Virus
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var startGrid = [Point: NodeState]()
let lines = input.components(separatedBy: .newlines)
let startRowCount = lines.count
let startColCount = lines[0].count

for (row, line) in lines.enumerated() {
    for (col, c) in line.enumerated() {
        if c == "#" {
            startGrid[Point(x: col, y: row)] = .infected
        }
    }
}

enum NodeState {
    case flagged, infected, weakened
}

struct Point: Hashable {
    var x, y: Int
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var grid = startGrid
var direction = (x: 0, y: -1)
var virusCarrier = Point(x: startColCount / 2, y: startRowCount / 2)
var infectionCount = 0

for _ in 1...10_000 {
    if grid[virusCarrier] != nil {
        direction = (-direction.y, direction.x)
        grid[virusCarrier] = nil
    } else {
        direction = (direction.y, -direction.x)
        grid[virusCarrier] = .infected
        infectionCount += 1
    }

    virusCarrier.x += direction.x
    virusCarrier.y += direction.y
}

print("Result for puzzle 1: \(infectionCount)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

grid = startGrid
direction = (x: 0, y: -1)
virusCarrier = Point(x: startColCount / 2, y: startRowCount / 2)
infectionCount = 0

for _ in 1...10_000_000 {
    switch grid[virusCarrier] {
        case .flagged:
            direction = (-direction.x, -direction.y)
            grid[virusCarrier] = nil
        case .infected:
            direction = (-direction.y, direction.x)
            grid[virusCarrier] = .flagged
        case .weakened:
            grid[virusCarrier] = .infected
            infectionCount += 1
        default:
            direction = (direction.y, -direction.x)
            grid[virusCarrier] = .weakened
    }

    virusCarrier.x += direction.x
    virusCarrier.y += direction.y
}

print("Result for puzzle 2: \(infectionCount)")
