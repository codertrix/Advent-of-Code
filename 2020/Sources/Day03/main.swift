// ---------------------------------------------------
//  main.swift
//  Day 3: Toboggan Trajectory
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var map = [[Character]]()

for line in input.split(separator: "\n") {
    map.append(line.map { $0 })
}

let bottom = map.count
let modulo = map[0].count
var treeCount = 0

func numberOfTrees(for angle: (right: Int, down: Int)) -> Int {
    var pos = (x: 0, y: 0)
    var treeCount = 0

    while pos.y < bottom {
        if map[pos.y][pos.x] == "#" {
            treeCount += 1
        }
        pos.x = (pos.x + angle.right) % modulo
        pos.y += angle.down
    }

    return treeCount
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

treeCount = numberOfTrees(for: (3, 1))

print("Result for puzzle 1: \(treeCount)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

let angles = [(1, 1), (5, 1), (7, 1), (1, 2)]

for angle in angles {
    treeCount *= numberOfTrees(for: angle)
}

print("Result for puzzle 2: \(treeCount)")
