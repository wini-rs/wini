# This script is in charge of formatting files

source ./utils.nu

bunx biome check --write
cargo +nightly fmt

try {
    cargo clippy --fix
} catch {
    let should_cargo_clippy = prompt_yesno "Directory is dirty. Do you still want to run `cargo clippy --fix` ? " 'n'

    if $should_cargo_clippy {
        cargo clippy --fix --allow-dirty
    }
}
