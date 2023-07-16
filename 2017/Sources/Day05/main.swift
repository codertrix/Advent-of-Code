// ---------------------------------------------------
//  main.swift
//  Day 5: A Maze of Twisty Trampolines, All Alike
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let jumps = input
    .components(separatedBy: .newlines)
    .map { Int($0)! }


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

func run(jumpList: [Int], isPuzzleOne: Bool) -> Int {
    var idx = 0
    var steps = 0
    var jumpList = jumpList

    while idx >= 0 && idx < jumpList.count {
        let offset = jumpList[idx]

        jumpList[idx] += isPuzzleOne || offset < 3 ? 1 : -1
        idx += offset
        steps += 1
    }

    return steps
}

print("Result for puzzle 1: \(run(jumpList: jumps, isPuzzleOne: true))")
print("Result for puzzle 2: \(run(jumpList: jumps, isPuzzleOne: false))")
