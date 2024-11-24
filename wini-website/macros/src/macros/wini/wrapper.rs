use {
    crate::{
        macros::wini::args::ProcMacroParameters,
        utils::wini::files::get_js_and_css_files_in_dir,
    },
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Ident},
};


pub fn wrapper(args: TokenStream, item: TokenStream) -> TokenStream {
    // Convert the attributes in a struct.
    let mut attributes = ProcMacroParameters::default();
    let attr_parser = syn::meta::parser(|meta| attributes.parse(meta));
    parse_macro_input!(args with attr_parser);

    let mut input = parse_macro_input!(item as syn::ItemFn);

    // Modify the name of the current input to a reserved one
    let name = input.sig.ident;
    let new_name = Ident::new(&format!("__reserved_fn_wini_{}", name), name.span());
    input.sig.ident = new_name.clone();

    let (js_files, css_files) = get_js_and_css_files_in_dir();

    let meta_headers = attributes.generate_all_headers();
    let components = attributes.components.unwrap_or_default();

    // Generate the output code
    let expanded = quote! {
        #[allow(non_snake_case)]
        #input

        const COMPONENTS: &[&'static str] = &[#(#components,)*];

        #[allow(non_snake_case)]
        pub async fn #name(
            req: axum::extract::Request,
            next: axum::middleware::Next
        ) -> crate::shared::wini::err::ServerResult<axum::response::Response> {
            let rep = next.run(req).await;
            let (mut res_parts, res_body) = rep.into_parts();

            let resp_str = crate::utils::wini::buffer::buffer_to_string(res_body).await.unwrap();

            let html = #new_name(&resp_str).await.into_string();

            let mut css_files: Vec<String> = (vec![#(#css_files)*] as Vec<&'static str>).into_iter().map(String::from).collect();
            let mut js_files: Vec<String> = (vec![#(#js_files)*] as Vec<&'static str>).into_iter().map(String::from).collect();

            // Add components
            let component_parent_path = crate::concat_paths!(
                "src",
                &crate::shared::wini::config::SERVER_CONFIG.path.components
            ).display().to_string();

            css_files.extend(
                COMPONENTS
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
                COMPONENTS
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


            let scripts = res_parts
                .headers
                .entry("js")
                .or_insert_with(|| axum::http::HeaderValue::from_str("").unwrap());

            *scripts = axum::http::HeaderValue::from_str(
                &format!(
                    "{}{};",
                    scripts.to_str().unwrap(),
                    js_files.join(";"),
                )
            ).unwrap();


            let styles = res_parts
                .headers
                .entry("styles")
                .or_insert_with(|| axum::http::HeaderValue::from_str("").unwrap());

            *styles = axum::http::HeaderValue::from_str(
                &format!(
                    "{}{};",
                    styles.to_str().unwrap(),
                    css_files.join(";"),
                )
            ).unwrap();


            // Modify header with meta tags in it
            #meta_headers

            let res = axum::response::Response::from_parts(res_parts, html.into());

            Ok(res)
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
