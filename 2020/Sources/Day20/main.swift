// ---------------------------------------------------
//  main.swift
//  Day 20: Jurassic Jigsaw
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

enum Edge: Int {
    case top, bottom, left, right
}

class Tile {
    let id: Int
    var grid: [[Character]]
    var top: String { String(grid.first!) }
    var bottom: String { String(grid.last!) }
    var left: String { return grid.reduce("") { $0 + String($1.first!) } }
    var right: String { return grid.reduce("") { $0 + String($1.last!) } }
    var edges: [String] { [top, bottom, left, right] }
    var matches = [Int]()

    init(id: Int, grid: [[Character]]) {
        self.id = id
        self.grid = grid
    }

    func flipVertical() {
        grid.reverse()
    }

    func rotateLeft() {
        var newGrid = grid

        for (row, rowValue) in grid.enumerated() {
            for (col, _) in rowValue.enumerated() {
                newGrid[col][row] = grid[row][col]
            }
        }

        grid = newGrid.reversed()
    }

    @discardableResult
    func arrange(toMatch: String, with edge: Edge) -> Bool {
        for _ in 1...4 {
            if edges[edge.rawValue] == toMatch { return true }

            if edges[edge.rawValue] == String(toMatch.reversed()) {
                if edge == .top || edge == .bottom {
                    grid = grid.map { $0.reversed() }
                } else {
                    grid.reverse()
                }

                return true
            }

            rotateLeft()
        }

        return false
    }
}

let rawTiles = input.components(separatedBy: "\n\n")
var tiles = [Int: Tile]()

for rawTile in rawTiles {
    let lines = rawTile.components(separatedBy: .newlines)
    let firstLine = lines.first!.split(whereSeparator: { $0 == " " || $0 == ":" })
    let id = Int(firstLine[1])!
    let grid = Array(lines[1...]).map { Array($0) }

    tiles[id] = (Tile(id: id, grid: grid))
}

var seaMonsterMask = [(x: Int, y: Int)]()

for (y, row) in seaMonster.components(separatedBy: .newlines).enumerated() {
    for (x, c) in row.enumerated() {
        if c == "#" { seaMonsterMask.append((x, y)) }
    }
}

let seaMonsterHeight = seaMonsterMask.max(by: { $0.y < $1.y })!.y + 1
let seaMonsterWidth = seaMonsterMask.max(by: { $0.x < $1.x })!.x + 1


// --------------------
//   MARK: - Puzzle 1
// --------------------

for tile1 in tiles.values {
    for tile2 in tiles.values {
        if tile1.id == tile2.id { continue }

        for edge in tile1.edges {
            if tile2.edges.contains(edge) || tile2.edges.contains(String(edge.reversed())) {
                tile1.matches.append(tile2.id)
            }
        }
    }
}

var result = tiles.values.filter { $0.matches.count == 2 }.reduce(1) { $0 * $1.id }

print("Result for puzzle 1: \(result)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

var currentTile = tiles.values.filter { $0.matches.count == 2 }.first!
var nextTile = tiles[currentTile.matches[0]]
var tileGrid = [[currentTile.id]]
var placed: Set<Int> = [currentTile.id]

// Place and arrange a corner

while !nextTile!.arrange(toMatch: currentTile.right, with: .left) {
    currentTile.rotateLeft()
}

if !tiles[currentTile.matches[1]]!.arrange(toMatch: currentTile.bottom, with: .top) {
    currentTile.flipVertical()
}

// Place and arrange the other tiles

var row = 0

while placed.count < tiles.count {
    var nextRow = [Int]()
    nextTile = tiles[tileGrid[row][0]]!

    while nextTile != nil {
        currentTile = nextTile!
        nextTile = nil

        currentTile.matches.filter { !placed.contains($0) }.forEach {
            let tile = tiles[$0]!
            placed.insert($0)

            if tile.arrange(toMatch: currentTile.right, with: .left) {
                tileGrid[row].append($0)
                nextTile = tile
            } else {
                nextRow.append($0)
                tile.arrange(toMatch: currentTile.bottom, with: .top)
            }
        }
    }

    if !nextRow.isEmpty {
        tileGrid.append(nextRow)
        row += 1
    }
}

// Create the image

var image = Tile(id: 0, grid: [[Character]]())

for imgRow in 0..<tileGrid.count {
    for tileRow in 1...8 {
        var imageRow = [Character]()

        for imgCol in 0..<tileGrid[imgRow].count {
            imageRow.append(contentsOf: tiles[tileGrid[imgRow][imgCol]]!.grid[tileRow][1...8])
        }

        image.grid.append(imageRow)
    }
}

// Search for sea monsters

var seaMonsterCount = 0

loop:
for _ in 1...2 {
    for _ in 1...4 {
        for row in 0..<image.grid.count - seaMonsterHeight {
            for col in  0..<image.grid[row].count - seaMonsterWidth {
                var matches = 0

                for offset in seaMonsterMask {
                    if image.grid[row + offset.y][col + offset.x] == "#" { matches += 1 }
                }

                if matches == seaMonsterMask.count { seaMonsterCount += 1 }
            }
        }

        if seaMonsterCount > 0 { break loop }

        image.rotateLeft()
    }

    image.flipVertical()
}

result = image.grid.joined().reduce(0) { $1 == "#" ? $0 + 1 : $0 } - seaMonsterCount * seaMonsterMask.count

print("Result for puzzle 2: \(result)")
