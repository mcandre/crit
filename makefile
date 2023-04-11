.PHONY: all

all:
	@rustup component add clippy rustfmt
	@cargo install --force cargo-audit@0.17.5
	@cargo install --force tinyrick@0.0.9
	@cargo install --force unmake@0.0.3
