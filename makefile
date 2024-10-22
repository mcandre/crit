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
		cross@0.2.5 \
		tinyrick@0.0.14 \
		unmake@0.0.16

rustup-components:
	rustup component add \
		clippy \
		rustfmt
