#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"

cargo fmt --check
cargo test
cargo test --all-features
cargo clippy --all-targets --all-features
cargo package --allow-dirty

echo
echo "scientific-method 0.1.0 verified"
