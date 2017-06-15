# iOS + Rust

Tested with Rust 1.18.0 and Xcode 8.3.3. Before building in Xcode, from the repository root, run:

```
# Add the tier-2 iOS platform targets
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios

# Install cargo-lipo (https://crates.io/crates/cargo-lipo)
cargo install cargo-lipo

# Build the Rust library
cargo lipo --release

# Symlink the built library to where the iOS app can find it
ln -s "../target/universal/release/librust.a" Library/librust.a
```

Then open the Xcode project, and run.

_NB. The app is so minimal that it doesn’t even have a run loop. It will launch, print to the console (the `test` function from the Rust library), and be terminated._
