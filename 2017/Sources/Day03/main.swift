// ---------------------------------------------------
//  main.swift
//  Day 3: Spiral Memory
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

let input = 347991


// --------------------
//   MARK: - Puzzle 1
// --------------------

var edgeLength = 1

while edgeLength * edgeLength < input {
    edgeLength += 2
}

var edgeCenter = edgeLength / 2
var centerValue = edgeLength * edgeLength - edgeCenter

while abs(centerValue - input) > edgeCenter {
    centerValue -= edgeLength - 1
}

print("Result for puzzle 1: \(abs(centerValue - input) + edgeCenter)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

struct Pos: Hashable {
    var x: Int
    var y: Int
}

let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]
var grid = [Pos(x: 0, y: 0): 1]
var pos = Pos(x: 0, y: 0)
var result = 0
edgeLength = 0

loop:
while true {
    pos.x += 1
    pos.y += 1
    edgeLength += 2

    for (dx, dy) in directions {
        for _ in 0..<edgeLength {
            var value = 0
            pos.x += dx
            pos.y += dy

            for adjX in -1...1 {
                for adjY in -1...1 {
                    if let storedValue = grid[Pos(x: pos.x + adjX, y: pos.y + adjY)] {
                        value += storedValue
                    }
                }
            }

            if value > input {
                result = value
                break loop
            }

            grid[pos] = value
        }
    }
}

print("Result for puzzle 2: \(result)")
