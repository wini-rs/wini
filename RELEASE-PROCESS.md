# Release process

1. Update the version in Cargo.toml

2. Update dependencies: `cargo update`

3. cd wini && cargo test && cd -

4. cd wini && cargo +nightly fmt  && cd -

5. cd wini && cargo clippy && cd -

6. git push

7. nix flake update
