use {
    crate::{
        macros::wini::args::ProcMacroParameters,
        utils::wini::{
            files::get_js_or_css_files_in_current_dir,
            js_pkgs,
            result::is_ouput_ty_result,
        },
    },
    proc_macro::TokenStream,
    proc_macro2::Span,
    quote::quote,
    std::{fmt::Display, str::FromStr},
    syn::{parse_macro_input, spanned::Spanned, FnArg, Ident, PatType},
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

    let mut handling_of_request = Vec::new();
    let mut handling_of_response = Vec::new();

    let number_of_args = input.sig.inputs.len();
    for (idx, input_it) in input.sig.inputs.iter_mut().enumerate() {
        let is_last = idx + 1 == number_of_args;
        match input_it {
            FnArg::Receiver(_) => panic!("Layouts don't support `self`"),
            FnArg::Typed(pat_ty) => {
                let ty = pat_ty.ty.clone();
                let path_ty = ty_into_path(&ty);
                let ty_str = ty.span().source_text().unwrap_or_default();

                // These parts of code will be used multiple times
                // RequestParts
                let from_request_parts = quote!(
                    {
                        let (mut req_parts, body) = req.into_parts();
                        let ty = match #path_ty::__from_request_parts(&mut req_parts, &()).await {
                            Ok(ok) => ok,
                            Err(into_resp) => return Ok(into_resp.into_response()),
                        };
                        req = axum::extract::Request::from_parts(req_parts, body);
                        ty
                    }
                );
                // ResponseParts
                let from_response_parts = quote!(
                    {
                        let ty = match #path_ty::__from_response_parts(&mut resp_parts, &()).await {
                            Ok(ok) => ok,
                            Err(into_resp) => return Ok(into_resp.into_response()),
                        };
                        ty
                    }
                );
                // ResponseBody
                let from_response_body = if is_last {
                    quote!(
                        match #path_ty::__from_response_body(resp_body, &()).await {
                            Ok(ok) => ok,
                            Err(into_resp) => return Ok(into_resp.into_response()),
                        }
                    )
                } else {
                    quote!(
                        const {
                            if <#ty as IsFromResponseBody>::IS_FROM_RESPONSE_BODY {
                                panic!("`FromResponseBody` should always be the last argument");
                            };
                        };
                        unreachable!()
                    )
                };

                let ident_of_request =
                    syn::Ident::new(&format!("__wini_ident_{idx}"), Span::call_site());

                let (const_check, request_attr_condition, response_attr_condition) =
                    match parse_attrs_of_arg(pat_ty) {
                        Ok(Some(impl_from_trait)) => {
                            (
                                {
                                    let condition = match impl_from_trait {
                                        FromTrait::ResponseBody => {
                                            quote!(
                                                <#ty as IsFromResponseBody>::IS_FROM_RESPONSE_BODY
                                            )
                                        },
                                        FromTrait::ResponseParts => {
                                            quote!(
                                                <#ty as IsFromResponseParts>::IS_FROM_RESPONSE_PARTS
                                            )
                                        },
                                        FromTrait::RequestParts => {
                                            quote!(
                                                <#ty as IsFromRequestParts>::IS_FROM_REQUEST_PARTS
                                            )
                                        },
                                    };

                                    let error_msg = format!("Invalid attribute macro: the argument's type doesn't implement `{impl_from_trait}`");

                                    quote!(
                                        let can_early_exit = if #condition {
                                            true
                                        } else {
                                            panic!(#error_msg)
                                        };
                                    )
                                },
                                match impl_from_trait {
                                    FromTrait::ResponseBody => {
                                        quote!(
                                            if true {
                                                None
                                            }
                                        )
                                    },
                                    FromTrait::ResponseParts => {
                                        quote!(
                                            if true {
                                                None
                                            }
                                        )
                                    },
                                    FromTrait::RequestParts => {
                                        quote!(if true { Some(#from_request_parts)})
                                    },
                                },
                                match impl_from_trait {
                                    FromTrait::ResponseBody => {
                                        quote!(if true { #from_response_body })
                                    },
                                    FromTrait::ResponseParts => {
                                        quote!(if true { #from_response_parts })
                                    },
                                    FromTrait::RequestParts => {
                                        quote!(if true { #ident_of_request.unwrap() })
                                    },
                                },
                            )
                        },
                        Ok(None) => {
                            (
                                quote!(let can_early_exit = false;),
                                quote!(
                                    if false {
                                        unreachable!()
                                    }
                                ),
                                quote!(
                                    if false {
                                        unreachable!()
                                    }
                                ),
                            )
                        },
                        Err(err_msg) => panic!("{err_msg}"),
                    };

                // Error messages
                //
                // The reason we generate it in the proc_macro and not in the code run at runtime
                // or const-time is because `std::fmt` is not `const`.
                let generate_err_msg_conflit =
                    |a: FromTrait, b: FromTrait, maybe_c: Option<FromTrait>| {
                        if let Some(c) = maybe_c {
                            format!(
                                "{ty_str} implements `{a}`, `{b}` and `{c}`. Specify which implementation it should come from by either anotating it with `#[{}]`, `#[{}]` or `#[{}]`",
                                a.to_snake_case_static_str(),
                                b.to_snake_case_static_str(),
                                c.to_snake_case_static_str()
                            )
                        } else {
                            format!(
                                "{ty_str} implements both `{a}` and `{b}`. Specify which implementation it should come from by either anotating it with `#[{}]` or `#[{}]`",
                                a.to_snake_case_static_str(),
                                b.to_snake_case_static_str()
                            )
                        }
                    };

                let not_a_valid_extractor_error = format!("{ty_str} is not a valid extractor since it does not implement either the `FromResponseBody`, `FromResponseParts` or `FromRequestParts` traits.", );
                let err_msg_reqp_and_respp = generate_err_msg_conflit(
                    FromTrait::RequestParts,
                    FromTrait::ResponseParts,
                    None,
                );
                let err_msg_reqp_and_respb = generate_err_msg_conflit(
                    FromTrait::RequestParts,
                    FromTrait::ResponseBody,
                    None,
                );
                let err_msg_respp_and_respb = generate_err_msg_conflit(
                    FromTrait::ResponseParts,
                    FromTrait::ResponseBody,
                    None,
                );
                let err_msg_all_impl = generate_err_msg_conflit(
                    FromTrait::RequestParts,
                    FromTrait::ResponseParts,
                    Some(FromTrait::ResponseBody),
                );

                handling_of_request.push(quote!(
                    let #ident_of_request = {
                        use {
                            crate::shared::wini::layout::*,
                            axum::response::IntoResponse
                        };

                        // Compile-time check to verify the validity of arguments
                        // The `const` name will appear in the error message, this is why it's
                        // named like that
                        const _CHECK_THE_ERROR_MESSAGE_IN_CASE_OF_ERROR: () = const {
                            #const_check

                            if !can_early_exit {
                                match
                                (
                                    <#ty as IsFromResponseBody>::IS_FROM_RESPONSE_BODY,
                                    <#ty as IsFromResponseParts>::IS_FROM_RESPONSE_PARTS,
                                    <#ty as IsFromRequestParts>::IS_FROM_REQUEST_PARTS,
                                )
                                {
                                    // Valid!
                                    (true, false,  false) |
                                    (false, true, false) |
                                    (false, false, true) => {}
                                    // Not a valid extractor
                                    (false, false, false) => {
                                        panic!(#not_a_valid_extractor_error)
                                    }
                                    // Overlapping
                                    (true, true,  false) => {
                                        panic!(#err_msg_respp_and_respb)
                                    }
                                    (true, false, true) => {
                                        panic!(#err_msg_reqp_and_respb)

                                    }
                                    (false, true, true) => {
                                        panic!(#err_msg_reqp_and_respp)
                                    }
                                    // Full overlapping
                                    (true, true, true) => {
                                        panic!(#err_msg_all_impl)
                                    }
                                }
                            }
                        };

                        let dummy: Option<#ty> = #request_attr_condition else {
                            match
                                (
                                    <#ty as IsFromResponseBody>::IS_FROM_RESPONSE_BODY,
                                    <#ty as IsFromResponseParts>::IS_FROM_RESPONSE_PARTS,
                                    <#ty as IsFromRequestParts>::IS_FROM_REQUEST_PARTS,
                                )
                            {
                                (true, false,  false) => {
                                    None
                                }
                                (false, true, false) => {
                                    None
                                }
                                (false, false, true) => {
                                    Some(#from_request_parts)
                                }
                                (_, _, _) => unreachable!("Verified in a `const` check")
                            }
                        };

                        dummy
                    };
                ));

                handling_of_response.push(quote!(
                    {
                        use {
                            crate::shared::wini::layout::*,
                            axum::response::IntoResponse
                        };

                        let dummy: #ty = #response_attr_condition else {
                            match
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
                                    #from_response_parts
                                }
                                (false, false, true) => {
                                    #ident_of_request.unwrap()
                                }
                                (_, _, _) => unreachable!("Verified in a `const` check")
                            }
                        };

                        dummy
                    }
                ));
            },
        }
    }

    let files_in_current_dir = get_js_or_css_files_in_current_dir();
    let len_files_in_current_dir = files_in_current_dir.len();
    let meta_extensions = attributes.generate_all_extensions(true);

    let js_pkgs = js_pkgs::handle(attributes.js_pkgs, quote!(files));

    // Generate the output code
    let expanded = quote! {
        #[allow(non_snake_case)]
        #input


        #[allow(non_snake_case)]
        pub async fn #name(
            mut req: axum::extract::Request,
            next: axum::middleware::Next
        ) -> crate::shared::wini::err::ServerResult<axum::response::Response> {
            use {
                axum::response::IntoResponse,
                itertools::Itertools,
                std::borrow::Cow,
            };

            const FILES_IN_CURRENT_DIR: [Cow<'static, str>; #len_files_in_current_dir] = [#(Cow::Borrowed(#files_in_current_dir)),*];

            #(#handling_of_request)*

            let mut resp = next.run(req).await;
            let (mut resp_parts, resp_body) = resp.into_parts();
            let html = #new_name( #(#handling_of_response),* ).await #early_return_if_is_result_err;

            let files: &mut crate::shared::wini::layer::Files = resp_parts
                .extensions
                .get_or_insert_default();

            files.extend(html.linked_files.into_iter().map(Cow::Owned));
            files.extend(FILES_IN_CURRENT_DIR);

            #js_pkgs

            // Modify extensions with meta tags in it
            #meta_extensions

            let res = axum::response::Response::from_parts(resp_parts, html.content.0.into());

            Ok(res)
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}

fn parse_attrs_of_arg(arg: &mut PatType) -> Result<Option<FromTrait>, &'static str> {
    let mut current_from = None;

    for (idx, attr) in arg.attrs.iter().enumerate() {
        if let Ok(from) = FromTrait::from_str(
            &attr
                .path()
                .get_ident()
                .span()
                .source_text()
                .unwrap_or_default(),
        ) {
            if current_from.is_some() {
                return Err("You cannot have different `#[from_request_parts]`, `#[from_response_body]` or `#[from_response_parts]` for the same argument.");
            } else {
                current_from = Some((from, idx));
            }
        }
    }

    if let Some((_, idx)) = current_from {
        arg.attrs.remove(idx);
    }

    Ok(current_from.map(|(from, _idx)| from))
}

fn ty_into_path(ty: &syn::Type) -> syn::Type {
    syn::parse_str(
        &ty.span()
            .source_text()
            .unwrap_or_default()
            .replace('<', "::<"),
    )
    .unwrap()
}

enum FromTrait {
    ResponseParts,
    ResponseBody,
    RequestParts,
}

impl Display for FromTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ResponseParts => "FromResponseParts",
            Self::ResponseBody => "FromResponseBody",
            Self::RequestParts => "FromRequestParts",
        })
    }
}
impl FromTrait {
    fn to_snake_case_static_str(&self) -> &'static str {
        match self {
            Self::ResponseParts => "from_response_parts",
            Self::ResponseBody => "from_response_body",
            Self::RequestParts => "from_request_parts",
        }
    }
}

impl FromStr for FromTrait {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "from_response_parts" => Self::ResponseParts,
            "from_response_body" => Self::ResponseBody,
            "from_request_parts" => Self::RequestParts,
            _ => return Err(()),
        })
    }
}
