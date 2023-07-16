// ---------------------------------------------------
//  main.swift
//  Day 13: Packet Scanners
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var firewall = [Int: (Int, Int)]()

for line in input.components(separatedBy: .newlines) {
    let lineParts = line.components(separatedBy: ": ")
    let range = Int(lineParts[1])!

    firewall[Int(lineParts[0])!] = (range, (range - 1) * 2)
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var serverity = 0

for (depth, (range, cycle)) in firewall {
    if depth % cycle == 0 {
        serverity += depth * range
    }
}

print("Result for puzzle 1: \(serverity)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var delay = 0
var caught = true

outer:
while caught {
    for (depth, (_, cycle)) in firewall {
        if (depth + delay) % cycle == 0 {
            delay += 1
            continue outer
        }
    }

    caught = false
}

print("Result for puzzle 2: \(delay)")
