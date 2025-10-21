use {
    crate::{
        macros::wini::args::ProcMacroParameters,
        utils::wini::{files::get_js_or_css_files_in_current_dir, result::is_ouput_ty_result},
    },
    proc_macro::TokenStream,
    quote::quote,
    syn::{FnArg, Ident, parse_macro_input, spanned::Spanned},
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

    // In case of an error, we want to early return with `?`
    let early_return_if_is_result_err = if is_ouput_ty_result(&input) {
        quote!(?)
    } else {
        Default::default()
    };

    if input.sig.inputs.is_empty() {
        panic!(
            "Layouts must take the child as a parameter.\nDid you mean to create a component or a page?"
        )
    }

    let mut handling_of_response = Vec::new();

    // TODO: split into 2, before and after request
    for (idx, input_it) in input.sig.inputs.iter().enumerate() {
        let is_last = idx + 1 == input.sig.inputs.len();
        match input_it {
            FnArg::Receiver(_) => panic!("Layouts don't support `self`"),
            FnArg::Typed(pat_ty) => {
                let ty = &pat_ty.ty;

                let from_response_body = if is_last {
                    quote!(
                        match #ty::__from_response_body(resp_body, &()).await {
                            Ok(ok) => ok,
                            Err(into_resp) => return Ok(into_resp.into_response()),
                        }
                    )
                } else {
                    quote!(
                        panic!("FromResponseBody should always be the last argument");
                    )
                };

                handling_of_response.push(quote!(
                    {
                        use {
                            crate::shared::wini::layout::*,
                            axum::response::IntoResponse
                        }; 

                        let dummy: #ty = match
                            (
                                <#ty as IsFromResponseBody>::IS_FROM_RESPONSE_BODY,
                                <#ty as IsFromResponseParts>::IS_FROM_RESPONSE_PARTS,
                                <#ty as IsFromRequestParts>::IS_FROM_REQUEST_PARTS,
                            )
                        {
                            (true, false,  false) => {
                                #from_response_body
                            }
                            (false, true, false) => {
                                let ty = match #ty::__from_response_parts(&mut resp_parts, &()).await {
                                    Ok(ok) => ok,
                                    Err(into_resp) => return Ok(into_resp.into_response()),
                                };
                                ty
                            }
                            (false, false, true) => {
                                todo!()
                                // let (mut req_parts, body) = req.into_parts();
                                // let ty = match #ty::__from_request_parts(&mut req_parts, &()).await {
                                //     Ok(ok) => ok,
                                //     Err(into_resp) => return Ok(into_resp.into_response()),
                                // };
                                // req = axum::extract::Request::from_parts(req_parts, body);
                                // ty
                            }
                            (_, _, _) => unimplemented!("Not implemented yet")
                        };

                        dummy
                    }
                ));

                // match (*pat_ty.ty).span().source_text() {
                //     Some(ty) => {
                //         if ty == "&str" {
                //             InputKind::Str
                //         } else if ty.contains("StatusCode") {
                //             InputKind::StatusCode
                //         } else if ty.contains("Parts") {
                //             if input.sig.inputs.len() == 2 {
                //                 InputKind::Response
                //             } else {
                //                 InputKind::Parts
                //             }
                //         } else {
                //             panic!("Unknown child type: {ty}")
                //         }
                //     },
                //     None => panic!("Expected Layout to have its first argument typed"),
                // }
            },
        }
    }

    // TODO: create an `Option<Backtrace>` to handle errors

    let files_in_current_dir = get_js_or_css_files_in_current_dir();
    let len_files_in_current_dir = files_in_current_dir.len();
    let meta_extensions = attributes.generate_all_extensions(true);

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
                std::borrow::Cow,
            };

            const FILES_IN_CURRENT_DIR: [Cow<'static, str>; #len_files_in_current_dir] = [#(Cow::Borrowed(#files_in_current_dir)),*];

            let mut resp = next.run(req).await;
            let (mut resp_parts, resp_body) = resp.into_parts();
            let html = #new_name( #(#handling_of_response),* ).await #early_return_if_is_result_err;

            let files: &mut crate::shared::wini::layer::Files = resp_parts
                .extensions
                .get_or_insert_default();

            files.extend(html.linked_files.into_iter().map(Cow::Owned));
            files.extend(FILES_IN_CURRENT_DIR);

            // Modify extensions with meta tags in it
            #meta_extensions

            let res = axum::response::Response::from_parts(resp_parts, html.content.0.into());

            Ok(res)
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
