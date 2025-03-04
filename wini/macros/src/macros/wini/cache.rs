use {
    crate::{utils::wini::path::is_str_eq_to_path, SHOULD_CACHE_FN},
    proc_macro::TokenStream,
    quote::quote,
    syn::{meta::ParseNestedMeta, parse_macro_input, Attribute, Ident, LitBool, MetaList},
};

/// The arguments expected in attribute
#[derive(Default, Debug)]
struct CacheProcMacroParameters {
    pub is_public: bool,
}

impl CacheProcMacroParameters {
    /// Function that serve of parser for attributes in syn::meta::parser
    /// See: https://docs.rs/syn/latest/syn/meta/fn.parser.html for more info.
    pub fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if let Some(ident) = meta.path.get_ident() {
            match ident.to_string().as_str() {
                "public" => {
                    meta.parse_nested_meta(|meta| {
                        let value = meta.value()?.parse::<LitBool>()?.value();

                        self.is_public = value;

                        Ok(())
                    })
                },
                _ => Err(meta.error(format!("Unexpected attribute name: {ident}"))),
            }
        } else {
            Err(meta.error("Expected an ident."))
        }
    }
}


pub fn init_cache(args: TokenStream, item: TokenStream) -> TokenStream {
    if !*SHOULD_CACHE_FN {
        return item;
    }

    // Parse attributes
    let mut attributes = CacheProcMacroParameters::default();
    let attr_parser = syn::meta::parser(|meta| attributes.parse(meta));
    parse_macro_input!(args with attr_parser);


    // Modify the name of the current input to a reserved one
    let mut input = parse_macro_input!(item as syn::ItemFn);

    // We always want to check that there is `#[cached]`
    let cache_fn_name = match input.attrs.iter().find(|attr| {
        match &attr.meta {
            syn::Meta::Path(path) => is_str_eq_to_path("cached", &path),
            syn::Meta::List(meta_list) => is_str_eq_to_path("cached", &meta_list.path),
            syn::Meta::NameValue(_) => false,
        }
    }) {
        Some(attr) => {
            match &attr.meta {
                syn::Meta::Path(_path) => {
                    Ident::new(
                        &format!("__reserved_fn_wini_{}_prime_cache", &input.sig.ident),
                        input.sig.ident.span(),
                    )
                },
                syn::Meta::List(meta_list) => todo!(),
                syn::Meta::NameValue(_) => unreachable!("We don't match `NameValue`"),
            }
        },
        None => panic!("uwu"),
    };



    let original_name = input.sig.ident.clone();
    let ctor_name = Ident::new(
        &format!("__ctor_initialize_{}", original_name),
        original_name.span(),
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
                use http_body_util::BodyExt;
                use axum::response::IntoResponse;
                let _ = #cache_fn_name().await;
            });
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
