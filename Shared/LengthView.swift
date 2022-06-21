//
//  LengthView.swift
//  Shared
//
//  Created by Quentin Mathé on 12/06/2022.
//

import SwiftUI

struct LengthView: View {
    
    struct Item: Identifiable {
        let id: Int
        let points: Binding<[Point]>
        var point: Binding<Point> { points[id] }
    }

    @State var polygon = Polygon()
    @State var points: [Point] = []
    var items: [Item] {
        $points.enumerated().map { Item(id: $0.offset, points: $points) }
    }
    @State var length: Double = 0
    @State var selection: Int?

    var body: some View {
        VStack {
            Text(length, format: .number)
                .font(.largeTitle)
                .padding()
            Table(items, selection: $selection) {
                TableColumn("X") {
                    TextField("X", value: $0.point.x, format: .number)
                }
                TableColumn("Y") {
                    TextField("Y", value: $0.point.y, format: .number)
                }
            }
            .tableStyle(.inset(alternatesRowBackgrounds: true))
            .padding()
            HStack {
                Button(action: {
                    points.append(Point(x: 0, y: 0))
                }, label: {
                    Image(systemName: "plus")
                })
                Button(action: {
                    guard let index = selection else { return }
                    points.remove(at: index)
                }, label: {
                    Image(systemName: "minus")
                })
                .disabled(selection == nil)
            }
        }
        .onChange(of: points) { newPoints in
            polygon.points = newPoints
            length = polygon.length
        }
        .padding()
    }
}
