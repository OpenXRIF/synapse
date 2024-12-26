.PHONY run:
run:
	RUST_LOG=debug cargo run

.PHONY test:
test:
	RUST_BACKTRACE=1 cargo test
