#!/usr/bin/env sh

rustup default stable && rustup component add rustfmt && rustup component add clippy && cargo install diesel_cli --no-default-features --features postgres && cd shop && diesel migration run