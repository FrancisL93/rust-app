BACKEND=backend

all: rust

rust:
	cd $(BACKEND) && cargo-watch -x 'run -- -release'
.PHONY: rust


cargo-watch:
	cargo install cargo-watch
.PHONY: cargo-watch