// ---------------------------------------------------
//  main.swift
//  Day 21: Allergen Assessment
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let foodList = input.components(separatedBy: .newlines)


// --------------------
//   MARK: - Puzzle 1
// --------------------

var allergens = [String: Set<String>]()
var ingredients = [String]()

for food in foodList {
    let parts = food.split(whereSeparator: { "()".contains($0) }).map { String($0) }
    let ingrdnts1 = Set(parts[0].split(separator: " ").map { String($0) })
    let allrgns = Set(parts[1].replacingOccurrences(of: "contains ", with: "").components(separatedBy: ", ").map { String($0) })

    ingredients.append(contentsOf: ingrdnts1)

    for key in allrgns {
        if let ingrdnts2 = allergens[key] {
            allergens[key] = ingrdnts2.intersection(ingrdnts1)
        } else {
            allergens[key] = ingrdnts1
        }
    }
}

for ingredient in allergens.reduce(Set<String>(), { $0.union($1.value) }) {
    ingredients.removeAll(where: { $0 == ingredient })
}

print("Result for puzzle 1: \(ingredients.count)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

while true {
    let allergens1 = allergens.keys.filter { allergens[$0]!.count == 1 }
    let allergensN = allergens.keys.filter { allergens[$0]!.count > 1 }

    if allergensN.count == 0 { break }

    for keyN in allergensN {
        var valueN = allergens[keyN]!

        for key1 in allergens1 {
            valueN.remove(allergens[key1]!.first!)
        }
        allergens[keyN] = valueN
    }
}

ingredients = [String]()

for key in allergens.keys.sorted() {
    ingredients.append(allergens[key]!.first!)
}

print("Result for puzzle 2: \(ingredients.joined(separator: ","))")
