// ---------------------------------------------------
//  main.swift
//  Day 12: Rain Risk
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let instructions = input.components(separatedBy: .newlines).map { (String($0.first!), Int($0[$0.index(after: $0.startIndex)...])!) }
let directions = ["N": (0, -1), "S": (0, 1), "E": (1, 0), "W": (-1, 0)]


// --------------------
//   MARK: - Puzzle 1
// --------------------

var position = (x: 0, y: 0)
var facing = (1, 0)

for instruction in instructions {
    switch instruction {
        case ("F", let value):
            position.x += facing.0 * value
            position.y += facing.1 * value
        case ("L", 90), ("R", 270):
            facing = (facing.1, -facing.0)
        case ("R", 90), ("L", 270):
            facing = (-facing.1, facing.0)
        case ("L", 180), ("R", 180):
            facing = (-facing.0, -facing.1)
        case (let dir, let value):
            position.x += directions[dir]!.0 * value
            position.y += directions[dir]!.1 * value
    }
}

print("Result for puzzle 1: \(abs(position.x) + abs(position.y))")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var waypoint = (10, -1)
position = (x: 0, y: 0)

for instruction in instructions {
    switch instruction {
        case ("F", let value):
            position.x += waypoint.0 * value
            position.y += waypoint.1 * value
        case ("L", 90), ("R", 270):
            waypoint = (waypoint.1, -waypoint.0)
        case ("R", 90), ("L", 270):
            waypoint = (-waypoint.1, waypoint.0)
        case ("L", 180), ("R", 180):
            waypoint = ( -waypoint.0, -waypoint.1)
        case (let dir, let value):
            waypoint.0 += directions[dir]!.0 * value
            waypoint.1 += directions[dir]!.1 * value
    }
}

print("Result for puzzle 2: \(abs(position.x) + abs(position.y))")
