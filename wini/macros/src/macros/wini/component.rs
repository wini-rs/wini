use {
    super::args::ProcMacroParameters,
    crate::utils::wini::{
        files::get_js_or_css_files_in_current_dir,
        params_from_itemfn::params_from_itemfn,
    },
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Ident},
};


pub fn component(args: TokenStream, item: TokenStream) -> TokenStream {
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

    let (arguments, param_names) = params_from_itemfn(&original_function);

    let files_in_current_dir = get_js_or_css_files_in_current_dir();

    // Generate the output code
    let expanded = quote! {
        #[allow(non_snake_case)]
        #original_function

        #[allow(non_snake_case)]
        pub async fn #original_name(#arguments) -> maud::Markup {
            use itertools::Itertools;

            const FILES_IN_CURRENT_DIR: &[&str] = &[#(#files_in_current_dir),*];

            let mut html = #new_name(#(#param_names),*).await;

            let hashset = std::collections::HashSet::<String>::from_iter(
                FILES_IN_CURRENT_DIR
                    .iter()
                    .map(std::ops::Deref::deref)
                    .map(String::from)
            );
            html.linked_files.extend(hashset);

            html
        }
    };

    // Convert the generated code back to TokenStream
    TokenStream::from(expanded)
}
