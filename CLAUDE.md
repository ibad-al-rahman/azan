# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Miqat is a Rust-based Islamic prayer time calculation library. It computes prayer times (Fajr, Sunrise, Dhuhr, Asr, Maghrib, Ishaa, Qiyam) using high-precision astronomical equations from Jean Meeus's "Astronomical Algorithms". It targets iOS, macOS, and Android via auto-generated bindings using UniFFI.

## Commands

### Rust

```bash
cargo build                    # Build all workspace crates
cargo test                     # Run all tests
cargo test <test_name>         # Run a single test by name
cargo clippy                   # Lint
cargo fmt                      # Format
```

### Apple (iOS/macOS)

```bash
just apple                     # Full debug build pipeline
just apple-build               # Build Rust library for all Apple targets
just apple-generate-ffi        # Generate Swift FFI bindings
just apple-build-xcframework   # Create XCFramework
```

### Android

```bash
just android                   # Full debug build pipeline
just android-build             # Build using Gradle
```

### Utilities

```bash
just clean-all                 # Clean all build artifacts
just update-versions X.Y.Z    # Update version across all manifests
```

## Architecture

The project is a Rust workspace with three layers:

### 1. `miqat_core` — Pure Rust calculation library

- `astronomy/` — Solar coordinates, astronomical operations (Meeus equations), Qiblah
- `models/` — Configuration types: `Method` (6 regional presets), `Mazhab` (Hanafi/Shafi), `Parameters`, `Prayer` enum, high-latitude rules, rounding, adjustments
- `prayer_times.rs` — Main `PrayerTimes` struct; calculates all prayer times, current/next prayer, and time remaining

### 2. `miqat_rslib` — FFI wrapper

Wraps `miqat_core` using UniFFI to expose a cross-platform API. Outputs times as millisecond timestamps (`i64`). Built as `cdylib`, `staticlib`, and `lib`.

### 3. `uniffi-bindgen` — Binding generator

Thin binary wrapping `uniffi_bindgen_main()` that auto-generates Swift and Kotlin bindings from the annotated Rust API.

### Platform outputs

- **iOS/macOS**: Auto-generated `apple/Sources/Miqat/MiqatBindings.swift` + XCFramework, consumed via Swift Package Manager (`Package.swift`), requires iOS 13+
- **Android**: Auto-generated `android/miqat/src/main/java/org/ibadalrahman/miqat/miqat.kt` + AAR via Gradle with JNA

### Binding generation pattern

Adding or changing public API in `miqat_rslib` requires re-running `just apple-generate-ffi` (and the Android equivalent) to regenerate the platform binding files. The generated files (`MiqatBindings.swift`, `miqat.kt`) are committed to the repo and should not be hand-edited.
