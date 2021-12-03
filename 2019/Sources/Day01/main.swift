// ---------------------------------------------------
//  main.swift
//  Day 1: The Tyranny of the Rocket Equation
//
//  Copyright © 2019 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let masses = input
    .components(separatedBy: .newlines)
    .compactMap { Int($0) }


// --------------------
//   MARK: - Puzzle 1
// --------------------

var sumOfFuel = 0

for mass in masses {
    sumOfFuel += mass / 3 - 2
}

print("Result for puzzle 1: \(sumOfFuel)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

sumOfFuel = 0

for var mass in masses {
    mass = mass / 3 - 2

    while mass > 0 {
        sumOfFuel += mass
        mass = mass / 3 - 2
    }
}

print("Result for puzzle 2: \(sumOfFuel)")
