# https://blog.rng0.io/how-to-do-code-coverage-in-rust

# Test
CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="target/cargo-test.profraw" cargo test

# Coverage
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore "../*" -o docs/coverage

# Documentation
cargo doc && cp -r ./target/doc/. ./docs
