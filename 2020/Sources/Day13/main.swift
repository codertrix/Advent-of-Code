// ---------------------------------------------------
//  main.swift
//  Day 13: Shuttle Search
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let lines = input.components(separatedBy: .newlines)
let earliest = Int(lines[0])!
let buses = lines[1].split(separator: ",")


// --------------------
//   MARK: - Puzzle 1
// --------------------

let buses1 = buses.filter { $0 != "x" }.map { Int($0)! }
var start = earliest

loop:
while true {
    for bus in buses1 {
        if start % bus == 0 {
            print("Result for puzzle 1: \((start - earliest) * bus)")
            break loop
        }
    }
    start += 1
}


// --------------------
//   MARK: - Puzzle 2
// --------------------

var buses2 = [(Int, Int)]()

for idx in 0..<buses.count {
    if buses[idx] == "x" { continue }
    buses2.append((Int(buses[idx])!, idx))
}

buses2.sort(by: { $0.0 < $1.0 })

var t = 0
var dt = 1

for idx1 in 1..<buses2.count {
    var found = false

    while !found {
        found = true
        for idx2 in 0...idx1 {
            if (t + buses2[idx2].1) % buses2[idx2].0 != 0 {
                found = false
                break
            }
        }

        if found {
            dt = buses2[...idx1].reduce(1, { $0 * $1.0 })
            break
        } else {
            t += dt
        }
    }
}

print("Result for puzzle 2: \(t)")
