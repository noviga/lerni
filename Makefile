.PHONY: all clean doc doc-open examples fmt fmt-check linter prepare pre-commit serve test

all:
	@echo ──────────── Build release ────────────────────
	@cargo build --release

clean:
	@echo ──────────── Clean ────────────────────────────
	@rm -rvf target

doc:
	@echo ──────────── Build docs ───────────────────────
	@cargo doc --no-deps

doc-open:
	@echo ──────────── Build docs ───────────────────────
	@cargo doc --no-deps --open

examples:
	@echo ──────────── Build examples ───────────────────
	@./scripts/examples.sh

fmt:
	@echo ──────────── Format ───────────────────────────
	@cargo fmt --all

fmt-check:
	@echo ──────────── Check format ─────────────────────
	@cargo fmt --all -- --check

linter:
	@echo ──────────── Run linter ───────────────────────
	@cargo clippy --all-targets -- --no-deps -D warnings -A clippy::derive_partial_eq_without_eq

prepare:
	@echo ──────────── Install toolchains ───────────────
	@rustup target add wasm32-unknown-unknown
	@cargo install wasm-bindgen-cli

pre-commit: fmt linter test

serve: examples
	@cd dist && python3 -m http.server 8000

test: all
	@echo ──────────── Run tests ────────────────────────
	@cargo test --release
