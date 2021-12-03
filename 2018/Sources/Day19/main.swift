// ---------------------------------------------------
//  main.swift
//  Day 19: Go With The Flow
//
//  Copyright © 2018 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

enum Opcode: String {
    case addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr
}

struct CPU {
    var register = [0, 0, 0, 0, 0, 0]

    private var program = [(opcode: Opcode?, A: Int, B: Int, C: Int)]()
    private var ip = -1
    private var instructionPointer = 0

    init(program: String) {
        for programLine in program.components(separatedBy: .newlines) {
            let instruction = programLine.split(separator: " ")
            if instruction[0] == "#ip" {
                if let boundRegister = Int(instruction[1]) {
                    ip = boundRegister
                    continue
                }
            } else {
                self.program.append((Opcode(rawValue: String(instruction[0])), Int(instruction[1])!, Int(instruction[2])!, Int(instruction[3])!))
            }
        }
    }

    mutating func run() {
        runUntil(instructionPointer: Int.max)
    }

    mutating func runUntil(instructionPointer: Int) {
        while self.instructionPointer != instructionPointer {
            register[ip] = self.instructionPointer

            guard self.instructionPointer >= 0 && self.instructionPointer < program.count else { return }
            execute(instruction: program[self.instructionPointer])

            self.instructionPointer = register[ip] + 1
        }
    }

    private mutating func execute(instruction: (opcode: Opcode?, A: Int, B: Int, C: Int)) {
        if let opcode = instruction.opcode {
            switch opcode {
            case .addr: register[instruction.C] = register[instruction.A] + register[instruction.B]
            case .addi: register[instruction.C] = register[instruction.A] + instruction.B
            case .mulr: register[instruction.C] = register[instruction.A] * register[instruction.B]
            case .muli: register[instruction.C] = register[instruction.A] * instruction.B
            case .banr: register[instruction.C] = register[instruction.A] & register[instruction.B]
            case .bani: register[instruction.C] = register[instruction.A] & instruction.B
            case .borr: register[instruction.C] = register[instruction.A] | register[instruction.B]
            case .bori: register[instruction.C] = register[instruction.A] | instruction.B
            case .setr: register[instruction.C] = register[instruction.A]
            case .seti: register[instruction.C] = instruction.A
            case .gtir: register[instruction.C] = instruction.A > register[instruction.B] ? 1 : 0
            case .gtri: register[instruction.C] = register[instruction.A] > instruction.B ? 1 : 0
            case .gtrr: register[instruction.C] = register[instruction.A] > register[instruction.B] ? 1 : 0
            case .eqir: register[instruction.C] = instruction.A == register[instruction.B] ? 1 : 0
            case .eqri: register[instruction.C] = register[instruction.A] == instruction.B ? 1 : 0
            case .eqrr: register[instruction.C] = register[instruction.A] == register[instruction.B] ? 1 : 0
            }
        } else {
            fatalError("Bad instruction!")
        }
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var cpu = CPU(program: input)
cpu.run()

print("Result for puzzle 1: \(cpu.register[0])")


// --------------------
//   MARK: - Puzzle 2
// --------------------

cpu = CPU(program: input)
cpu.register[0] = 1
cpu.runUntil(instructionPointer: 3)

var divider: Set<Int> = []
var number = 1

while !divider.contains(number) {
    let result = cpu.register[3] / number

    if cpu.register[3] % number == 0 {
        divider.insert(number)
        divider.insert(result)
    }

    number += 1
}

cpu.register[0] = divider.reduce(0, +)

print("Result for puzzle 2: \(cpu.register[0])")
