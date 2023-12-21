build:
	cargo build

release:
	cargo build --release

.PHONEY: build release
