<div align="center">

<img alt="Wini logo" src="./assets/wini.svg" height="160"/>

# Wini

### A way to rethink front-end development ⚡

</div>

<table>
    <tr>
        <th>
            Page
        </th>
        <th>
            Layout / Middleware
        </th>
        <th>
            Component
        </th>
    </tr>
    <tr>
        <td>

```rs
#[page]
async fn page() -> Markup {
    html! {
        h1 {
            "My title"
        }
        main .bg-red {
            "Some content"
        }
    }
}
```

</td>
<td>

```rs
#[layout]
async fn layout(child: Markup) -> Markup {
    html! {
        header {
            "Welcome to Wini!"
        }
        (child)
    }
}
```

</td>
<td>

```rs
#[component]
async fn button() -> Markup {
    html! {
        button
            .btn-blue
            onclick="jsFn()"
        {
            "Blue button!"
        }
    }
}
```

</td>
    </tr>
</table>


## What is wini ?

Wini is a set of templates written in [Rust](https://www.rust-lang.org/) for building websites. Instead of using [WebAssembly](https://webassembly.org/) like other common Rust front-end frameworks, Wini templates rely on server-side rendering and, when needed, [TypeScript](https://www.typescriptlang.org/). Other options are also available, like [`htmx`](https://htmx.org/), [Alpine.js](https://alpinejs.dev/), and [`_hyperscript`](https://hyperscript.org/). See the integration docs for [`htmx`](https://wini.rocks/doc/htmx) and [Alpine.js](https://wini.rocks/doc/alpinejs).

The goal of wini is therefore to be **fast**, **lightweight**, and to handle the maximum of things that can be handled on the server, **server-side**.

Even tho wini doesn't use a JavaScript framework and rely purely on vanilla javascript, you still have the possibility of installing packages with [`bun`](https://bun.sh) and to use them as you would with any javascript front-end framework.

For more information about how wini works and how it handles logic such as pages, components, middleware, routing and layout: <https://wini.rocks>


## Getting started

### Installation

You can install wini with `cargo`

```sh
cargo install wini
```

or by downloading the binary (not recommended)

```sh
curl -fsSL https://wini.rocks/install | sh
```

> [!NOTE]
> You need to be root

### Create your first project

After the installation of `wini`, you can create a new project with

```sh
wini init
```

## Documentation

All the documentation concerning the inner workings of wini and how to work with it are available here: <https://wini.rocks/>.

## State 
Even tho wini works, it's still on a very early version: Things might not always work as expected and there might be some breaking change in the future. But if you are curious, and you want to help develop this project, you can still try it!
