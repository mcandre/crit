.POSIX:
.SILENT:
.PHONY: \
	all \
	crates \
	rustup-components

all: crates rustup-components

crates:
	cargo install --force \
		cargo-audit \
		tinyrick@0.0.15 \
		unmake@0.0.18
	cargo install --force cross --git https://github.com/cross-rs/cross --rev 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa

rustup-components:
	rustup component add \
		clippy \
		rustfmt
