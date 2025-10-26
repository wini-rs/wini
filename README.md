<div align="center">

<img alt="Wini logo" src="./assets/wini.svg" height="160"/>

# Wini


<!-- Version -->
<a href="https://github.com/wini-rs/wini/releases">
    <img src="https://img.shields.io/github/v/release/wini-rs/wini" alt="Release" />
</a>

<!-- CI -->
<a href="https://github.com/wini-rs/wini/actions">
    <img src="https://github.com/wini-rs/wini/actions/workflows/check.yml/badge.svg" alt="CI status" />
</a>

<!-- Doc -->
<a href="https://wini.rocks">
    <img src="https://img.shields.io/badge/Docs-wini.rocks-blue" alt="Documentation" />
</a>


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


## 🤔 What is `wini` ?

Wini is a set of templates written in [Rust](https://www.rust-lang.org/) for building websites. Instead of using [WebAssembly](https://webassembly.org/) like other common Rust front-end frameworks, Wini templates rely on server-side rendering and when needed [TypeScript](https://www.typescriptlang.org/). Other options are also available, like [`htmx`](https://htmx.org/), [Alpine.js](https://alpinejs.dev/), and [`_hyperscript`](https://hyperscript.org/). (_See the doc for [`htmx`](https://wini.rocks/doc/htmx) and [Alpine.js](https://wini.rocks/doc/alpinejs)._)

The goal of Wini is therefore to be **fast** ⚡, **lightweight** 🪶 and **server-side** 🌐 oriented.

Even though Wini doesn't use a JavaScript framework and relies purely on vanilla JavaScript, you still have the possibility of installing packages with [`bun`](https://bun.sh) and using them as static files.

## ✨ Features
- [Server side rendering](https://developer.mozilla.org/en-US/docs/Glossary/SSR) (SSR)
- [Static site generation](https://developer.mozilla.org/en-US/docs/Glossary/SSG) (SSG)
- Creation of concepts like [_pages_](https://wini.rocks/doc/concepts/pages), [_components_](https://wini.rocks/doc/concepts/components) and [_layouts_](https://wini.rocks/doc/concepts/layouts)
- Compatible with [axum](https://docs.rs/axum)'s & [tower](https://docs.rs/tower)'s ecosystems
- Compatible with [htmx](https://htmx.org), [alpinejs](https://alpinejs.dev), [hyperscript](https://hyperscript.org) and similar frameworks
- [SEO](https://en.wikipedia.org/wiki/Search_engine_optimization) scoping to pages and layouts
- [Caching](https://en.wikipedia.org/wiki/Cache_(computing)) rules for [`Cache-` headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Caching) and _components_ + _pages_ ([`cached`](http://docs.rs/cached))
- Automatic linking of JavaScript and CSS files
- Default support for [TypeScript](https://www.typescriptlang.org/) and [Scss](https://sass-lang.com)
- Advanced [error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) and error propagation
- Basic scripting automation in bash or [nushell](https://nushell.org) depending on your liking
- Integration with [maud](https://maud.lambda.xyz) for rendering HTML (see [the advantages](https://wini.rocks/doc/why-maud))
- Easily [forkable and self-hostability](https://wini.rocks/doc/hosting_your_template) of your own template

## 🏁 Getting started

### Installation

You can install Wini with `cargo`

```sh
cargo install wini
```

### Create your first Wini project

After the installation of `wini`, you can create a new project with

```sh
wini init
```

## 📄 Documentation

All the documentation concerning the inner workings of `wini` and how to work with it are available at: <https://wini.rocks/>.

## 🚧 State 
Even though Wini works, it's still in a very early state: Things might not always work as expected and there might be some breaking changes in the future; but if you are curious, and you want to help contribute to this project, you can still try it!
