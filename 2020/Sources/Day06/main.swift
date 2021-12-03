// ---------------------------------------------------
//  main.swift
//  Day 6: Custom Customs
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let questions = input.components(separatedBy: "\n\n")


// --------------------
//   MARK: - Puzzle 1
// --------------------

var numberOfQuestions = questions.map { Set($0.replacingOccurrences(of: "\n", with: "")).count }.reduce(0, +)

print("Result for puzzle 1: \(numberOfQuestions)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

let startSet = Set("abcdefghijklmnopqrstuvwxyz")
numberOfQuestions = 0

for group in questions {
    let persons = group.components(separatedBy: .newlines).map { Set($0) }
    var combinedSet = startSet

    for person in persons {
        combinedSet = combinedSet.intersection(person)
    }

    numberOfQuestions += combinedSet.count
}

print("Result for puzzle 2: \(numberOfQuestions)")
