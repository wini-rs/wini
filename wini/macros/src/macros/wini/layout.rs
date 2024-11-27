use {
    crate::{
        macros::wini::args::ProcMacroParameters,
        utils::wini::files::get_js_or_css_files_in_current_dir,
    },
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Ident},
};


pub fn layout(args: TokenStream, item: TokenStream) -> TokenStream {
    // Convert the attributes in a struct.
    let mut attributes = ProcMacroParameters::default();
    let attr_parser = syn::meta::parser(|meta| attributes.parse(meta));
    parse_macro_input!(args with attr_parser);

    let mut input = parse_macro_input!(item as syn::ItemFn);

    // Modify the name of the current input to a reserved one
    let name = input.sig.ident;
    let new_name = Ident::new(&format!("__reserved_fn_wini_{}", name), name.span());
    input.sig.ident = new_name.clone();

    let files_in_current_dir = get_js_or_css_files_in_current_dir().join(";");
    let meta_headers = attributes.generate_all_headers();

    // Generate the output code
    let expanded = quote! {
        #[allow(non_snake_case)]
        #input


        #[allow(non_snake_case)]
        pub async fn #name(
            req: axum::extract::Request,
            next: axum::middleware::Next
        ) -> crate::shared::wini::err::ServerResult<axum::response::Response> {
            use itertools::Itertools;

            const FILES_IN_CURRENT_DIR: &str = #files_in_current_dir;


            let rep = next.run(req).await;
            let (mut res_parts, res_body) = rep.into_parts();

            let resp_str = crate::utils::wini::buffer::buffer_to_string(res_body).await.unwrap();

            let html = #new_name(&resp_str).await;

            let files_from_components = html.linked_files.iter().join(";");

            let files = res_parts
                .headers
                .entry("files")
                .or_insert_with(|| axum::http::HeaderValue::from_str("").unwrap());

            *files = axum::http::HeaderValue::from_str(
                &format!(
                    "{FILES_IN_CURRENT_DIR};{files_from_components};{};",
                    files.to_str().unwrap(),
                )
            ).unwrap();


            // Modify header with meta tags in it
            #meta_headers

            let res = axum::response::Response::from_parts(res_parts, html.into_string().into());

            Ok(res)
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
