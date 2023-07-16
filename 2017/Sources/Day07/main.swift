// ---------------------------------------------------
//  main.swift
//  Day 7: Recursive Circus
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let lines = input.components(separatedBy: .newlines)

class Program {
    let name: String
    var bottom: String
    var weight: Int
    var stack: [Program]

    var isBottom: Bool { bottom.count == 0 }
    var towerWeight: Int {
        if stack.count == 0 {
            return weight
        } else {
            return stack.reduce(weight) { $0 + $1.towerWeight }
        }
    }

    init(name: String, bottom: String = "", weight: Int = 0, stack: [Program] = [Program]()) {
        self.name = name
        self.bottom = bottom
        self.weight = weight
        self.stack = stack
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var programs = [Program]()

for line in lines {
    let parts = line.components(separatedBy: " -> ")
    let leftParts = parts[0].components(separatedBy: CharacterSet(charactersIn: " ()"))
    let weight = Int(leftParts[2])!
    var program = Program(name: leftParts[0], weight: weight)

    if let existing = programs.first(where: { $0.name == leftParts[0] }) {
        existing.weight = weight
        program = existing
    } else {
        programs.append(program)
    }

    if parts.count == 2 {
        for subTower in parts[1].components(separatedBy: ", ") {
            var subProgram = Program(name: subTower, bottom: program.name)

            if let existing = programs.first(where: { $0.name == subTower }) {
                existing.bottom = program.name
                subProgram = existing
            }

            program.stack.append(subProgram)
            programs.append(subProgram)
        }
    }
}

let bottomProgram = programs.first { $0.isBottom }!

print("Result for puzzle 1: \(bottomProgram.name)")

// --------------------
//   MARK: - Puzzle 2
// --------------------

var balancedWeight = 0
var currentProgram = bottomProgram

while true {
    var weights = [Int: [Program]]()

    for program in currentProgram.stack {
        let towerWeight = program.towerWeight
        var programs = [program]

        if let existing = weights[towerWeight] {
            programs += existing
        }

        weights[towerWeight] = programs
    }

    if weights.count == 1 {
        break
    }

    var balanced = 0
    var unbalanced = 0

    for (towerWeight, programs) in weights {
        if programs.count == 1 {
            currentProgram = programs.first!
            unbalanced = towerWeight
        } else {
            balanced = towerWeight
        }
    }

    balancedWeight = currentProgram.weight - (unbalanced - balanced)
}

print("Result for puzzle 2: \(balancedWeight)")
