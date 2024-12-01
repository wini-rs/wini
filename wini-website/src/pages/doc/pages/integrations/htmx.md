# htmx

_[htmx](https://htmx.org/) is a JavaScript library that enables dynamic HTML updates via AJAX requests without full page reloads._

htmx, works really great with `wini` for multiple reasons.

## 1. Easy to install:

```sh
wini js-add htmx.org
```

## 2. Easy to use:

Simply add this code to the script linked to the page where you want to use htmx.

```js
import "htmx.org";
```

## 3. Goes well with maud:

```scss
div hx-post="/mouse_entered" hx-trigger="mouseenter" {
    "[Here Mouse, Mouse!]"
}
```

_A complete example can be found here: [github](https://github.com/wini-rs/wini/tree/main/examples/htmx) / [codeberg](https://codeberg.org/wini/wini/src/branch/main/examples/htmx)_
