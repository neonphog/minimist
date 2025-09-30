# minimist makefile

.PHONY: all test bump

all: test

test:
	cargo fmt -- --check
	cargo clippy --locked -- -D warnings
	RUSTFLAGS="-D warnings" cargo test --locked
	cargo rdme --force

bump:
	@if [ "$(ver)x" = "x" ]; then \
		echo "USAGE: make bump ver=0.1.1"; \
		exit 1; \
	fi
	sed -i 's/^\(version = "\)[^"]*"/\1$(ver)"/g' ./Cargo.toml
	cargo update
	$(MAKE) test
