// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Never push to remote with this flag set to true
let useLocalFramework = false
let releaseTag = "0.3.2"
let releaseChecksum = "29c4b57bbdaf485c609b8cdb9e5a1163171ad062c9b1602277cca058fbac7f9b"

let binaryTarget: Target = if useLocalFramework {
    .binaryTarget(
        name: "MiqatFFI",
        path: "./target/ios/libmiqat-rs.xcframework"
    )
} else {
    .binaryTarget(
        name: "MiqatFFI",
        url: "https://github.com/ibad-al-rahman/miqat/releases/download/\(releaseTag)/libmiqat-rs.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "Miqat",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "Miqat", targets: ["Miqat"]),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "Miqat",
            dependencies: [.target(name: "MiqatFFI")],
            path: "apple/Sources/Miqat",
            resources: [
                .process("Resources/PrivacyInfo.xcprivacy")
            ]
        )
    ]
)
