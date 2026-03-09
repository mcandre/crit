.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-cargo \
	clean-crit \
	clean-example \
	clean-packages \
	clean-ports \
	clippy \
	crit \
	doc \
	install \
	lint \
	port \
	publish \
	rustfmt \
	test \
	uninstall
.IGNORE: \
	clean \
	clean-cargo \
	clean-crit \
	clean-example \
	clean-packages \
	clean-ports

VERSION=0.0.17
BANNER=crit-$(VERSION)

all: build

audit:
	cargo audit

build: lint test
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-cargo \
	clean-crit \
	clean-example \
	clean-packages \
	clean-ports

clean-cargo:
	cargo clean

clean-crit:
	crit -c

clean-example:
	rm -f example/Cargo.lock
	rm -rf example/target
	rm -rf example/.crit

clean-ports:
	rm -rf .crit/bin/crit-ports

clippy:
	cargo clippy

crit:
	crit -b $(BANNER)

doc:
	cargo doc

install:
	cargo install --force --path .

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt

port: crit
	./port -C .crit/bin -a crit $(BANNER)

publish:
	cargo publish

rustfmt:
	cargo fmt

test:
	cargo test

uninstall:
	cargo uninstall crit
