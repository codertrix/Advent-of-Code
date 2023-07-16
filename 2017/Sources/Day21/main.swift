// ---------------------------------------------------
//  main.swift
//  Day 21: Fractal Art
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var rules = [String: [String]]()

for line in input.components(separatedBy: .newlines) {
    let parts = line.components(separatedBy: " => ")
    let newSquare = parts[1].components(separatedBy: "/")
    var key = parts[0]
    let squareSize = key.count == 5 ? 2 : 3

    for i in 1...8 {
        rules[key] = newSquare

        if i == 4 {
            let prefix = key.prefix(squareSize)
            let suffix = key.suffix(squareSize)
            key.removeFirst(squareSize)
            key.removeLast(squareSize)
            key = suffix + key + prefix
        } else {
            key = rotate(pattern: key, size: squareSize)
        }
    }
}

func rotate(pattern: String, size: Int) -> String {
    let oldPattern = pattern.map { $0 }
    var newPattern = ""

    for col in 0..<size {
        for row in stride(from: size - 1, to: -1, by: -1) {
            newPattern.append(oldPattern[row * size + row + col])
        }

        if col < size - 1 {
            newPattern.append("/")
        }
    }

    return newPattern
}

func generateArt(iterations: Int) -> Int {
    var grid = [".#.", "..#", "###"]

    for _ in 1...iterations {
        let squareSize = grid.count & 1 == 0 ? 2 : 3
        var newGrid = [String]()

        for row in stride(from: 0, to: grid.count, by: squareSize) {
            var newRows = Array(repeating: "", count: squareSize + 1)

            for col in stride(from: 0, to: grid.count, by: squareSize) {
                let fromIdx = grid[row].index(grid[row].startIndex, offsetBy: col)
                let toIdx = grid[row].index(fromIdx, offsetBy: squareSize)
                var square = String(grid[row][fromIdx..<toIdx])

                for offset in 1..<squareSize {
                    square += "/" + grid[row + offset][fromIdx..<toIdx]
                }

                if let newSquare = rules[square] {
                    for (offset, squareRow) in newSquare.enumerated() {
                        newRows[offset].append(squareRow)
                    }
                }
            }

            newGrid.append(contentsOf: newRows)
        }

        grid = newGrid
    }

    return grid.joined().reduce(0, { $0 + ($1 == "#" ? 1 : 0) })
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

print("Result for puzzle 1: \(generateArt(iterations: 5))")


// --------------------
//   MARK: - Puzzle 2
// --------------------

print("Result for puzzle 2: \(generateArt(iterations: 18))")
