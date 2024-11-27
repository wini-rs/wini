# Layouts

A layout, is a function that wraps the computation of the requested page in another HTML and sends back the resulting HTML.

## Usage

```
#[page]
pub async fn my_page() {
    html! { "Hello world!" }
}


#[layout]
pub async fn my_layout(html: &str) {
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
     User request
          |
+----------------------+
|       Layout1        |
| +------------------+ |
| |     Layout2      | |
| | +--------------+ | |
| | |    Page      | | |
| | +--------------+ | |
| +------------------+ |
+----------------------+
          |
          V
     User response
```

Will result in: `layout1(layout2(page))`

There is more or less the same relationship between a page/component and layout/page, but there are some differences:

- A layout will just always have one parameter `&str` that is the result of the page, since there is only one page per request / endpoint. At the contrary, a page can use one or zero or multiple components.

- A layout is meant to be usable accross multiple pages. In the case of page/componetent, it's the component that is meant to be used for multiple pages.



_**Note**: A layout can also use a component_

```
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
