# Pages

Pages are functions — returning some HTML — that are used as an endpoint on the server. In wini, this is defined as a function returning `Markup` (which is more or less, a `String` (more in the Maud chapter))

## Usage 
```rs
#[page]
async fn my_page() -> Markup {
    html! {
        h1 { "Hello world!" }
    }
} // Will return `<h1>Hello world!</h1>`
```

## About

Notice the use of the `#[page]` macro. If we were only sending back some basic String, we wouldn't need to use a procedural macro just for that.

The purpose of this macro is to include the Typescript and Scss files associated to this page, so, when it's sent back it's included with the appropriate JavaScript and CSS.

## Example

```
.
├── my_script_1.ts
├── my_script_2.ts
├── mod.rs
└── style.scss
```

Will send back in header:
```html
<head>
    ...
    <script src="/.../my_script_1.js"></script>
    <script src="/.../my_script_2.js"></script>
    <link rel="stylesheet" href="/.../style.css">
    ...
</head>
```

<div class="note">

This is the same thing with the `#[component]` and `#[layout]` macros that we wil see next. Tho, they don't do _exactly_ the same thing

</div>
