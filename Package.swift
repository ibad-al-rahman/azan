// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Never push to remote with this flag set to true
let useLocalFramework = false
let releaseTag = "0.3.2"
let releaseChecksum = "29c4b57bbdaf485c609b8cdb9e5a1163171ad062c9b1602277cca058fbac7f9b"

let binaryTarget: Target = if useLocalFramework {
    .binaryTarget(
        name: "AzanFFI",
        path: "./target/ios/libazan-rs.xcframework"
    )
} else {
    .binaryTarget(
        name: "AzanFFI",
        url: "https://github.com/ibad-al-rahman/azan/releases/download/\(releaseTag)/libazan-rs.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "Azan",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "Azan", targets: ["Azan"]),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "Azan",
            dependencies: [.target(name: "AzanFFI")],
            path: "apple/Sources/Azan",
            resources: [
                .process("Resources/PrivacyInfo.xcprivacy")
            ]
        )
    ]
)
