// ---------------------------------------------------
//  main.swift
//  Day 6: Memory Reallocation
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

var memoryBanks = [2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14]


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

var history = Set<[Int]>()
var isPuzzleOne = true
var statePuzzleTwo = [Int]()
var resultPuzzleTwo = 0

while true {
    var redistribute = 0

    if isPuzzleOne {
        if history.contains(memoryBanks) {
            isPuzzleOne = false
            statePuzzleTwo = memoryBanks
        }

        history.insert(memoryBanks)
    } else {
        resultPuzzleTwo += 1

        if statePuzzleTwo == memoryBanks {
            break
        }
    }

    for (idx, blocks) in memoryBanks.enumerated() {
        if memoryBanks[redistribute] < blocks {
            redistribute = idx
        }
    }

    let blocks = memoryBanks[redistribute]
    memoryBanks[redistribute] = 0

    for _ in 0..<blocks {
        redistribute = (redistribute + 1) % memoryBanks.count
        memoryBanks[redistribute] += 1
    }
}

print("Result for puzzle 1: \(history.count)")
print("Result for puzzle 2: \(resultPuzzleTwo)")
