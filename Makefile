build:
	cargo build

build-watch:
	cargo watch -x build

test:
	cargo test -- --nocapture

test-debug:
	RUST_BACKTRACE=1 RUST_LOG=duke=debug cargo test -- --nocapture

fmt:
	cargo fmt

rustfix:
	rustfix
