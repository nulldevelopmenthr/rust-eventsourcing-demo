SHELL := /bin/bash # Use bash syntax

.PHONY=*


build:
	time -p cargo build

check:
	time -p cargo check

debug:
	RUST_BACKTRACE=1 time -p cargo run

format:
	time -p cargo fmt

run:
	time -p cargo run

test:
	time -p cargo test --tests

wip:
	git add . && git commit -am 'wip' && make build
