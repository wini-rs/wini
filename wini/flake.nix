{
    description = "Wini flake";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        systems.url = "github:nix-systems/default";
        rust-overlay.url = "github:oxalica/rust-overlay";
        flake-utils.url  = "github:numtide/flake-utils";
    };

    outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
        flake-utils.lib.eachDefaultSystem (system:
        let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
                inherit system overlays;
            };
        in
        {
            devShells.default = with pkgs; mkShell {
                buildInputs = [
# IFFEAT posix-sh
                    watchexec
                    yq-go
# ENDIF
                    coreutils
                    bun
                    dart-sass
                    fd
                    ripgrep
                    (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
                    delta
# IFFEAT nushell
                    nushell
# ENDIF
                ];
            };
        }
    );
}
