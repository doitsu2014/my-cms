rustup toolchain add stable --component llvm-tools
rustup override set stable
export RUSTC_BOOTSTRAP=1
# export RUSTFLAGS='-Cinstrument-coverage -Ccodegen-units=1 -Cllvm-args=--inline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests'
export RUSTFLAGS='-Cinstrument-coverage -Zpanic_abort_tests'
export LLVM_PROFILE_FILE=test-llvm-reports/my-cms-%p-%m.profraw
export CARGO_INCREMENTAL=0
cargo test --all --all-features --no-fail-fast
# LCOV
grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o lcov.info
# HTML
# grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/