// ---------------------------------------------------
//  main.swift
//  Day 16: Ticket Translation
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

struct Rule {
    let name: String
    let range1: ClosedRange<Int>
    let range2: ClosedRange<Int>
    var index = -1

    func contains(_ value: Int) -> Bool {
        range1.contains(value) || range2.contains(value)
    }
}

let inputParts = input.components(separatedBy: "\n\n")
var lines = inputParts[0].components(separatedBy: .newlines)
var rules = [Rule]()

for line in lines {
    let parts = line.replacingOccurrences(of: " or ", with: ": ").components(separatedBy: ": ")
    let range1 = parts[1].split(separator: "-").map { Int($0)! }
    let range2 = parts[2].split(separator: "-").map { Int($0)! }

    rules.append(Rule(name: parts[0], range1: range1[0]...range1[1], range2: range2[0]...range2[1]))
}

lines = inputParts[1].components(separatedBy: .newlines)
let myTicket = lines.last!.split(separator: ",").map { Int($0)! }

lines = inputParts[2].components(separatedBy: .newlines)
var nearbyTickets = [[Int]]()

for idx in 1..<lines.count {
    nearbyTickets.append(lines[idx].split(separator: ",").map { Int($0)! })
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var errorRate = 0
var invalidTicketIdxs = Set<Int>()

loop:
for (idx, value) in nearbyTickets.joined().enumerated() {
    for rule in rules {
        if rule.contains(value) {
            continue loop
        }
    }

    invalidTicketIdxs.insert(idx / rules.count)
    errorRate += value
}

print("Result for puzzle 1: \(errorRate)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var validTickets = [[Int]]()

for (idx, ticket) in nearbyTickets.enumerated() {
    if !invalidTicketIdxs.contains(idx) {
        validTickets.append(ticket)
    }
}

var fields = [Int: [Int]]()

for fieldIdx in 0..<rules.count {
    var possibleRules = [Int]()

    for (ruleIdx, rule) in rules.enumerated() {
        if validTickets.reduce(true, { $0 && rule.contains($1[fieldIdx]) }) {
            possibleRules.append(ruleIdx)
        }
    }

    fields[fieldIdx] = possibleRules
}

while let field = fields.first(where: { $0.value.count == 1 }) {
    let ruleIdx = field.value.first!
    rules[ruleIdx].index = field.key

    for key in fields.keys {
        fields[key]?.removeAll(where: { $0 == ruleIdx })
    }
}

print("Result for puzzle 2: \(rules[0...5].reduce(1, { $0 * myTicket[$1.index] }))")
