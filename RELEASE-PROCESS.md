# Release process

1. Update the version in Cargo.toml

2. Update dependencies: `cargo update`

3. cd wini && cargo test && cd -

4. cd wini && cargo +nightly fmt  && cd -

5. cd wini && cargo clippy && cd -

6. nix flake update

7. git tag -a vMAJOR.MID.MINOR -m "wini: v"

8. git push
