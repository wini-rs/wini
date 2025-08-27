# This script is in charge of linting files

source ./utils.nu

info "Compiling scss..."
nu ./scripts/scss.nu

info "=== Biome linter ==="
bunx biome lint

info "=== Rust clippy ==="
cargo clippy
