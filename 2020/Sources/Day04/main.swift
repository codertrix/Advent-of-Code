// ---------------------------------------------------
//  main.swift
//  Day 4: Passport Processing
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let passports = input.components(separatedBy: "\n\n")


// --------------------
//   MARK: - Puzzle 1
// --------------------

let required = Set(["byr", "iyr","eyr", "hgt", "hcl", "ecl", "pid"])
var validPassports = 0

for passport in passports {
    var valid = true
    for field in required {
        if !passport.contains(field) {
            valid = false
            break
        }
    }

    if valid { validPassports += 1 }
}

print("Result for puzzle 1: \(validPassports)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

let hclRegex = try! NSRegularExpression(pattern: "^#[\\da-f]{6}$")
let eclSet = Set(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
validPassports = 0

for passport in passports {
    let fields = passport.split { $0 == "\n" || $0 == ":" || $0 == " " }
    var validCount = 0
    for idx in stride(from: 0, to: fields.count, by: 2) {
        let value = fields[idx + 1]
        switch fields[idx] {
            case "byr":
                validCount += (1920...2002).contains(Int(value)!) ? 1 : 0
            case "iyr":
                validCount += (2010...2020).contains(Int(value)!) ? 1 : 0
            case "eyr":
                validCount += (2020...2030).contains(Int(value)!) ? 1 : 0
            case "hgt":
                let number = Int(value[..<value.index(value.endIndex, offsetBy: -2)]) ?? 0
                if value.hasSuffix("cm") {
                    validCount += (150...193).contains(number) ? 1 : 0
                } else if value.hasSuffix("in") {
                    validCount += (59...76).contains(number) ? 1 : 0
                }
            case "hcl":
                validCount += hclRegex.numberOfMatches(in: String(value), range: NSMakeRange(0, value.count))
            case "ecl":
                validCount += eclSet.contains(String(value)) ? 1 : 0
            case "pid":
                validCount += (value.count == 9 && Int(value) != nil) ? 1 : 0
            default:
                break
        }
    }

    if validCount == 7 { validPassports += 1 }
}

print("Result for puzzle 2: \(validPassports)")
