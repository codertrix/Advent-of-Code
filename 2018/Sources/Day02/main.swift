// ---------------------------------------------------
//  main.swift
//  Day 2: Inventory Management System
//
//  Copyright © 2018 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let inputValues = input.components(separatedBy: .newlines)


// --------------------
//   MARK: - Puzzle 1
// --------------------

var twoTimesCount = 0
var threeTimesCount = 0

inputValues.forEach { (id) in
    var foundTwoTimes = false
    var foundThreeTimes = false
    let countedSet = NSCountedSet(array: Array(id))

    countedSet.objectEnumerator().forEach({
        let count = countedSet.count(for: $0)
        if count == 2 { foundTwoTimes = true }
        if count == 3 { foundThreeTimes = true }
    })

    if foundTwoTimes { twoTimesCount += 1 }
    if foundThreeTimes { threeTimesCount += 1 }
}

print("Result for puzzle 1: \(twoTimesCount * threeTimesCount)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var commonLetters = ""
let commonIDLength = 25
let inputCount = inputValues.count

outerLoop: for index1 in 0..<inputCount - 1 {
    let id1 = inputValues[index1]

    for index2 in (index1 + 1)..<inputCount {
        let id2 = inputValues[index2]
        commonLetters = ""

        for (char1, char2) in zip(id1, id2) {
            if char1 == char2 { commonLetters.append(char1) }
        }

        if commonLetters.count == commonIDLength {
            break outerLoop
        }
    }
}

print("Result for puzzle 2: \(commonLetters)")
