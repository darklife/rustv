# Makefile for RUST-V simulator (single-file Rust toy)
# Just run: make

BINARY = rustv
SOURCE = rustv.rs

.PHONY: all build run clean

all: build

build: $(BINARY)

$(BINARY): $(SOURCE)
    rustc $(SOURCE) -o $@

run: build
    ./$(BINARY)

clean:
    rm -f $(BINARY)

# Optional: quick test
test: run
