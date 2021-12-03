// ---------------------------------------------------
//  main.swift
//  Day 22: Crab Combat
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let decks = input.components(separatedBy: "\n\n")

let startDeck1 = Array(decks[0].components(separatedBy: .newlines)[1...].map { Int($0)! }.reversed())
let startDeck2 = Array(decks[1].components(separatedBy: .newlines)[1...].map { Int($0)! }.reversed())

extension Array where Element == Int {
    var score: Int {
        var value = 0

        for (idx, element) in self.enumerated() {
            value += (idx + 1) * element
        }

        return value
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var deck1 = startDeck1
var deck2 = startDeck2

while deck1.count > 0 && deck2.count > 0 {
    let card1 = deck1.popLast()!
    let card2 = deck2.popLast()!

    if card1 > card2 {
        deck1.insert(card1, at: 0)
        deck1.insert(card2, at: 0)
    } else {
        deck2.insert(card2, at: 0)
        deck2.insert(card1, at: 0)
    }
}

deck1.append(contentsOf: deck2)

print("Result for puzzle 1: \(deck1.score)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

@discardableResult
func playGame(with deck1: [Int], and deck2: [Int]) -> Bool {
    var deck1 = deck1
    var deck2 = deck2
    var previous1 = [[Int]]()
    var previous2 = [[Int]]()

    while deck1.count > 0 && deck2.count > 0 {
        if previous1.contains(deck1) || previous2.contains(deck2) {
            winnerDeck = deck1
            return true
        }

        previous1.append(deck1)
        previous2.append(deck2)

        let card1 = deck1.popLast()!
        let card2 = deck2.popLast()!

        if card1 <= deck1.count && card2 <= deck2.count {
            if playGame(with: Array(deck1.suffix(from: deck1.count - card1)), and: Array(deck2.suffix(from: deck2.count - card2))) {
                deck1.insert(card1, at: 0)
                deck1.insert(card2, at: 0)
            } else {
                deck2.insert(card2, at: 0)
                deck2.insert(card1, at: 0)
            }
        } else {
            if card1 > card2 {
                deck1.insert(card1, at: 0)
                deck1.insert(card2, at: 0)
            } else {
                deck2.insert(card2, at: 0)
                deck2.insert(card1, at: 0)
            }
        }
    }

    winnerDeck = deck1.count > 0 ? deck1 : deck2

    return deck1.count > 0
}

var winnerDeck = [Int]()

playGame(with: startDeck1, and: startDeck2)

print("Result for puzzle 2: \(winnerDeck.score)")
