#!/usr/bin/env sh
git config --global --add safe.directory /workspaces
rustup default stable && \
rustup component add rustfmt && \
rustup component add clippy && \
cargo install diesel_cli --no-default-features --features postgres && \
cd shop && \
diesel migration run && \
cargo build