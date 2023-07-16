// ---------------------------------------------------
//  main.swift
//  Day 1: Inverse Captcha
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

let inputChars = input.utf8.map { Int($0 - 48) }
let charCount = inputChars.count


// --------------------
//   MARK: - Puzzle 1
// --------------------

var captcha = inputChars[0] == inputChars[charCount - 1] ? inputChars[0] : 0

for idx in 0..<charCount - 2 {
    if inputChars[idx] == inputChars[idx + 1] {
        captcha += inputChars[idx]
    }
}

print("Result for puzzle 1: \(captcha)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

let offset = charCount / 2
captcha = 0

for idx in 0..<charCount {
    if inputChars[idx] == inputChars[(idx + offset) % charCount] {
        captcha += inputChars[idx]
    }
}

print("Result for puzzle 2: \(captcha)")
