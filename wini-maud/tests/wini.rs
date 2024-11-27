use {
    hashbrown::HashSet,
    maud::{html, Markup},
};

#[test]
fn basic() {
    let result = html! { h1 { "test" } };
    assert_eq!(result.into_string(), "<h1>test</h1>");
}



fn test_component() -> Markup {
    html! { "test" }
}

#[test]
fn component() {
    let result = html! { h1 { [test_component] } };
    assert_eq!(result.linked_files.len(), 0);
    assert_eq!(result.into_string(), "<h1>test</h1>");
}



fn component_with_linked_file() -> Markup {
    let mut a = html! { "test" };
    let mut set = HashSet::new();
    set.insert("test".to_owned());
    a.linked_files.extend(set);
    a
}

#[test]
fn component_with_linked_file_test() {
    let result = html! { h1 { [component_with_linked_file] } };
    assert_eq!(result.linked_files.len(), 1);
    assert_eq!(result.into_string(), "<h1>test</h1>");
}


fn child() -> Markup {
    let mut a = html! { "test" };
    let mut set = HashSet::new();
    set.insert("one".to_owned());
    a.linked_files.extend(set);
    a
}

fn parent() -> Markup {
    let mut a = html! { [child] };
    let mut set = HashSet::new();
    set.insert("two".to_owned());
    a.linked_files.extend(set);
    a
}


#[test]
fn component_calling_component() {
    let result = html! { h1 { [parent] } };
    assert_eq!(result.linked_files.len(), 2);
    assert_eq!(result.into_string(), "<h1>test</h1>");
}
