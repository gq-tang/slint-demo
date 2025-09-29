.PHONY: run

APP ?= user
run:
	cargo run -p $(APP)

build:
	cargo build -p $(APP)--release