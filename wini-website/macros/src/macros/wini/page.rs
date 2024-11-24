use {
    super::args::ProcMacroParameters,
    crate::utils::wini::files::get_js_and_css_files_in_dir,
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Ident},
};

pub fn page(args: TokenStream, item: TokenStream) -> TokenStream {
    // Convert the attributes in a struct.
    let mut attributes = ProcMacroParameters::default();
    let attr_parser = syn::meta::parser(|meta| attributes.parse(meta));
    parse_macro_input!(args with attr_parser);


    // Modify the name of the original function to a reserved one
    let mut original_function = parse_macro_input!(item as syn::ItemFn);
    let original_name = original_function.sig.ident.clone();
    let new_name = Ident::new(
        &format!("__reserved_fn_wini_{}", original_name),
        original_name.span(),
    );
    // Change the function name
    original_function.sig.ident = new_name.clone();

    // Make so, that the new function has the same arguments as the previous one
    let arguments = original_function.sig.inputs.clone();
    let param_names = arguments
        .iter()
        .map(|arg| {
            match arg {
                syn::FnArg::Typed(pat_type) => {
                    if let syn::Pat::Ident(param_name) = &*pat_type.pat {
                        param_name.ident.clone()
                    } else {
                        panic!("Unsupported parameter pattern")
                    }
                },
                syn::FnArg::Receiver(_) => panic!("self parameters not supported."),
            }
        })
        .collect::<Vec<_>>();


    let (js_files, css_files) = get_js_and_css_files_in_dir();


    // Get arguments values stored in an identifier
    let meta_headers = attributes.generate_all_headers();
    let components = attributes.components.unwrap_or_default();

    // Generate the output code
    let expanded = quote! {
        #[allow(non_snake_case)]
        #original_function

        #[allow(non_snake_case)]
        pub async fn #original_name(#arguments) -> axum::response::Response<axum::body::Body> {
            let html = #new_name(#(#param_names),*).await;

            let mut css_files: Vec<String> = (vec![#(#css_files)*] as Vec<&str>).into_iter().map(String::from).collect();
            let mut js_files: Vec<String> = (vec![#(#js_files)*] as Vec<&str>).into_iter().map(String::from).collect();

            // Components to read from
            let component_parent_path = crate::concat_paths!(
                "src",
                &crate::shared::wini::config::SERVER_CONFIG.path.components
            ).display().to_string();

            let components: Vec<&'static str> = vec![#(#components,)*];

            css_files.extend(
                components
                    .iter()
                    .filter_map(|comp|
                        crate::shared::wini::components_files::COMPONENTS_FILES
                            .css
                            .get(*comp)
                    )
                    .flatten()
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            );
            js_files.extend(
                components
                    .iter()
                    .filter_map(|comp|
                        crate::shared::wini::components_files::COMPONENTS_FILES
                            .js
                            .get(*comp)
                    )
                    .flatten()
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            );

            let mut res = axum::response::IntoResponse::into_response(html);

            res.headers_mut().insert(
                "styles",
                axum::http::HeaderValue::from_str(&format!(
                    "{};",
                    css_files.join(";"),
                )).unwrap()
            );

            res.headers_mut().insert(
                "js",
                axum::http::HeaderValue::from_str(&format!(
                    "{};",
                    js_files.join(";"),
                )).unwrap()
            );

            // Modify header with meta tags in it
            #meta_headers

            res
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
