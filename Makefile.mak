build:
    cargo build

run:
    cargo run -- --file "test.txt" --print

help:
    cargo run -- --help

test:
    cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy
