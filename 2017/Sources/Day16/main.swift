// ---------------------------------------------------
//  main.swift
//  Day 16: Permutation Promenade
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

enum Move {
    case spin(Int)
    case exchange(Int, Int)
    case partner(String, String)
}

let moves = input
    .split(separator: ",")
    .map {
        var parts = $0
            .split(separator: "/")
            .map { String($0) }

        switch parts[0].removeFirst() {
            case "s": return Move.spin(Int(parts[0])!)
            case "x": return Move.exchange(Int(parts[0])!, Int(parts[1])!)
            case "p": return Move.partner(parts[0], parts[1])
            default: fatalError()
        }
    }

func dance() {
    for move in moves {
        switch move {
            case let .spin(count):
                programs.insert(contentsOf: programs.suffix(count), at: 0)
                programs.removeLast(count)
            case let .exchange(idx1, idx2):
                programs.swapAt(idx1, idx2)
            case let .partner(s1, s2):
                programs.swapAt(programs.firstIndex(of: s1)!, programs.firstIndex(of: s2)!)
        }
    }
}


// ------------------------
//   MARK: - Puzzle 1 + 2
// ------------------------

var programs = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"]
var orders = [[String]]()
var oneBillionIndex = 0

for _ in 0..<1_000_000_000 {
    dance()

    if let firstOccurance = orders.firstIndex(of: programs) {
        let interval = orders.count - firstOccurance
        let cycles = 1_000_000_000 / interval

        oneBillionIndex = firstOccurance + 1_000_000_000 - cycles * interval - 1
        break
    }

    orders.append(programs)
}

print("Result for puzzle 1: \(orders[0].joined())")
print("Result for puzzle 2: \(orders[oneBillionIndex].joined())")
