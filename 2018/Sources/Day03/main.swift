// ---------------------------------------------------
//  main.swift
//  Day 3: No Matter How You Slice It
//
//  Copyright © 2018 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let inputValues = input
    .components(separatedBy: .newlines)
    .compactMap {
        $0
            .split(whereSeparator: { "# ,:x".contains($0) })
            .compactMap { Int($0) }
    }


// --------------------
//   MARK: - Puzzle 1
// --------------------

var fabric = Array(repeating: Array(repeating: 0, count: 1000), count: 1000)
var overlappingClaims = 0

inputValues.forEach { (claim) in
    for x in claim[1]..<claim[1] + claim[3] {
        for y in claim[2]..<claim[2] + claim[4] {
            fabric[x][y] += 1
            if fabric[x][y] == 2 { overlappingClaims += 1 }
        }
    }
}

print("Result for puzzle 1: \(overlappingClaims)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var claimID = 0

for claim in inputValues {
    var square = 0
    for x in claim[1]..<claim[1] + claim[3] {
        for y in claim[2]..<claim[2] + claim[4] {
            square += fabric[x][y]
        }
    }

    if square == claim[3] * claim[4] {
        claimID = claim[0]
        break
    }
}

print("Result for puzzle 2: \(claimID)")
