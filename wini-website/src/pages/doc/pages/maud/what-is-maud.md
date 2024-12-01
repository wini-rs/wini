# What is maud ?
Maud is a Rust library that allows users to render HTML by using a syntax more readable and lightweight than the one of HTML.

Maud is also really fast, because the procedural macro `html!`, transforms this abstracted way of representing the DOM into a String at compilation time.

## Quick tutorial

_If you want a complete tutorial, I encourage you to read <https://maud.lambda-fairy.xyz/> which goes in depth of the complete syntax of Maud and is really a great ressource_

### Element declaration

In maud, you can declare an element with the following syntax:

```rs
element {}  // <=> <element></element>
element;  // <=> <element>
```

The second one is typically used when using an image or a meta tag for example.

<div class="note">

Using `element;` won't declare an empty `script` (`<script></script>`).

It will create a `<script>` without closing, and therefore, lead to an error. So the correct way of doing it is: `script {}`

</div>

### Attributes

The syntax of maud is really inspired by the one of CSS. So, you can declare an id and a class with '#' and '.' respectively:

```css
div #my-id .my-class-1 .my-class-2 {}
```

To declare some special attribtues, here is the following syntax:

```css
div my-attribute="hello" {}
```

<div class="note">

When you defined a div with an id or a class, you don't have to write `div`: it will be infered automatically

This means that `#my-id {}` <=> `div #my-id {}`

</div>

### String

Maud doesn't have the same relation with string than HTML. HTML automatically converts everything that is not valid to a String (e.g. `div>` is a string.). While maud, force the quoting of String (e.g. `"div>"`). A basic hello world example is therefore:

```css
span { "Hello world!" }
```

### Expressions

You can really easily embed the value of identifiers in the resulting HTML by using parenthesis

```rs
let my_string = "Hello world!"
html! { span { (my_string) }}
```

This also works with all types of expressions in Rust when the expression returns a value.

### Conditions, Match and loops

Maud supports `match`, `if`, `else`, `while` and `for` statements. To use them, you just need to preceed them by a `@` and you can use them like you would have in a normal context.

```rs
let my_vec = vec!["hello", "world", "!"];

html! {
    main {
        @for token in my_vec {
            span { (token) }
        }
    }
}
```

<div class="note">

it supports pattern matching. So `@if let`s are valid.

</div>
