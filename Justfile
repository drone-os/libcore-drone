# Check with clippy.
clippy:
	cargo clippy --all

# Generate README.md from src/lib.rs.
readme:
	cargo readme -o README.md
