// ---------------------------------------------------
//  main.swift
//  Day 23: Coprocessor Conflagration
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let program = input
    .components(separatedBy: .newlines)
    .map { $0.components(separatedBy: .whitespaces) }
    .map {
        let x = Int($0[1].unicodeScalars.first!.value) - 97
        var y: RegVal? = nil

        if $0.count == 3 {
            if let value = Int($0[2]) {
                y = RegVal.value(value)
            } else {
                y = RegVal.register(Int($0[2].unicodeScalars.first!.value) - 97)
            }
        }

        if Int($0[1]) == nil {
            //registers[x] = 0
        }

        switch $0[0] {
            case "jnz":
                if let value = Int($0[1]) {
                    return Instruction.jnz(RegVal.value(value), y!)
                } else {
                    return Instruction.jnz(RegVal.register(x), y!)
                }
            case "mul": return Instruction.mul(x, y!)
            case "set": return Instruction.set(x, y!)
            case "sub": return Instruction.sub(x, y!)
            default: fatalError()
        }
    }

enum Instruction {
    case jnz(RegVal, RegVal)
    case mul(Int, RegVal)
    case set(Int, RegVal)
    case sub(Int, RegVal)
}

enum RegVal {
    case register(Int)
    case value(Int)
}

struct Process {
    var program: [Instruction]
    var registers: [Int]
    var ip = 0
    var mulCount = 0

    func valueFor(_ regOrVal: RegVal) -> Int {
        switch regOrVal {
            case let .register(register): return registers[register]
            case let .value(value): return value
        }
    }

    mutating func run() {
        while ip >= 0 && ip < program.count {
            switch program[ip] {
                case let .jnz(regOrValX, regOrValY): if valueFor(regOrValX) != 0 { ip += valueFor(regOrValY) - 1 }
                case let .mul(register, regOrVal):
                    registers[register] = registers[register] * valueFor(regOrVal)
                    mulCount += 1
                case let .set(register, regOrVal): registers[register] = valueFor(regOrVal)
                case let .sub(register, regOrVal): registers[register] = registers[register] - valueFor(regOrVal)
            }

            ip += 1
        }
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var process = Process(program: program, registers: Array(repeating: 0, count: 8))

process.run()

print("Result for puzzle 1: \(process.mulCount)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var b = 81 * 100 + 100000
var c = b + 17000
var h = 0

while b <= c {
    var d = 2

    while d < b {
        if b % d == 0 {
            h += 1
            break
        }

        d += 1
    }

    b += 17
}

print("Result for puzzle 2: \(h)")
