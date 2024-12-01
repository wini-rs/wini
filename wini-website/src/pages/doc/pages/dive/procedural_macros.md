# Procedural macros

This chapter covers how the procedural macros of `macros/` work

## `#[page]`

`#[page]` will rename the function that you wrote to another name, and use the base name to render the function and add the files in the header:

```rs
#[page]
async fn my_page() -> Markup;

// Becomes

async fn __my_page() -> Markup;

async fn my_page() -> Response<Body> {
    let content = __my_page().await;

    // We also get the files in the current directory and add
    // them to the header, so the function that called the component
    // can know which file to render
    let files_in_current_directory = get_files_in_current_directory();
    // After that:
    // * Add the files of the rendered page and the files in
    // the current directory, in the header
    // * Meta tags are added if there are some
}
```

_Definition of [`#[page]`](<https://codeberg.org/wini/wini-template/src/branch/main/macros/src/macros/wini/page.rs>)_

## `#[layout]`

`#[layout]` works in a very similar way. The difference is that, it doesn't do `__my_page().await` but

```rs 
// Render the request
let rep = next.run(req).await;

// Apply the layout to what the request was rendered to
let html = __my_layout(&resp_str).await;
```
_Definition of [`#[layout]`](<https://codeberg.org/wini/wini-template/src/branch/main/macros/src/macros/wini/layout.rs>)_

## `#[component]`

`#[component]` is also somewhat similar to `#[page]`. The difference is that it doesn't add files to the headers, but to `Markup`

_Definition of [`#[component]`](<https://codeberg.org/wini/wini-template/src/branch/main/macros/src/macros/wini/component.rs>)_

## `#[cache]`

`#[cache]` is a somewhat unique macro, that cache all the result of a page a component or a layout when the program is ran.

This is due to the [`ctor`](https://docs.rs/ctor/latest/ctor/`) crate and [macro](https://docs.rs/ctor/latest/ctor/attr.ctor.html), that allows to run arbitrary code when the binary is executed.

Therefore, what is done is more or less this:

```rs 
#[cache]
#[page]
async fn my_page() -> Markup;

// Becomes

#[page]
async fn __inner_my_page() -> Markup;

#[ctor::ctor]
fn init_lazylock() {
    LazyLock::force(&__STORED_RESPONSE_my_page);
}

static __STORED_RESPONSE_my_page: LazyLock<Response<Body>> = LazyLock::new(__inner_my_page);

async fn my_page() -> Response<Body> {
    __STORED_RESPONSE_my_page
}
```
