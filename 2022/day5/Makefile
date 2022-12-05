.PHONY: build

PROG=day5

run: build
	target/release/$(PROG) input.txt

build:
	cargo build --release

clean:
	cargo clean || rm -rf target/
