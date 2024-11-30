# The `src` directory

When creating a new project with `cargo`, the `src/` directory is the directory where all the rust code should be written (there are exceptions like `build.rs`, but this is more of an exception).

Here is the basic overview of `src/`

```
src
├── components/
├── cron/
├── layout/
├── lib.rs
├── main.rs
├── pages/
├── server.rs
├── shared/
├── template/
└── utils/
```

Even tho, most of the name are self explicit, we'll go throught each file

## components/

The definition of components used in the project


## cron/

A cron is a scheduling tool that allows you to run functions automatically at specified intervals.
Think of it as a way to automate repetitive tasks without needing to manually execute them each time.

Therefore, this directory is just a place where you can put your crons if you have some. If you don't need it, you can just remove the directory and remove it from `lib.rs`


## layout/

The definition of layouts used in the project


## lib.rs

`lib.rs` contains the core functionality and public API of the project.

It allows you to organize your code into modules and provides reusable components that can be imported by other crates or the executable.


## main.rs

`main.rs` contains the main function, which is the starting point of the executable.

It can call functions and use types defined in lib.rs, enabling the executable to leverage the library's functionality.

Having both a `lib.rs` and a `main.rs` promotes code reuse, as the library can be tested independently (like in `tests/`) and used in multiple projects, while the executable can focus on application-specific logic.


## pages/

The definition of pages used in the project


## server.rs

The definition of the server that will be created when launching the project


## shared/

This directory contains all the structs or const/static that are meant to be shared in the project


## template/

This directory contains the default HTML template that should be sent when rendering an HTML page. This is kind of the "Master layout".


## utils/

This directory contains all the utilities functions/macros that might come in handy in the project
