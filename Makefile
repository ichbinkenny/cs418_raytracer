PHONY: all

build:
	cargo build

run: 
	cargo run -- $(file)

all:
	make build && make run file=$(file)