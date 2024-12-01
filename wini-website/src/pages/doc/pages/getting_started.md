# Getting started

## Installation

To start a new project based on a wini template, you will first need to install the wini command line tool.

```
cargo install wini
```

_**Info**: This tool essentially is in charge of initializing the project and also acts as an interface with [`just`](https://github.com/casey/just) (the command runner used by default with wini projects)_

## Creating a new project

Once installed, you can create a project with

```
wini init # `wini new` is used for something else
```

It will ask you to create a project using an interactive prompt like:

```sh
┌───────────────────────────────────┐
│ Welcome to your new Wini project! │
◆ ──────────────────────────────────┘
│
◇ Create a project from: Official wini templates
│
◇ Which template should be used: Basic
│
◇ Project name: hello_wini
│
◆ Project created at `./hello_wini`!
```
and... that's it! Your project is created!

## Running the project

Go in the project directory that has just been created (in our case: `cd ./hello_wini`).

Next, you can use
```sh
wini env
```
This will call `nix develop` to install the dependencies needed. For that you therefore need to have [nix](https://nixos.org/learn/) installed. Else, you can see which system dependencies you should install by looking at the `buildInputs` in `./flake.nix`.

<div class="note">

If you don't use `wini env`, you will have to select the nightly toolchain for rust manually

</div>

Once in your development environment, the last command you need to enter is:
```sh
wini run
```

and that's it! Your project is running! Congrats!
