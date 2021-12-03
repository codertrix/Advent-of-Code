// ---------------------------------------------------
//  main.swift
//  Day 24: Lobby Layout
//
//  Copyright © 2020 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

import Foundation

let tileList = input.components(separatedBy: .newlines).map { Array($0) }

struct Tile: Hashable {
    var x = 0
    var y = 0
    var z = 0

    var neighbors: Set<Tile> {
        var tiles = Set<Tile>()

        for (dx, dy, dz) in [(0, 1, -1), (0, -1, 1), (1, 0, -1), (-1, 0,1), (1, -1, 0), (-1, 1, 0)] {
            tiles.insert(Tile(x: x + dx, y: y + dy, z: z + dz))
        }

        return tiles
    }
}


// --------------------
//   MARK: - Puzzle 1
// --------------------

var blackTiles = Set<Tile>()

for toFlip in tileList {
    var tile = Tile()
    var idx = 0

    while idx < toFlip.count {
        switch toFlip[idx] {
            case "e":
                tile.x += 1
                tile.y -= 1
            case "s":
                idx += 1
                if toFlip[idx] == "e" {
                    tile.y -= 1
                    tile.z += 1
                } else {
                    tile.x -= 1
                    tile.z += 1
                }
            case "w":
                tile.x -= 1
                tile.y += 1
            case "n":
                idx += 1
                if toFlip[idx] == "w" {
                    tile.y += 1
                    tile.z -= 1
                } else {
                    tile.x += 1
                    tile.z -= 1
                }
            default: break
        }

        idx += 1
    }

    if blackTiles.contains(tile) {
        blackTiles.remove(tile)
    } else {
        blackTiles.insert(tile)
    }
}

print("Result for puzzle 1: \(blackTiles.count)")


// --------------------
//   MARK: - Puzzle 2
// --------------------

for _ in 1...100 {
    var newBlackTiles = Set<Tile>()
    var tilesToCheck = Set<Tile>()

    for tile in blackTiles {
        tilesToCheck.insert(tile)
        tile.neighbors.forEach { tilesToCheck.insert($0) }
    }

    for tile in tilesToCheck {
        var blackNeighborsCount = 0
        tile.neighbors.forEach { if blackTiles.contains($0) { blackNeighborsCount += 1 } }

        if blackTiles.contains(tile) {
            if blackNeighborsCount == 1 || blackNeighborsCount == 2 { newBlackTiles.insert(tile) }
        } else {
            if blackNeighborsCount == 2 { newBlackTiles.insert(tile) }
        }
    }

    blackTiles = newBlackTiles
}

print("Result for puzzle 2: \(blackTiles.count)")
