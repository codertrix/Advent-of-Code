// ---------------------------------------------------
//  main.swift
//  Day 22: Slam Shuffle
//
//  Copyright © 2019 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

func numberFrom(_ technique: String) -> Int {
    let parts = technique.split(separator: " ").last
    guard let number = Int(String(parts!)) else { fatalError()}

    return number
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var deckSize = 10007
var position = 2019

for tech in input.components(separatedBy: .newlines) {
    if tech.hasPrefix("deal into") {
        position = deckSize - 1 - position
    } else if tech.hasPrefix("deal with") {
        position = (position * numberFrom(String(tech))) % deckSize
    } else {
        var number = numberFrom(String(tech))
        if number < 0 { number = deckSize + number }
        position = position < number ? deckSize - number + position : position - number
    }
}

print("Result for puzzle 1: \(position)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

// This solution runs "forever" -> it's not a solution

/*
let shuffles = 101_741_582_076_661
deckSize = 119_315_717_514_047
position = 2020

for _ in 1...shuffles {
    for tech in input.components(separatedBy: .newlines).reversed() {
        if tech.hasPrefix("deal into") {
            position = deckSize - 1 - position
        } else if tech.hasPrefix("deal with") {
            let number = numberFrom(String(tech))
            var nDecks = 0
            while true {
                if (nDecks + position) % number == 0 {
                    position = (nDecks + position) / number
                    break
                }
                nDecks += deckSize
            }
        } else {
            var number = -numberFrom(String(tech))
            if number < 0 { number = deckSize + number }
            position = position < number ? deckSize - number + position : position - number
        }
    }
}

print("Result for puzzle 2: \(position)")
*/
