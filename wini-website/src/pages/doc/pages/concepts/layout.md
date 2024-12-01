# Layouts

Layouts are functions that wraps the computation of the requested page in another HTML and return back the resulting HTML.

## Usage

```rs
#[page]
async fn my_page() {
    html! { "Hello world!" }
}


#[layout]
async fn my_layout(html: &str) {
    html! {
        header {
            "Hello"
        }
        main {
            (html)
        }
    }
} // Will return `<header>Hello</header><main>Hello world!</main>` when applied to my_page
```

## About

Multiple layouts can be applied to an endpoint/request. In this case, the parent layout get the computation of the previous layout:

```
      requests
         |
         v
+--- layer_one ---+
| +- layer_two -+ |
| |             | |
| |   my_page   | |
| |             | |
| +- layer_two -+ |
+--- layer_one ---+
         |
         v
      responses
```

Will result in: `layout_one(layout_two(my_page))`

(_Graph inspired by: <https://docs.rs/axum/latest/axum/middleware/index.html>_)

There is more or less the same relationship between a page/component and layout/page, but there are some differences:

- A layout will just always have one parameter `&str` that is the result of the page, since there is only one page per request / endpoint. At the contrary, a page can use one or zero or multiple components.

- A layout is meant to be usable accross multiple pages. In the case of page/componetent, it's the component that is meant to be used for multiple pages.



<div class="note">

A layout can also use a component

</div>

```rs
#[layout]
pub async fn my_layout(html: &str) {
    html! {
        header {
            [my_header]
        }
        main {
            (html)
        }
    }
}
```
