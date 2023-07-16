// ---------------------------------------------------
//  main.swift
//  Day 18: Duet
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

var registers = [Character: Int]()
let program = input
    .components(separatedBy: .newlines)
    .map { $0.components(separatedBy: .whitespaces) }
    .map {
        let x = $0[1].first!
        var y: RegVal? = nil

        if $0.count == 3 {
            if let value = Int($0[2]) {
                y = RegVal.value(value)
            } else {
                y = RegVal.register($0[2].first!)
            }
        }

        if Int($0[1]) == nil {
            registers[x] = 0
        }

        switch $0[0] {
            case "add": return Instruction.add(x, y!)
            case "jgz":
                if let value = Int($0[1]) {
                    return Instruction.jgz(RegVal.value(value), y!)
                } else {
                    return Instruction.jgz(RegVal.register(x), y!)
                }
            case "mod": return Instruction.mod(x, y!)
            case "mul": return Instruction.mul(x, y!)
            case "rcv": return Instruction.rcv(x)
            case "set": return Instruction.set(x, y!)
            case "snd":
                if let value = Int($0[1]) {
                    return Instruction.snd(RegVal.value(value))
                } else {
                    return Instruction.snd(RegVal.register(x))
                }
            default: fatalError()
        }
    }

enum Instruction {
    case add(Character, RegVal)
    case jgz(RegVal, RegVal)
    case mod(Character, RegVal)
    case mul(Character, RegVal)
    case rcv(Character)
    case set(Character, RegVal)
    case snd(RegVal)
}

enum RegVal {
    case register(Character)
    case value(Int)
}

struct Process {
    var program: [Instruction]
    var registers: [Character: Int]
    var ip = 0
    var sndQueue = [Int]()
    var sndCount = 0
    var finished = false

    func valueFor(_ regOrVal: RegVal) -> Int {
        switch regOrVal {
            case let .register(register): return registers[register]!
            case let .value(value): return value
        }
    }

    mutating func run(_ rcvQueue: [Int], isPuzzleOne: Bool = false) {
        var rcvQueue = rcvQueue
        sndQueue = [Int]()

        while ip >= 0 && ip < program.count {
            switch program[ip] {
                case let .add(register, regOrVal): registers[register] = registers[register]! + valueFor(regOrVal)
                case let .jgz(regOrValX, regOrValY): if valueFor(regOrValX) > 0 { ip += valueFor(regOrValY) - 1 }
                case let .mod(register, regOrVal): registers[register] = registers[register]! % valueFor(regOrVal)
                case let .mul(register, regOrVal): registers[register] = registers[register]! * valueFor(regOrVal)
                case let .set(register, regOrVal): registers[register] = valueFor(regOrVal)
                case let .snd(regOrVal):
                    sndQueue.append(valueFor(regOrVal))
                    sndCount += 1
                case let .rcv(register):
                    if isPuzzleOne && registers[register]! != 0 || rcvQueue.isEmpty {
                        return
                    } else {
                        registers[register] = rcvQueue.popLast()
                    }
            }

            ip += 1
        }

        finished = true
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var process = Process(program: program, registers: registers)

process.run([], isPuzzleOne: true)

print("Result for puzzle 1: \(process.sndQueue.last!)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var process0 = Process(program: program, registers: registers)
var process1 = Process(program: program, registers: registers)

process1.registers["p"] = 1
process0.run([])
process1.run([])

while !process0.finished && !process0.sndQueue.isEmpty || !process1.finished && !process1.sndQueue.isEmpty {
    var rcvQueue0 = process1.sndQueue
    var rcvQueue1 = process0.sndQueue

    rcvQueue0.reverse()
    rcvQueue1.reverse()
    process0.run(rcvQueue0)
    process1.run(rcvQueue1)
}

print("Result for puzzle 2: \(process1.sndCount)")
