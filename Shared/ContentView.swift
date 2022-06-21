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
            LengthView()
                .tabItem {
                    Text("Length")
                }
        }
        .padding(24)
        .frame(maxWidth: 600, maxHeight: 400)
    }
}
