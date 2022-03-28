.DEFAULT_GOAL := all

.PHONY: all
all: clean check lint test build

.PHONY: init
init:
	pre-commit install
	cargo install cargo-audit

.PHONY: clean
clean:
	rm -rf target

.PHONY: check
check:
	cargo check --bins --tests --benches --examples --all-features

.PHONY: lint
lint: fmt-check clippy

.PHONY: fmt-check
fmt-check:
	cargo fmt -- --check

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clippy
clippy:
	cargo clippy --bins --tests --benches --examples --all-features

.PHONY: test
test:
	cargo test --bins --tests --all-features
	cargo test --doc --all-features

.PHONY: build
build:
	cargo build --bins --tests --benches --examples --all-features

.PHONY: release
release:
	cargo build --bins --tests --benches --examples --all-features --release

.PHONY: audit
audit:
	cargo audit
