# Release process

1. Update the version in Cargo.toml
Remove the `path` for `wini-maud-macros`

2. Update Cargo.toml of wini-maud:
Remove the `path` for `wini-maud-macros`

3. Update dependencies: `cargo update`

4. cargo publish
