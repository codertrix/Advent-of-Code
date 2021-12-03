// ---------------------------------------------------
//  main.swift
//  Day 5: Binary Boarding
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let seats = input.split(separator: "\n").map { Array($0) }


// --------------------
//   MARK: - Puzzle 1
// --------------------

var seatIDs = Set<Int>()

for seat in seats {
    var seatID = 0

    for c in seat {
        seatID <<= 1
        if c == "B" || c == "R" { seatID += 1 }
    }

    seatIDs.insert(seatID)
}

print("Result for puzzle 1: \(seatIDs.max()!)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

for seatID in seatIDs.min()!...seatIDs.max()! {
    if !seatIDs.contains(seatID) && seatIDs.contains(seatID - 1) && seatIDs.contains(seatID + 1) {
        print("Result for puzzle 2: \(seatID)")
        break
    }
}
