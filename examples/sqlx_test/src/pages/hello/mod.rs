use {
    crate::POOL,
    maud::{html, Markup},
    wini_macros::{cache, page},
};

struct MyUser {
    name: String,
    age: Option<i32>,
}

#[cache]
#[page]
pub async fn render() -> Markup {
    let random_user = sqlx::query_as!(
        MyUser,
        r#"
        select name, age
        from users
        order by random()
        limit 1;
        "#
    )
    .fetch_one(&*POOL)
    .await
    .expect("An error occurred");

    html! {
        main {
            span { "Hello to "(random_user.name)"! ("(random_user.age.unwrap_or_default())" years old)"}
        }
    }
}
