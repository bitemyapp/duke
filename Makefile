build:
	cargo build

test:
	cargo test -- --nocapture

test-debug:
	RUST_LOG=duke=debug cargo test -- --nocapture

fmt:
	cargo fmt

rustfix:
	rustfix
