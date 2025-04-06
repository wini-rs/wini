use {
    crate::{
        macros::wini::args::ProcMacroParameters,
        utils::wini::{files::get_js_or_css_files_in_current_dir, result::is_ouput_ty_result},
    },
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, spanned::Spanned, FnArg, Ident},
};


enum InputKind {
    StatusCode,
    Str,
    Parts,
    Response,
}

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

    // In case of an error, we want to early return with `?`
    let early_return_if_is_result_err = if is_ouput_ty_result(&input) {
        quote!(?)
    } else {
        Default::default()
    };

    // We want to do different things depending on the input
    let input_kind = match input.sig.inputs.first() {
        Some(first_arg) => {
            match first_arg {
                FnArg::Receiver(_) => panic!("Layouts don't support `self`"),
                FnArg::Typed(pat_ty) => {
                    match (*pat_ty.ty)
                        .span()
                        .source_text()
                    {
                        Some(ty) => {
                            if ty == "&str" {
                                InputKind::Str
                            } else if ty.contains("StatusCode") {
                                InputKind::StatusCode
                            } else if ty.contains("Parts") {
                                if input.sig.inputs.len() == 2 {
                                    InputKind::Response
                                } else {
                                    InputKind::Parts
                                }
                            } else {
                                panic!("Unknown child type: {ty}")
                            }
                        },
                        None => panic!("Expected Layout to have a it's first argument being typed")
                    }
                },
            }
        },
        None => panic!("Layouts should always take the child in parameter.\nDid you meant to create a component or a page ?"),
    };
    let handling_of_response = match input_kind {
        InputKind::Str => {
            quote!(
                let (mut resp_parts, resp_body) = resp.into_parts();

                let resp_str = crate::utils::wini::buffer::buffer_to_string(resp_body).await.unwrap();

                let html = #new_name(&resp_str).await #early_return_if_is_result_err;
            )
        },
        InputKind::Response => {
            quote!(
                let (mut resp_parts, resp_body) = resp.into_parts();

                let html = #new_name(&mut resp_parts, &resp_body).await #early_return_if_is_result_err;
            )
        },
        InputKind::Parts => {
            quote!(
                let (mut resp_parts, _) = resp.into_parts();

                let html = #new_name(&mut resp_parts).await #early_return_if_is_result_err;
            )
        },
        InputKind::StatusCode => {
            quote!(
                let (mut resp_parts, _resp_body) = resp.into_parts();
                let html = #new_name(resp_parts.status).await #early_return_if_is_result_err;
            )
        },
    };

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
            use {
                axum::response::IntoResponse,
                itertools::Itertools,
            };

            const FILES_IN_CURRENT_DIR: &str = #files_in_current_dir;


            let resp = next.run(req).await;

            #handling_of_response

            let files_from_components = html.linked_files.iter().join(";");

            let files = resp_parts
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

            let res = axum::response::Response::from_parts(resp_parts, html.into_string().into());

            Ok(res)
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
