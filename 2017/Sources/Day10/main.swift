// ---------------------------------------------------
//  main.swift
//  Day 10: Knot Hash
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let input = "106,118,236,1,130,0,235,254,59,205,2,87,129,25,255,118"

func knotHashRount() {
    for length in lengthSequence {
        for offset in 0..<length / 2 {
            let idx1 = (index + offset) % 256
            let idx2 = (index + length - offset - 1) % 256

            sparseHash.swapAt(idx1, idx2)
        }

        index = (index + length + skipSize) % 256
        skipSize += 1
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var sparseHash: [UInt8] = Array(0...255)
var lengthSequence = input
    .split(separator: ",")
    .map { Int($0)! }
var index = 0
var skipSize = 0

for x in 0...255 {
    sparseHash[x] = UInt8(x)
}

knotHashRount()

print("Result for puzzle 1: \(UInt(sparseHash[0]) * UInt(sparseHash[1]))")


// --------------------
//   MARK: - Puzzle 2
// --------------------

lengthSequence = input
    .utf8
    .map { Int($0) }
lengthSequence.append(contentsOf: [17, 31, 73, 47, 23])
index = 0
skipSize = 0
sparseHash = Array(0...255)

for _ in 1...64 {
    knotHashRount()
}

var denseHash = ""

for block in stride(from: 0, to: 255, by: 16) {
    denseHash += String(format: "%02x", sparseHash[block + 1...block + 15].reduce(sparseHash[block], ^))
}

print("Result for puzzle 2: \(denseHash)")
