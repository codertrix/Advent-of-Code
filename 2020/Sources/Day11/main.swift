// ---------------------------------------------------
//  main.swift
//  Day 11: Seating System
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let area = input.components(separatedBy: .newlines).map { Array($0)}
let areaWidth = area[0].count
let areaHeight = area.count
var lastArea = [[String.Element]]()

func occupiedSeats(tollerance: Int, occupied: (Int, Int) -> Int) -> Int {
    var isChanged = true
    var currentArea = area
    lastArea = area

    while isChanged {
        lastArea = currentArea
        isChanged = false

        for y in 0..<areaHeight {
            for x in 0..<areaWidth {
                switch lastArea[y][x] {
                    case "L":
                        if occupied(x, y) == 0 {
                            currentArea[y][x] = "#"
                            isChanged = true
                        }
                    case "#":
                        if occupied(x, y) >= tollerance {
                            currentArea[y][x] = "L"
                            isChanged = true
                        }
                    default:
                        break
                }
            }
        }
    }

    return lastArea.joined().filter { $0 == "#" }.count
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

func occupiedAdjacent(_ x: Int, _ y: Int) -> Int {
    let dx = [x - 1, x, x + 1]
    let dy = [y - 1, y, y + 1]
    var count = 0

    for yy in dy {
        for xx in dx {
            if xx == x && yy == y { continue }
            if xx >= 0 && xx < areaWidth && yy >= 0 && yy < areaHeight {
                if lastArea[yy][xx] == "#" { count += 1 }
            }
        }
    }

    return count
}

print("Result for puzzle 1: \(occupiedSeats(tollerance: 4, occupied: occupiedAdjacent(_:_:)))")


// --------------------
//   MARK: - Puzzle 2
// --------------------

func occupiedSeen(_ x: Int, _ y: Int) -> Int {
    let dx = [-1, 0, +1]
    let dy = [-1, 0, +1]
    var count = 0

    for yy in dy {
        for xx in dx {
            if xx == 0 && yy == 0 { continue }
            var dxx = x + xx
            var dyy = y + yy

            while dxx >= 0 && dxx < areaWidth && dyy >= 0 && dyy < areaHeight {
                if lastArea[dyy][dxx] != "." {
                    if lastArea[dyy][dxx] == "#" { count += 1 }
                    break
                }

                dxx += xx
                dyy += yy
            }
        }
    }

    return count
}

print("Result for puzzle 2: \(occupiedSeats(tollerance: 5, occupied: occupiedSeen(_:_:)))")
