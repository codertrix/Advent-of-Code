// ---------------------------------------------------
//  main.swift
//  Day 7: Handy Haversacks
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var bags = [String: [(count: Int, color: String)]]()

for line in input.components(separatedBy: .newlines) {
    let parts = line.components(separatedBy: .whitespaces)
    var bag = [(Int, String)]()

    if parts[4] != "no" {
        for idx in stride(from: 4, to: parts.count, by: 4) {
            bag.append((Int(parts[idx])!, parts[idx + 1...idx + 2].joined(separator: " ")))
        }
    }

    bags[parts[0...1].joined(separator: " ")] = bag
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var colors: Set<String> = ["shiny gold"]

while true {
    let count = colors.count

    for color in colors {
        colors = colors.union(bags.filter { $0.value.contains(where: { $0.color == color }) }.map { $0.key })
    }

    if colors.count == count { break }
}

print("Result for puzzle 1: \(colors.count - 1)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

func numberOfBags(for color: String) -> Int {
    bags[color]!.reduce(1) { bagsCount, bag in
        bagsCount + bag.count * numberOfBags(for: bag.color)
    }
}

print("Result for puzzle 2: \(numberOfBags(for: "shiny gold") - 1)")
