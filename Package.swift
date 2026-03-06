// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Never push to remote with this flag set to true
let useLocalFramework = false
let releaseTag = "0.1.0"
let releaseChecksum = "72bb4b61c21125c6d5476ceb56601fd763a3b49edae81c54f219f51821af373e"

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
