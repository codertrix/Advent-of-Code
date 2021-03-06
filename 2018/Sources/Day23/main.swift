// ---------------------------------------------------
//  main.swift
//  Day 23: Experimental Emergency Teleportation
//
//  Copyright © 2018 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let inputLines = input
    .components(separatedBy: .newlines)
    .map { String($0).split(whereSeparator: { "<,>=".contains($0) }) }
let nanobots = inputLines.compactMap { (x: Int($0[1])!, y: Int($0[2])!, z: Int($0[3])!, r: Int($0[5])!) }


// --------------------
//   MARK: - Puzzle 1
// --------------------

guard let strongestNanobot = nanobots.max(by: { $0.r < $1.r }) else { preconditionFailure() }

var nanobotsInRange = nanobots.filter { (nanobot) -> Bool in
    let dx = abs(nanobot.x - strongestNanobot.x)
    let dy = abs(nanobot.y - strongestNanobot.y)
    let dz = abs(nanobot.z - strongestNanobot.z)

    return (dx + dy + dz) <= strongestNanobot.r
}

print("Result for puzzle 1: \(nanobotsInRange.count)")
