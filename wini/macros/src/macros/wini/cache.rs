use {
    crate::{SHOULD_CACHE_FN, utils::wini::path::is_str_eq_to_path},
    proc_macro::TokenStream,
    quote::quote,
    syn::{Ident, parse_macro_input},
};



pub fn init_cache(_args: TokenStream, item: TokenStream) -> TokenStream {
    if !*SHOULD_CACHE_FN {
        return item;
    }

    // Modify the name of the current input to a reserved one
    let input = parse_macro_input!(item as syn::ItemFn);

    // We always want to check that there is `#[cached]`
    let cache_fn_name = match input.attrs.iter().find(|attr| {
        match &attr.meta {
            syn::Meta::Path(path) => is_str_eq_to_path("cached", path),
            syn::Meta::List(meta_list) => is_str_eq_to_path("cached", &meta_list.path),
            syn::Meta::NameValue(_) => false,
        }
    }) {
        Some(_attr) => {
            Ident::new(
                &format!("__reserved_fn_wini_{}_prime_cache", &input.sig.ident),
                input.sig.ident.span(),
            )
        },
        None => panic!("There should be a `#[cached]` proc_macro when using `#[init_cache]`"),
    };



    let ctor_name = Ident::new(
        &format!("__ctor_initialize_{}", input.sig.ident),
        input.sig.ident.span(),
    );

    let expanded = quote! {
        // The original function with all the code
        #input

        // The function that is going to force the compute of the lazylock on the start of the
        // program
        #[ctor::ctor]
        fn #ctor_name() {
            let temp_runtime = tokio::runtime::Runtime::new().unwrap();

            temp_runtime.block_on(async {
                let _ = #cache_fn_name().await;
            });
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
