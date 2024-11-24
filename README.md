<div align="center">

<img alt="Wini logo" src="./assets/wini.svg" height="160"/>

# Wini

### A way to rethink front-end development ⚡

</div>

## What is wini ?

Wini is a set of templates written in Rust, that are made to create websites. Instead of using [WebAssembly](https://webassembly.org/) like other common Rust front-end frameworks, Wini templates rely on server side rendering, and, when needed, [Typescript](https://www.typescriptlang.org/).

The goal of wini is therefore to be fast, lightweight, and to handle the maximum of things that can be handled on the server, server-side.

Even tho wini doesn't use a JavaScript framework and rely purely on vanilla javascript, you still have the possibility of installing packages with [`bun`](https://bun.sh) and to use them as you would with any javascript front-end framework.

For more info about how wini works, and how it handles logic such as pages, components, middleware, routing and layout: <https://wini.rocks>


## Getting started

### Installation

You can install wini with `cargo`

```sh
cargo install wini
```

or by downloading the binary (not recommended)

```sh
curl -fsSL https://wini.rocks | sh
```

_Note: you need to be root_

### Create your first project

After the installation of `wini`, you can create a new project with

```sh
wini init
```

## Documentation

TODO
