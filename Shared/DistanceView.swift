//
//  PointView.swift
//  Shared
//
//  Created by Quentin Mathé on 12/06/2022.
//

import SwiftUI

public typealias Point = RustyPoint

extension Point: Equatable {
    static let zero: Point = .init(x: 0, y: 0)
    
    public static func == (left: Point, right: Point) -> Bool {
        left.x == right.x && left.y == right.y
    }
}

struct DistanceView: View {
    
    @State var a: Point = .zero
    @State var b: Point = .zero
    @State var result: Double = 0

    var body: some View {
        VStack {
            Text(result, format: .number)
                .font(.largeTitle)
                .padding()
            HStack {
                Form {
                    Section(header: Text("A")) {
                        TextField("X", value: $a.x, format: .number)
                        TextField("Y", value: $a.y, format: .number)
                    }
                }
                Form {
                    Section(header: Text("B")) {
                        TextField("X", value: $b.x, format: .number)
                        TextField("Y", value: $b.y, format: .number)
                    }
                }
            }
            .padding()
        }
        .onChange(of: a, perform: update)
        .onChange(of: b, perform: update)
        .padding()
    }
    
    func update(_ value: Point) {
        result = distance_to(a, b)
    }
}
