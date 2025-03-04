use syn::spanned::Spanned;

pub(crate) fn is_str_eq_to_path(str: &str, path: &syn::Path) -> bool {
    path.get_ident()
        .span()
        .source_text()
        .is_some_and(|source_text| {
            source_text
                .split(':')
                .last()
                .is_some_and(|last| last == str)
        })
}

pub(crate) fn last_path_to_string(path: &syn::Path) -> String {
    path.get_ident()
        .span()
        .source_text()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .to_owned()
}
