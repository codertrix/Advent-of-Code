// ---------------------------------------------------
//  main.swift
//  Day 25: The Halting Problem
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var states = [Character: State]()
var currentState = Character(" ")
var steps = 0

var state = Character(" ")
var currentValue = Character("0")
var valueIsZero = (0, 0, Character(" "))
var valueIsOne = (0, 0, Character(" "))

for line in input.components(separatedBy: .newlines) {
    let lineParts = line
        .components(separatedBy: .whitespaces)
        .filter { $0.count > 0 && $0 != "-" }

    if !lineParts.isEmpty {
        switch lineParts[0] {
            case "Begin":
                currentState = lineParts[3].first!
            case "Perform":
                steps = Int(lineParts[5])!
            case "In":
                state = lineParts[2].first!
            case "If":
                currentValue = lineParts[5].first!
            case "Write":
                if currentValue == "0" {
                    valueIsZero.0 = Int(String(lineParts[3].first!))!
                } else {
                    valueIsOne.0 = Int(String(lineParts[3].first!))!
                }
            case "Move":
                if currentValue == "0" {
                    valueIsZero.1 = lineParts[5] == "right." ? 1 : -1
                } else {
                    valueIsOne.1 = lineParts[5] == "right." ? 1 : -1
                }
            case "Continue":
                if currentValue == "0" {
                    valueIsZero.2 = lineParts[3].first!
                } else {
                    valueIsOne.2 = lineParts[3].first!
                    states[state] = State(valueIsZero: valueIsZero, valueIsOne: valueIsOne)
                }
            default:
                continue
        }
    }
}

struct State {
    let valueIsZero: (Int, Int, Character)
    let valueIsOne: (Int, Int, Character)
}

// --------------------
//   MARK: - Puzzle 1
// --------------------

var tape = Set<Int>()
var cursor = 0

for _ in 1...steps {
    let state = states[currentState]!
    let (value, direction, nextState) = tape.contains(cursor) ? state.valueIsOne : state.valueIsZero

    if value == 1 {
        tape.insert(cursor)
    } else {
        tape.remove(cursor)
    }

    cursor += direction
    currentState = nextState
}

print("Result for puzzle 1: \(tape.count)")
