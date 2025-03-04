use {
    cached::proc_macro::cached,
    maud::{html, Markup},
    std::{thread::sleep, time::Duration},
    wini_macros::{init_cache, page},
};

#[init_cache]
#[page]
#[cached]
pub async fn render() -> Markup {
    sleep(Duration::from_secs(3));
    html! {
        button #hello {
            "Say hello!"
        }
    }
}
