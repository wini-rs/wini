# sqlx

_[sqlx](https://github.com/launchbadge/sqlx) is an asynchronous, compile-time checked SQL library for Rust that provides a type-safe way to interact with databases using raw SQL queries_

Since wini is built on top of [`axum`](https://github.com/tokio-rs/axum), it works really well with sqlx!

<div class="warn">

Using sqlx is not always a good idea since you need to wait for the query to render the page!

</div>

Here is how you would do it:

```rs
struct MyUser {
    name: String,
    age: i32,
}

#[page]
fn my_page() -> Markup {
    let user = sqlx::query_as!(
        MyUser,
        r#"
        select name, age 
        from users
        where id = $1
        "#,
        10
    )
        .fetch_one(&POOL)
        .await
        .unwrap();

    html! {
        h1 {"Hi "(user.name)"!"}
        span {"You're "(user.age)" years old!"}
    }
}
```
