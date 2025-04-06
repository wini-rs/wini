pub(crate) fn is_ouput_ty_result(syn_fn: &syn::ItemFn) -> bool {
    if let syn::ReturnType::Type(_, ref return_type) = syn_fn.sig.output {
        if let syn::Type::Path(return_type_path) = &**return_type {
            return_type_path
                .path
                .segments
                .first()
                .is_some_and(|path_segment| {
                    path_segment
                        .ident
                        .span()
                        .source_text()
                        .is_some_and(|string_ident| string_ident.contains("Result"))
                })
        } else {
            false
        }
    } else {
        false
    }
}
