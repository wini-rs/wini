use {
    super::args::ProcMacroParameters,
    crate::utils::wini::{
        files::get_js_or_css_files_in_current_dir,
        params_from_itemfn::params_from_itemfn,
        result::is_ouput_ty_result,
    },
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
    original_function.sig.ident = new_name.clone();

    let early_return_if_is_result_err = if is_ouput_ty_result(&original_function) {
        quote!(?)
    } else {
        Default::default()
    };

    let (arguments, param_names) = params_from_itemfn(&original_function);

    let files_in_current_dir = get_js_or_css_files_in_current_dir().join(";");
    let meta_headers = attributes.generate_all_headers();

    // Generate the output code
    let expanded = quote! {
        #[allow(non_snake_case)]
        #original_function

        #[allow(non_snake_case)]
        pub async fn #original_name(#arguments) -> crate::shared::wini::err::ServerResult<axum::response::Response<axum::body::Body>> {
            use {
                axum::response::IntoResponse,
                itertools::Itertools,
            };

            const FILES_IN_CURRENT_DIR: &str = #files_in_current_dir;

            let html = #new_name(#(#param_names),*).await #early_return_if_is_result_err;

            let files = html.linked_files.iter().join(";");

            let mut res = axum::response::IntoResponse::into_response(html);

            res.headers_mut().insert(
                "files",
                axum::http::HeaderValue::from_str(&format!(
                    "{FILES_IN_CURRENT_DIR};{files};",
                )).unwrap()
            );


            // Modify header with meta tags in it
            #meta_headers

            Ok(res)
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
