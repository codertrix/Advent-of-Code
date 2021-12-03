// ---------------------------------------------------
//  main.swift
//  Day 10: Adapter Array
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var outputs = input.components(separatedBy: .newlines).map { Int($0)! }.sorted()
outputs.insert(0, at: 0)
outputs.append(outputs.max()! + 3)


// --------------------
//   MARK: - Puzzle 1
// --------------------

var diff1 = 0
var diff3 = 0

for idx in 1..<outputs.count {
    let diff = outputs[idx] - outputs[idx - 1]

    if diff == 1 { diff1 += 1 }
    if diff == 3 { diff3 += 1 }
}

print("Result for puzzle 1: \(diff1 * diff3)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var cache = [Int: Int]()

func arrangements(for idx: Int = 0) -> Int {
    if idx == outputs.count - 1 { return 1 }
    if let value = cache[idx] { return value }

    var value = 0

    for nextIdx in idx + 1..<outputs.count {
        if outputs[nextIdx] - outputs[idx] <= 3 {
            value += arrangements(for: nextIdx)
        }
    }

    cache[idx] = value

    return value
}

print("Result for puzzle 2: \(arrangements())")
