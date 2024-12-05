#!/usr/bin/env sh
apt update && \
apt install cmake -y && \
git config --global --add safe.directory /workspaces
rustup install nightly && \
rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu && \
rustup default stable && \
rustup component add rustfmt && \
rustup component add clippy && \
cargo install diesel_cli --no-default-features --features postgres && \
cd shop && \
diesel migration run && \
cargo build
