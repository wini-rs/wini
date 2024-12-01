# Introduction

This Rust book is a book explaining the workings of [Wini](https://github.com/wini-rs/wini). The source code for this book is available [here](https://github.com/wini-rs/wini/tree/main/wini-website), if you see any typos or possible improvements, feel free to make pull requests!

This book assumes that you have basic knowledge in [Rust](https://www.rust-lang.org/) and Web development.

<div class="note">

If you want to learn Rust but don't know where to start, the [rust book](https://doc.rust-lang.org/stable/book/) is a good start.

</div>

## So, what is Wini ?

Wini is a set of templates written (mostly) in Rust. Template in the sense, that wini - at it's core - is just files. Files that the user can freely overwrite and modify as they want. Therefore, a project based on wini, is just a clone (in the sens of git) of a template repository.

Wini comes with a set of useful features already made such as pages, middleware/layout, components and caching. Wini is also based on the [axum](https://github.com/tokio-rs/axum) framework, so you can easily customize it and use the large ecosystem of middleware of [tower](https://github.com/tower-rs/tower).

Once a project is created from one of the wini templates, the update to the original template can be managed with git. Don't worry, there is the required tooling to automatically pull the latest changes!

## What makes Wini so unique ?

Wini has an _old-school_ approach of seing front-end: Front-end is just back-end sending HTML, CSS and web-related files. Which is what all front-end frameworks are essentially doing, but they cover that from the developer.

Instead of using [WebAssembly](https://webassembly.org/) like [Dioxus](https://dioxuslabs.com/), [Leptos](https://leptos.dev/), [Yew](https://yew.rs/), [Sycamore](https://sycamore.dev/) and others, Wini uses JavaScript transpiled from [Typescript](https://www.typescriptlang.org/). Why ? Because JavaScript is more lightweight (in term of bytes transfered), supported and has a larger ecosystem than current WebAssembly.

Instead of using JavaScript for everything, Wini allows the user to do everything server-side in Rust, and when you need to use JavaScript in last resort, you can create a TypeScript file.
But, JavaScript should only be a minimal part of a website, since most of the things can and should be rendered on the server.

Wini also uses the incredibly useful and elegant [maud](https://maud.lambda.xyz/) crate that allow you to describe [DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model) in a syntax way more elegant than some [SGML](https://en.wikipedia.org/wiki/Standard_Generalized_Markup_Language)-inspired syntax:

### Maud
```css
html {
    h1 { "Hello, world!" }
    p.intro {
        "This is an example of the "
        a href="https://github.com/lambda-fairy/maud" { "Maud" }
        " template language."
    }
}
```

### HTML
```html
<html>
    <h1> Hello, world! </h1>
    <p class="intro">
        This is an example of the 
        <a href="https://github.com/lambda-fairy/maud"> Maud </a>
        template language.
    </p>
</html>
```

_Source: <https://maud.lambda.xyz/>_
