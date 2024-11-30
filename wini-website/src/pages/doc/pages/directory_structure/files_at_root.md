# Files at the root

First of all, we'll cover the files at the root of the project


## biome.json

[biome](https://biomejs.dev/) is toolchain used to format javascript and typescript code (written in Rust). It's a modern alternative to [prettier](https://prettier.io/).

This file is therefore, a configuration of the TypeScript formatter.


## bun.lockb

A binary lockfile of all the javascript packages managed by [`bun`](https://bun.sh/). More information at <https://bun.sh/docs/install/lockfile>.


## Cargo.toml

The cargo configuration file of the current project. More information at <https://doc.rust-lang.org/cargo/reference/manifest.html>.


## flake.nix

The [nix](https://nixos.org) [flake](https://nixos.wiki/wiki/flakes) of this project. When creating a new project, it's just here to create the default shell environment when using `wini env`.


## justfile

The file specifying commands for [`just`](https://github.com/casey/just). For more information refer to the previous chapter: [Getting used to wini-cli](https://wini.rocks/doc/getting_used_to).


## packages.json

The file specifying the javascript packages used in your website. More information at <https://docs.npmjs.com/cli/configuring-npm/package-json>


## packages-files.toml

_This file is specific to wini._

When you install a javascript package with `wini js-add mypackage`, what you are essentially doing is clonning the repository behind `mypackage`.

This package is after that put in the infamous directory called `node_modules/`.

Therefore, a legitimate question is:

When you are using a package in one of your script, which files should it includes ? And this is exactly what `packages-files.toml` tries to solve: specifying which files should be included when using a pacakge. This can be javascript files like stylesheets.

Most of the times, you won't have to touch this file because the question of which files to include will be asked to you when using `wini js-add`


## README.md 

A brief introduction to "how do X in the project ?"


## rustfmt.toml

[Rustfmt](https://github.com/rust-lang/rustfmt) is the default Rust formatter. Therefore, this file is just the configuration of the formatter.


## tsconfig.json

A configuration file specifying how the typescript should be compiled. More information at <https://www.typescriptlang.org/docs/handbook/tsconfig-json.html>


## wini.toml

_This file is specific to wini._

`wini.toml` symbolizes the root of a wini project. It has 3 different sections:

- origin: Specifying the template repository on which it's based on.
- path: Specifying the name of the different core directories used in wini
- cache: Specifying the different cache rules for different file types and environments

