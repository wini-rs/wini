# Release process

1. Update the version in Cargo.toml

2. Update dependencies: `cargo update`

3. cargo test

4. cargo +nightly fmt

5. cargo clippy

6. git tag -a vMAJOR.MID.MINOR -m "wini-cli: v"

7. git push

8. cargo publish
