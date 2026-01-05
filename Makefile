.PHONY: build run test clean fmt lint build-all build-linux build-macos build-windows install-targets clean-dist

# Define targets
TARGET_LINUX_GNU = x86_64-unknown-linux-gnu
TARGET_MACOS_INTEL = x86_64-apple-darwin
TARGET_MACOS_ARM = aarch64-apple-darwin
TARGET_WIN_GNU = x86_64-pc-windows-gnu

# Output directory
DIST_DIR = dist

build:
	cargo build --release

run:
	cargo run -- $(ARGS)

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy

clean:
	cargo clean
	rm -rf $(DIST_DIR)

# Build All
build-all: build-linux build-macos build-windows

# Linux
build-linux:
	cargo build --release --target $(TARGET_LINUX_GNU)
	mkdir -p $(DIST_DIR)
	cp target/$(TARGET_LINUX_GNU)/release/air $(DIST_DIR)/air-linux-amd64

# MacOS
build-macos:
	cargo build --release --target $(TARGET_MACOS_INTEL)
	cargo build --release --target $(TARGET_MACOS_ARM)
	mkdir -p $(DIST_DIR)
	cp target/$(TARGET_MACOS_INTEL)/release/air $(DIST_DIR)/air-darwin-amd64
	cp target/$(TARGET_MACOS_ARM)/release/air $(DIST_DIR)/air-darwin-arm64

# Windows
build-windows:
	cargo build --release --target $(TARGET_WIN_GNU)
	mkdir -p $(DIST_DIR)
	cp target/$(TARGET_WIN_GNU)/release/air.exe $(DIST_DIR)/air-windows-amd64.exe

# Auxiliary
install-targets:
	rustup target add $(TARGET_LINUX_GNU) $(TARGET_MACOS_INTEL) $(TARGET_MACOS_ARM) $(TARGET_WIN_GNU)

clean-dist:
	rm -rf $(DIST_DIR)