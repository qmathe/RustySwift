//
//  RustyIntegration.swift
//  Shared
//
//  Created by Quentin Mathé on 12/06/2022.
//

import Swift

// MARK: - Point

public typealias Point = RustyPoint

extension Point: Equatable {
    static let zero: Point = .init(x: 0, y: 0)
    
    public static func == (left: Point, right: Point) -> Bool {
        left.x == right.x && left.y == right.y
    }
}

// MARK: - Polygon

struct Polygon {
    let raw: OpaquePointer = polygon_new()
    var points: [Point] {
        get { 
            var len: UInt32 = 0 
            let pointer = polygon_points(raw, &len)
            let points = Array(UnsafeBufferPointer(start: pointer, count: Int(len)))
            free_points(pointer)
            return points
        }
        set {
            newValue.withUnsafeBufferPointer {
                guard let pointer = $0.baseAddress else { 
                    return
                }
                polygon_set_points(raw, pointer, UInt32(newValue.count))
            }
        }
    }
    var length: Double { polygon_length(raw) }
    
    func add(_ point: Point) {
        polygon_push(raw, point)
    }
    
    func remove(at index: Int) {
        polygon_remove(raw, Int64(index))
    }
}
