// ---------------------------------------------------
//  main.swift
//  Day 18: Operation Order
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let expressions = input.components(separatedBy: .newlines)

func result(for expression: String, operatorPrecedence: Bool = false) -> Int {
    var token = ""
    var stack = [String]()
    var idx = expression.startIndex

    loop:
    while idx < expression.endIndex {
        let c = expression[idx]

        switch c {
            case "(":
                let startIdx = expression.index(idx, offsetBy: 1)
                var parentheses = 1

                while parentheses > 0 {
                    idx = expression.index(idx, offsetBy: 1)
                    if expression[idx] == "(" { parentheses += 1 }
                    if expression[idx] == ")" { parentheses -= 1 }
                }

                stack.append("\(result(for: String(expression[startIdx...expression.index(idx, offsetBy: -1)]), operatorPrecedence: operatorPrecedence))")
            case "*":
                token.append(c)

                if operatorPrecedence {
                    stack.append(token)
                    stack.append("\(result(for: String(expression[expression.index(idx, offsetBy: 2)...]), operatorPrecedence: true))")
                    break loop
                }
            case " ":
                if !token.isEmpty {
                    stack.append(token)
                    token = ""
                }
            case _:
                token.append(c)
        }

        idx = expression.index(idx, offsetBy: 1)
    }

    if !token.isEmpty { stack.append(token) }

    var value = Int(stack.first!)!

    for idx in stride(from: 1, to: stack.count - 1, by: 2) {
        if let a = Int(stack[idx + 1]) {
            if stack[idx] == "+" {
                value += a
            } else {
                value *= a
            }
        }
    }

    return value
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

print("Result for puzzle 1: \(expressions.reduce(0, { $0 + result(for: $1) }))")


// --------------------
//   MARK: - Puzzle 2
// --------------------

print("Result for puzzle 2: \(expressions.reduce(0, { $0 + result(for: $1, operatorPrecedence: true) }))")
