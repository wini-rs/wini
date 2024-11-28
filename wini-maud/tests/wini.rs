use {
    hashbrown::HashSet,
    maud::{html, Markup},
};

#[tokio::test]
async fn basic() {
    let result = html! { h1 { "test" } };
    assert_eq!(result.into_string(), "<h1>test</h1>");
}



async fn test_component() -> Markup {
    html! { "test" }
}

#[tokio::test]
async fn component() {
    let result = html! { h1 { [test_component] } };
    assert_eq!(result.linked_files.len(), 0);
    assert_eq!(result.into_string(), "<h1>test</h1>");
}



async fn component_with_linked_file() -> Markup {
    let mut a = html! { "test" };
    let mut set = HashSet::new();
    set.insert("test".to_owned());
    a.linked_files.extend(set);
    a
}

#[tokio::test]
async fn component_with_linked_file_test() {
    let result = html! { h1 { [component_with_linked_file] } };
    assert_eq!(result.linked_files.len(), 1);
    assert_eq!(result.into_string(), "<h1>test</h1>");
}


async fn child() -> Markup {
    let mut a = html! { "test" };
    let mut set = HashSet::new();
    set.insert("one".to_owned());
    a.linked_files.extend(set);
    a
}

async fn parent() -> Markup {
    let mut a = html! { [child] };
    let mut set = HashSet::new();
    set.insert("two".to_owned());
    a.linked_files.extend(set);
    a
}


#[tokio::test]
async fn component_calling_component() {
    let result = html! { h1 { [parent] } };
    assert_eq!(result.linked_files.len(), 2);
    assert_eq!(result.into_string(), "<h1>test</h1>");
}


#[tokio::test]
async fn component_with_parenthesis() {
    let result = html! { h1 { [test_component()] } };
    assert_eq!(result.linked_files.len(), 0);
    assert_eq!(result.into_string(), "<h1>test</h1>");
}

async fn param_component(name: &str) -> Markup {
    html! { span { "Hello "(name)"!" }}
}

#[tokio::test]
async fn component_with_parameters() {
    let result = html! { main { [param_component("Amy")] } };
    assert_eq!(result.linked_files.len(), 0);
    assert_eq!(result.into_string(), "<main><span>Hello Amy!</span></main>");
}
