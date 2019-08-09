# Check with clippy.
clippy:
	cargo clippy

# Generate documentation.
doc:
	cargo doc

# Generate README.md from src/lib.rs.
readme:
	cargo readme -o README.md
