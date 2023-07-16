// ---------------------------------------------------
//  main.swift
//  Day 15: Dueling Generators
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

let generatorAStart = 699
let generatorBStart = 124
let factorA = 16807
let factorB = 48271
let divider = 2147483647


// --------------------
//   MARK: - Puzzle 1
// --------------------

var pairCount = 0
var generatorA = generatorAStart
var generatorB = generatorBStart

for _ in 0..<40_000_000 {
    generatorA = (generatorA * factorA) % divider
    generatorB = (generatorB * factorB) % divider

    if generatorA & 0xffff == generatorB & 0xffff {
        pairCount += 1
    }
}

print("Result for puzzle 1: \(pairCount)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

pairCount = 0
generatorA = generatorAStart
generatorB = generatorBStart

for _ in 0..<5_000_000 {
    repeat {
        generatorA = (generatorA * factorA) % divider
    } while generatorA & 3 > 0

    repeat {
        generatorB = (generatorB * factorB) % divider
    } while generatorB & 7 > 0

    if generatorA & 0xffff == generatorB & 0xffff {
        pairCount += 1
    }
}

print("Result for puzzle 2: \(pairCount)")
