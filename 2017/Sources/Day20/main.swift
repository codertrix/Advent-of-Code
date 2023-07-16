// ---------------------------------------------------
//  main.swift
//  Day 20: Particle Swarm
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

class Particle {
    var p: (x: Int, y: Int, z: Int)
    var v: (x: Int, y: Int, z: Int)
    let a: (x: Int, y: Int, z: Int)

    init(p: (x: Int, y: Int, z: Int), v: (x: Int, y: Int, z: Int), a: (x: Int, y: Int, z: Int)) {
        self.p = p
        self.v = v
        self.a = a
    }

    static func ==(lhs: Particle, rhs: Particle) -> Bool {
        lhs.p == rhs.p
    }

    func tick() {
        v = (v.x + a.x, v.y + a.y, v.z + a.z)
        p = (p.x + v.x, p.y + v.y, p.z + v.z)
    }
}

var particles = input
    .components(separatedBy: .newlines)
    .map {
        var values = [Int]()

        for part in $0.split(whereSeparator: { "<>,".contains($0) }) {
            if let value = Int(part) {
                values.append(value)
            }
        }

        return Particle(p: (values[0], values[1], values[2]), v: (values[3], values[4], values[5]), a: (values[6], values[7], values[8]))
    }


// --------------------
//   MARK: - Puzzle 1
// --------------------

var a_min = Int.max
var closestParticle = -1

for (idx, particle) in particles.enumerated() {
    let a = abs(particle.a.x) + abs(particle.a.y) + abs(particle.a.z)

    if a < a_min {
        closestParticle = idx
        a_min = a
    }
}

print("Result for puzzle 1: \(closestParticle)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

for _ in 1...39 {
    var indicesToRemove = [Int]()
    particles.forEach { $0.tick() }

    for (idx, particle) in particles.enumerated() {
        if let firstIdx = particles.firstIndex(where: { $0 == particle }) {
            if firstIdx == idx {
                continue
            }

            if !indicesToRemove.contains(firstIdx) {
                indicesToRemove.append(firstIdx)
            }

            indicesToRemove.append(idx)
        }
    }

    while let idx = indicesToRemove.popLast() {
        particles.remove(at: idx)
    }
}

print("Result for puzzle 2: \(particles.count)")
