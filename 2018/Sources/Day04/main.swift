// ---------------------------------------------------
//  main.swift
//  Day 4: Repose Record
//
//  Copyright © 2018 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let inputValues = input
    .components(separatedBy: .newlines)
    .sorted()

var guards = [String: [Int]]()
var theGuard = ""
var guardTimeline = [Int]()
var sleepStartMinute = 0

inputValues.forEach { (entry) in
    let elements = entry
        .split(whereSeparator: { "[] :".contains($0) })
        .compactMap { String($0) }

    switch elements[3] {
    case "Guard":
        theGuard = elements[4]
        guardTimeline = guards[theGuard] ?? Array(repeating: 0, count: 60)
    case "falls":
        sleepStartMinute = Int(elements[2])!
    case "wakes":
        for minute in sleepStartMinute..<Int(elements[2])! {
            guardTimeline[minute] += 1
        }
        guards[theGuard] = guardTimeline
   default:
        preconditionFailure()
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var (guardID, maxSleepTime) = guards.max { $0.value.reduce(0, +) < $1.value.reduce(0, +) }!
var maxMinute = guards[guardID]?.firstIndex(of: maxSleepTime.max()!) ?? -1

print("Result for puzzle 1: \(Int(guardID.dropFirst())! * maxMinute)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

(guardID, maxSleepTime) = guards.max { $0.value.max()! < $1.value.max()! }!
maxMinute = guards[guardID]?.firstIndex(of: maxSleepTime.max()!) ?? -1

print("Result for puzzle 2: \(Int(guardID.dropFirst())! * maxMinute)")
