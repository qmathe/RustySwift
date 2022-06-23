//
//  ContentView.swift
//  Shared
//
//  Created by Quentin Mathé on 12/06/2022.
//

import SwiftUI

struct ContentView: View {

    var body: some View {
        TabView {
            AverageView()
                .tabItem {
                    Text("Average")
                }
            DistanceView()
                .tabItem {
                    Text("Distance")
                }
            #if os(macOS)
            LengthView()
                .tabItem {
                    Text("Length")
                }
            #endif
        }
        .padding(24)
        .frame(maxWidth: 600, maxHeight: 400)
    }
}
