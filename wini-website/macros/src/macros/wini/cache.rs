use {
    crate::SHOULD_CACHE_FN,
    proc_macro::TokenStream,
    quote::quote,
    syn::{meta::ParseNestedMeta, parse_macro_input, Ident, LitBool},
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


pub fn cache(args: TokenStream, item: TokenStream) -> TokenStream {
    if !*SHOULD_CACHE_FN {
        return item;
    }

    // Parse attributes
    let mut attributes = CacheProcMacroParameters::default();
    let attr_parser = syn::meta::parser(|meta| attributes.parse(meta));
    parse_macro_input!(args with attr_parser);


    // Modify the name of the current input to a reserved one
    let mut input = parse_macro_input!(item as syn::ItemFn);
    let original_name = input.sig.ident.clone();
    let new_name = Ident::new(
        &format!("__reserved_cache_fn_wini_{}", original_name),
        original_name.span(),
    );
    let ctor_name = Ident::new(
        &format!("__ctor_reserved_cache_fn_wini_{}", original_name),
        original_name.span(),
    );
    let const_cache_name = Ident::new(
        &format!(
            "{}CONST_CACHE_WINI_{}",
            if attributes.is_public {
                ""
            } else {
                "__RESERVED_"
            },
            original_name.to_string().to_uppercase()
        ),
        original_name.span(),
    );
    // Change the function name
    input.sig.ident = new_name.clone();

    let expanded = quote! {
        // The original function with all the code
        #input

        // The function that is going to force the compute of the lazylock on the start of the
        // program
        #[ctor::ctor]
        fn #ctor_name() {
            std::sync::LazyLock::force(&#const_cache_name);
        }


        // The lazylock containg the computed request response
        static #const_cache_name: std::sync::LazyLock<(axum::http::response::Parts, axum::body::Bytes)> = std::sync::LazyLock::new(|| {
            let temp_runtime = tokio::runtime::Runtime::new().unwrap();

            temp_runtime.block_on(async {
                use http_body_util::BodyExt;
                use axum::response::IntoResponse;
                let ok = #new_name().await.into_response().into_parts();
                (ok.0, ok.1.collect().await.unwrap().to_bytes())
            })
        });

        // The function returning the lazylock content
        #[allow(non_snake_case)]
        pub async fn #original_name() -> axum::response::Response<axum::body::Body> {
            let (parts, bytes) = #const_cache_name.clone();
            axum::response::Response::from_parts(parts, bytes.into())
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
