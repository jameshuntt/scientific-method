#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
cargo fmt --all
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
printf '\nscientific-method 0.2.0 verified\n'
