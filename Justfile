# Install dependencies
deps:
	rustup component add clippy
	rustup component add rustfmt
	rustup component add rls rust-analysis rust-src
	type cargo-readme >/dev/null || cargo +stable install cargo-readme

# Reformat the source code
fmt:
	cargo fmt

# Check for mistakes
lint:
	cargo clippy

# Generate the docs
doc:
	cargo doc

# Open the docs in a browser
doc-open: doc
	cargo doc --open

# Update README.md
readme:
	cargo readme -o README.md

# Publish to crates.io
publish:
	cargo publish
