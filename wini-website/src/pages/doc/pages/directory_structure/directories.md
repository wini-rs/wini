# Directories

Now that we covered the basic files, we will cover the different directories that you can find and their purpose

## macros

This directory contains all the [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) specific to wini. (`#[layout]`, `#[page]`, ...) This is just a cargo project that exports procedural macro.

They are in a folder and not a crate so you can easily customize them if you want to! And if you don't want to customize them, they will be updated when `wini update-template`, without conflicts!


## public

The default public directory. A public directory is a directory made to serve static files, like `favicon` or some images.

## public/helpers.js

A javascript file that contains helper functions inspired by jQuery that makes you win a lot of time (typed in `public/helpers.g.ts`)

## scripts

The directory where all the scripts used by `just` are. You can also modify them as your need very easily!

## target

The directory where cargo store the cache. More info at: <https://doc.rust-lang.org/cargo/reference/build-cache.html>

## tests

The directory where you can write tests for your lib

## src

The `src/` directory has a lot of content in it! And therefore, it has it's own page, which is just the next page!

