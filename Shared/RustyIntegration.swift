//
//  RustyIntegration.swift
//  Shared
//
//  Created by Quentin Mathé on 12/06/2022.
//

import Foundation
import Swift

// MARK: - Point

public typealias Point = RustyPoint

extension Point: Equatable {
    static let zero: Point = .init(x: 0, y: 0)
    
    func distance(to other: Point) -> Double {
        distance_to(self, other)
    }
    
    public static func == (left: Point, right: Point) -> Bool {
        left.x == right.x && left.y == right.y
    }
}

// MARK: - Polygon

class Polygon {
    let raw: OpaquePointer = polygon_new()
    var points: [Point] {
        get { 
            var len: UInt32 = 0 
            let pointer = polygon_points(raw, &len)
            guard len > 0 else { 
                return []
            }
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
    var description: String { 
        guard let cString = polygon_description(raw) else {
            return ""
        }
        let string = String(cString: UnsafePointer(cString))
        free_polygon_description(cString)
        return string
    }
    var length: Double { polygon_length(raw) }
    
    deinit {
        polygon_free(raw)
    }

    func add(_ point: Point) {
        polygon_push(raw, point)
    }
    
    func remove(at index: Int) {
        polygon_remove(raw, Int64(index))
    }
}

@_cdecl("point_equals")
func pointEquals(left: Point, right: Point) -> CBool { left == right }

// The returned string must deallocated on Rust side with libc::free().
@_cdecl("random_uuid_str")
func randomUUIDString() -> UnsafePointer<CChar> {
    UnsafePointer(strdup(UUID().uuidString)!)
}
