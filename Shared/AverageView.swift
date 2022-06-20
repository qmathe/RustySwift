//
//  AverageView.swift
//  Shared
//
//  Created by Quentin Mathé on 12/06/2022.
//

import SwiftUI

struct AverageView: View {
    
    @State var a: Double = 0
    @State var b: Double = 0
    @State var result: Double = 0

    var body: some View {
        VStack {
            Text(result, format: .number)
                .font(.largeTitle)
                .padding()
            HStack {
                TextField("A", value: $a, format: .number)
                TextField("A", value: $b, format: .number)
            }
            .padding()
        }
        .onChange(of: a, perform: update)
        .onChange(of: b, perform: update)
        .padding()
    }
    
    func update(_ value: Double) {
        result = average(a, b)
    }
}
