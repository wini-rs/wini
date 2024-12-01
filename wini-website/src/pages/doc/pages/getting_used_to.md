# Getting used to the wini-cli

## Create a new page

You can easily create a page, a component or a layout by using

```sh
wini new page # or component or layout
```

## Update the original wini-template

When you first created your project, you used the latest version of a wini template.

But now, this version is maybe outdated. In this case, you should use:

```sh
wini update-template
```



## Capabilities of wini CLI

All the commands in wini (excepted `init`) are a mapping of [`just`](https://github.com/casey/just).

`just` is a command runner tool and a modern alternative to [`make`](https://www.gnu.org/software/make/). You can see the list of all the commands premade by either looking at `./justfile` or by doing

```sh
just -l # which is more or less the same as `wini help`
```

Since all the commands are made in `./justfile`, you can therefore edit and update the justfile to run custom commands that will be directly updated when you run `wini`.

You can even see what `wini new`, `wini run` and all the commands are doing by looking at `./justfile` and the `./scripts` directory.
