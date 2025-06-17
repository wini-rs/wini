use syn::spanned::Spanned;

pub(crate) fn is_str_eq_to_path(str: &str, path: &syn::Path) -> bool {
    path.get_ident()
        .span()
        .source_text()
        .is_some_and(|source_text| {
            source_text
                .split(':')
                .next_back()
                .is_some_and(|last| last == str)
        })
}
