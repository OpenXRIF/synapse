.PHONY run:
run:
	RUST_LOG=debug cargo run

.PHONY test:
test:
	RUST_BACKTRACE=1 cargo test -- --nocapture

# TODO: Use seperate coverage library for Linux based deployments
.PHONY coverage:
coverage:
	cargo llvm-cov --html
	cargo llvm-cov report --lcov
	open target/llvm-cov/html/index.html

.PHONY coverage-install:
coverage-install:
	cargo install cargo-llvm-cov

.PHONY coverage-clean:
coverage-clean:
	cargo llvm-cov clean --workspace
