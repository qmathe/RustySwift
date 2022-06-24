# Rust + Swift sur iOS et macOS

Présentation au Swift Meetup de Toulouse le 23/06/2202.

Le repo contient :
- les slides de présentation (Rust + Swit.pdf)
- un projet Xcode pour compiler le code de l'appli de démo

Pour tester la démo :
1) cloner le repo

    git clone https://github.com/qmathe/RustySwift.git

2) installer Rust et les targets de cross-compilation ARM et x86 pour iOS et macOS

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
   rustup target add aarch64-apple-darwin x86_64-apple-darwin

3) ouvrez RustySwift.xcodeproj, sélectionner un scheme (iOS et macOS) puis choisir _Run_

Note: le dernier exemple de la démo (LengthView.swift qui propose d'éditer une liste de points) n'est pas compilé sur iOS, parce que SwiftUI Table n'est disponible que sur macOS.

Pour créer, vos propre projets Swift et Rust, regardez la section *Rust dans Xcode* des slides qui détaille comment le projet Xcode est configuré pour la compilation d'une bibiothèque Rust statique. La bibliothèque statique est ensuite linkée par l'appli Swift.
