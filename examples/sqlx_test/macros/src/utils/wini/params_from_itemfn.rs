use syn::{punctuated::Punctuated, token::Comma, FnArg, Ident, ItemFn};

pub fn params_from_itemfn(function: &ItemFn) -> (Punctuated<FnArg, Comma>, Vec<Ident>) {
    // Make so, that the new function has the same arguments as the previous one
    let arguments = function.sig.inputs.clone();
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

    (arguments, param_names)
}
