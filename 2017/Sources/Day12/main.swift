// ---------------------------------------------------
//  main.swift
//  Day 12: Digital Plumber
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let pipes = input
    .components(separatedBy: .newlines)
    .map { $0.components(separatedBy: " <-> ") }
    .map { $0[1]
        .components(separatedBy: ", ")
        .map { Int($0)! }
    }

func groupFor(program: Int) -> Set<Int> {
    var group = Set([program])
    var seen = Set<Int>()
    var queue = [program]

    while let nextProgram = queue.popLast() {
        if seen.contains(nextProgram) {
            continue
        }

        seen.insert(nextProgram)

        for p in pipes[nextProgram] {
            group.insert(p)
            queue.append(p)
        }
    }

    return group
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

print("Result for puzzle 1: \(groupFor(program: 0).count)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var groupsCount = 0
var programs = Set(0..<pipes.count)

while !programs.isEmpty {
    for program in groupFor(program: programs.first!) {
        programs.remove(program)
    }

    groupsCount += 1
}

print("Result for puzzle 2: \(groupsCount)")
