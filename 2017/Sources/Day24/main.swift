// ---------------------------------------------------
//  main.swift
//  Day 24: Electromagnetic Moat
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var components = input
    .components(separatedBy: .newlines)
    .map { $0.components(separatedBy: "/") }
    .map { (Int($0[0])!, Int($0[1])!) }


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

func buildBridges(port: Int, components: [(Int, Int)]) -> [(Int, Int)] {
    let possibleComponents = components.filter { $0.0 == port || $0.1 == port }
    var newBridges = [(Int, Int)]()

    for component in possibleComponents {
        let remainingComponents = components.filter { $0 != component }
        let nextPort = component.0 == port ? component.1 : component.0
        let bridges = buildBridges(port: nextPort, components: remainingComponents)

        if bridges.isEmpty {
            newBridges.append((component.0 + component.1, 1))
        } else {
            for bridge in bridges {
                newBridges.append((component.0 + component.1 + bridge.0, bridge.1 + 1))
            }
        }
    }

    return newBridges
}

let bridges = buildBridges(port: 0, components: components)
let resultOne = bridges.max { $0.0 < $1.0 }!.0
let resultTwo = bridges.max {
    if $0.1 == $1.1 {
        return $0.0 < $1.0
    }

    return $0.1 < $1.1
}!.0

print("Result for puzzle 1: \(resultOne)")
print("Result for puzzle 2: \(resultTwo)")
