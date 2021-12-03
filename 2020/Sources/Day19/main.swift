// ---------------------------------------------------
//  main.swift
//  Day 19: Monster Messages
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let inputParts = input.components(separatedBy: "\n\n")
let messages = inputParts[1].components(separatedBy: .newlines)
var rules = [String: String]()

inputParts[0].components(separatedBy: .newlines).forEach {
    let parts = $0.components(separatedBy: ": ")
    if ["\"a\"", "\"b\""].contains(parts[1]) {
        rules[parts[0]] = parts[1].replacingOccurrences(of: "\"", with: "")
    } else {
        rules[parts[0]] = parts[1]
    }
}

func resolve(rule: String, result: [String] = [""]) -> [String] {
    if "ab".contains(rule) {
        return result.map { $0.appending(rule) }
    }

    var resolved = result
    var first = [String]()

    rule.components(separatedBy: " ").forEach {
        if let nextRule = rules[$0] {
            resolved = resolve(rule: nextRule, result: resolved)
        } else {
            first = resolved
            resolved = result
        }
    }

    if !first.isEmpty {
        resolved = first + resolved
    }

    return resolved
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

// Manual resolution of rule 0 results in significantly better performance

let prefixRules = resolve(rule: "8")
var suffixRules = resolve(rule: "11")
let prefixLength = prefixRules.first!.count
let messageLength = prefixLength + suffixRules.first!.count
var messageCount = 0

messages.forEach {
    if $0.count != messageLength { return }

    let splitIdx = $0.index($0.startIndex, offsetBy: prefixLength)

    if !prefixRules.contains(String($0[..<splitIdx])) { return }
    if !suffixRules.contains(String($0[splitIdx...])) { return }

    messageCount += 1
}

print("Result for puzzle 1: \(messageCount)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

messageCount = 0
suffixRules = resolve(rule: "31")
let suffixLength = suffixRules.first!.count
let minLength = prefixLength * 2 + suffixLength

messages.forEach {
    if $0.count < minLength { return }

    var matchCount = 0
    var startIdx = $0.startIndex
    var endIdx = $0.index(startIdx, offsetBy: prefixLength)
    let endIndex = $0.endIndex

    while prefixRules.contains(String($0[startIdx..<endIdx])) {
        matchCount += 1

        if $0.distance(from: endIdx, to: endIndex) < prefixLength { break }

        startIdx = endIdx
        $0.formIndex(&endIdx, offsetBy: prefixLength)
    }

    while suffixRules.contains(String($0[startIdx..<endIdx])) {
        matchCount -= 1

        if endIdx == endIndex {
            if matchCount > 0 { messageCount += 1 }
            return
        }

        if $0.distance(from: endIdx, to: endIndex) < suffixLength { return }

        startIdx = endIdx
        $0.formIndex(&endIdx, offsetBy: suffixLength)
    }
}

print("Result for puzzle 2: \(messageCount)")
