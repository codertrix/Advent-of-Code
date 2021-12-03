// ---------------------------------------------------
//  main.swift
//  Day 23: Crab Cups
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let input = "398254716"

func move(_ cups: inout [Int], currentCup: Int, times: Int) {
    var currentCup = currentCup

    for _ in 1...times {
        let pickedCup1 = cups[currentCup]
        let pickedCup2 = cups[pickedCup1]
        let pickedCup3 = cups[pickedCup2]

        cups[currentCup] = cups[pickedCup3]

        var destCup = currentCup

        repeat {
            destCup -= 1

            if destCup < 1 {
                destCup = cups.count - 1
            }
        } while [pickedCup1, pickedCup2, pickedCup3].contains(destCup)

        cups[pickedCup3] = cups[destCup]
        cups[destCup] = pickedCup1

        currentCup = cups[currentCup]
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var cups = input.map { Int(String($0))! }
cups.append(cups.first!)
var circleOfCups = Array(repeating: 0, count: cups.count)

for idx in 0..<cups.count - 1 {
    circleOfCups[cups[idx]] = cups[idx + 1]
}

move(&circleOfCups, currentCup: cups.first!, times: 100)

var cup = 1
var cupsAfterCupOne = ""

while circleOfCups[cup] != 1 {
    cup = circleOfCups[cup]
    cupsAfterCupOne.append("\(cup)")
}

print("Result for puzzle 1: \(cupsAfterCupOne)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

circleOfCups = Array(repeating: 0, count: 1_000_001)
circleOfCups[1_000_000] = cups.first!
cups = input.map { Int(String($0))! }
cups.append(cups.max()! + 1)

for idx in 0..<cups.count - 1 {
    circleOfCups[cups[idx]] = cups[idx + 1]
}

for cup in cups.max()!..<1_000_000 {
    circleOfCups[cup] = cup + 1
}

move(&circleOfCups, currentCup: cups.first!, times: 10_000_000)
let starCup1 = circleOfCups[1]

print("Result for puzzle 2: \(starCup1 * circleOfCups[starCup1])")
