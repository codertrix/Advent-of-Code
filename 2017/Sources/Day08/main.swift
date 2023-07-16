// ---------------------------------------------------
//  main.swift
//  Day 8: I Heard You Like Registers
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let instructions = input.components(separatedBy: .newlines)


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

var registers = [String: Int]()
var highestValue = 0

for instruction in instructions {
    let parts = instruction.components(separatedBy: .whitespaces)

    let registerValue = registers[parts[4]] ?? 0
    let checkValue = Int(parts[6])!

    switch parts[5] {
        case ">" where registerValue > checkValue,
            "<" where registerValue < checkValue,
            ">=" where registerValue >= checkValue,
            "<=" where registerValue <= checkValue,
            "==" where registerValue == checkValue,
            "!=" where registerValue != checkValue:

            let amount = Int(parts[2])!
            let value = registers[parts[0]] ?? 0

            registers[parts[0]] = value + (parts[1] == "inc" ? amount : -amount)
            highestValue = max(highestValue, value)
        default:
            continue
    }
}

print("Result for puzzle 1: \(registers.values.max()!)")
print("Result for puzzle 2: \(highestValue)")
