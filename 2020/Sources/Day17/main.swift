// ---------------------------------------------------
//  main.swift
//  Day 17: Conway Cubes
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

struct Cube: Hashable {
    let x, y, z, w: Int

    init(x: Int, y: Int, z: Int = 0, w: Int = 0) {
        self.x = x
        self.y = y
        self.z = z
        self.w = w
    }
}

let pocketDimension = input.components(separatedBy: .newlines).map { Array($0) }

func startDimension(from dimension: [[String.Element]]) -> Set<Cube> {
    var startDimension = Set<Cube>()

    for y in 0..<dimension.count {
        for x in 0..<dimension[y].count {
            if dimension[y][x] == "#" {
                startDimension.insert(Cube(x: x, y: y))
            }
        }
    }

    return startDimension
}

func activeNeighbors(for cube: Cube, fourthDimension: Bool = false) -> Int {
    let dx = [cube.x - 1, cube.x, cube.x + 1]
    let dy = [cube.y - 1, cube.y, cube.y + 1]
    let dz = [cube.z - 1, cube.z, cube.z + 1]
    let dw = [cube.w - 1, cube.w, cube.w + 1]
    var count = 0

    for zz in dz {
        for yy in dy {
            for xx in dx {
                if fourthDimension {
                    for ww in dw {
                        if xx == cube.x && yy == cube.y && zz == cube.z && ww == cube.w { continue }
                        if dimension.contains(Cube(x: xx, y: yy, z: zz, w: ww)) { count += 1 }
                    }
                } else {
                    if xx == cube.x && yy == cube.y && zz == cube.z { continue }
                    if dimension.contains(Cube(x: xx, y: yy, z: zz)) { count += 1 }
                }
            }
        }
    }

    return count
}

func cubeCountAfterBoot(fourthDimension: Bool = false) -> Int {
    var nextDimension: Set<Cube>

    func check(cube: Cube, with activeCount: Int) {
        if dimension.contains(cube) {
            if activeCount != 2 && activeCount != 3 { nextDimension.remove(cube) }
        } else {
            if activeCount == 3 { nextDimension.insert(cube) }
        }
    }

    var xRange = 0...pocketDimension[0].count - 1
    var yRange = 0...pocketDimension.count - 1
    var zRange = 0...0
    var wRange = 0...0

    for _ in 1...6 {
        nextDimension = dimension
        xRange = xRange.lowerBound - 1...xRange.upperBound + 1
        yRange = yRange.lowerBound - 1...yRange.upperBound + 1
        zRange = zRange.lowerBound - 1...zRange.upperBound + 1
        wRange = wRange.lowerBound - 1...wRange.upperBound + 1

        for z in zRange {
            for y in yRange {
                for x in xRange {
                    if fourthDimension {
                        for w in wRange {
                            let cube = Cube(x: x, y: y, z: z, w: w)
                            let activeCount = activeNeighbors(for: cube, fourthDimension: true)
                            check(cube: cube, with: activeCount)
                        }
                    } else {
                        let cube = Cube(x: x, y: y, z: z)
                        let activeCount = activeNeighbors(for: cube)
                        check(cube: cube, with: activeCount)
                    }
                }
            }
        }

        dimension = nextDimension
    }

    return dimension.count
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var dimension = startDimension(from: pocketDimension)

print("Result for puzzle 1: \(cubeCountAfterBoot())")


// --------------------
//   MARK: - Puzzle 2
// --------------------

dimension = startDimension(from: pocketDimension)

print("Result for puzzle 2: \(cubeCountAfterBoot(fourthDimension: true))")
