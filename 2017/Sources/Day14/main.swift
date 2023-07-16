// ---------------------------------------------------
//  main.swift
//  Day 14: Disk Defragmentation
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let input = "xlqgujun"
let gridSize = 128


// --------------------
//   MARK: - Puzzle 1
// --------------------

func knotHash(keyString: String) -> [Int] {
    var index = 0
    var skipSize = 0
    var sparseHash = Array(0...255)
    var key = keyString
        .utf8
        .map { Int($0) }
    key.append(contentsOf: [17, 31, 73, 47, 23])

    for _ in 1...64 {
        for x in key {
            for offset in 0..<x / 2 {
                let idx1 = (index + offset) % 256
                let idx2 = (index + x - offset - 1) % 256

                sparseHash.swapAt(idx1, idx2)
            }

            index = (index + x + skipSize) % 256
            skipSize += 1
        }
    }

    var denseHash = [Int]()

    for block in stride(from: 0, to: 255, by: 16) {
        denseHash.append(sparseHash[block + 1...block + 15].reduce(sparseHash[block], ^))
    }

    return denseHash
}

var usedCount = 0
var grid = Array(repeating: Array(repeating: ".", count: gridSize), count: gridSize)

for row in 0..<gridSize {
    let rowKey = "\(input)-\(row)"

    for (byte, var cols) in knotHash(keyString: rowKey).enumerated() {
        for column in stride(from: byte * 8 + 7, to: byte * 8 - 1, by: -1) {
            if cols & 1 == 1 {
                grid[row][column] = "#"
                usedCount += 1
            }

            cols >>= 1
        }
    }
}

print("Result for puzzle 1: \(usedCount)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

func clearRegion(_ row: Int, _ column: Int) {
    var stack = [(row, column)]

    while let (r, c) = stack.popLast() {
        grid[r][c] = "."

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (rr, cc) = (r + dr, c + dc)

            if rr >= 0 && rr < gridSize && cc >= 0 && cc < gridSize && grid[rr][cc] == "#" {
                stack.append((rr, cc))
            }
        }
    }
}

func isRegion(_ row: Int, _ column: Int) -> Int {
    if grid[row][column] == "." {
        return 0
    }

    clearRegion(row, column)
    return 1
}

var regionCount = 0

for row in 0..<gridSize {
    for column in 0..<gridSize {
        regionCount += isRegion(row, column)
    }
}

print("Result for puzzle 2: \(regionCount)")
