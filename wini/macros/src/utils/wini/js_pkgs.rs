use {proc_macro2::TokenStream, quote::quote};

pub(crate) fn handle(js_pkgs: Option<Vec<String>>, add_to: TokenStream) -> TokenStream {
    if let Some(js_pkgs) = js_pkgs {
        quote!(#(
            match crate::shared::wini::packages_files::PACKAGES_FILES.get(#js_pkgs) {
                Some(crate::shared::wini::packages_files::VecOrString::Vec(pkgs)) => {
                    #add_to.extend(pkgs.into_iter().map(|pkg| Cow::Owned(pkg.strip_prefix('/').unwrap_or(pkg).to_owned())));
                },
                Some(crate::shared::wini::packages_files::VecOrString::String(pkg)) => {
                    #add_to.insert(Cow::Owned(pkg.strip_prefix('/').unwrap_or(pkg).to_owned()));
                },
                None => panic!("Package `{}` does not exist", #js_pkgs),
            };
        )*)
    } else {
        quote!()
    }
}
