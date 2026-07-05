.PHONY: test release

test:
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	npm pack --dry-run

release:
	@scripts/release.sh $(VERSION)
