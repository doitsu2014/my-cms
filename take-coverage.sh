rustup toolchain add nightly --component llvm-tools-preview
rustup override set nightly
export RUSTC_BOOTSTRAP=1
export RUSTFLAGS='-Cinstrument-coverage'
export LLVM_PROFILE_FILE=my-cms-%p-%m.profraw
cargo clean
cargo test
# LCOV
# grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o lcov.info
# HTML
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
