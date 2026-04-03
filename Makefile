# =============================================
# RUST-V Makefile - April Fools 2026 Edition 🦀
# "crab, run, crash" — the official build system
# =============================================

BINARY = rustv
SOURCE = rustv.rs

.PHONY: all build run clean

all: build run

build: $(BINARY)

$(BINARY): $(SOURCE)
    @echo "🦀 Compiling RUST-V with rustc..."
    rustc --edition=2021 $(SOURCE) -o $@

run: build
    @echo "🚀 Running the first RISC-V processor fully designed in Rust..."
    ./$(BINARY)

clean:
    rm -f $(BINARY)
    @echo "🧹 Cleaned up the binary. Reality restored."

# Bonus: one command to rule them all
everything: clean build run
