.PHONY: format lint test ci

format:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features

test:
	cargo test

ci: format lint test

