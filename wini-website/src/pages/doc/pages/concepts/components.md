# Components

Components are functions - returning some HTML - that can be used in different pages, layout and components too.

## Usage
```
// ./components/my_button/mod.rs
#[component]
pub async fn render() -> MarkUp {
    html! {
        button.my_button {
            "Press me!"
        }
    }
}

// ./pages/home/mod.rs
use crate::components::my_button;
#[page]
pub async fn render() -> MarkUp {
    html! {
        main {
            "Hello!"
            [my_button::render]
        }
    }
}
```

## About

A component can also include another component, and there are _no limit_ in how many you can nest.

Components can also be used accross multiple pages, layout or components, making them really useful when there is some common logic in the website.


## Example

```
// ./components/my_button/mod.rs
#[component]
pub async fn render() -> MarkUp {
    html! {
        button.my_button {
            "Press me!"
        }
    }
}

// ./pages/home/mod.rs
use crate::components::my_button;
#[page]
pub async fn render() -> MarkUp {
    html! {
        main {
            "Hello!"
            [my_button::render]
        }
    }
}

// ./pages/article/mod.rs
use crate::components::my_button;
#[page]
pub async fn render() -> MarkUp {
    html! {
        h1 { "Article" }
        [my_button::render]
    }
}
```

_**Note**: The syntax of `[my_function]` to delimiter a component is only available in `wini-maud` and not `maud`_
