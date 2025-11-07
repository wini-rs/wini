use {
    std::collections::HashMap,
    syn::{ExprArray, Lit, LitStr, meta::ParseNestedMeta},
};

/// The arguments expected in attribute
#[derive(Default, Debug)]
pub struct ProcMacroParameters {
    /// Title of the webpage: `<title>`, `og:title`
    pub title: Option<String>,
    /// Meta Description: `description`, `og:description`
    pub description: Option<String>,
    /// Meta keywords of this website: `keywords`
    pub keywords: Option<Vec<String>>,
    /// Meta robots: `robots`
    pub robots: Option<String>,
    /// Meta author: `author`
    pub author: Option<String>,
    /// Meta author: `og:site_name`
    pub site_name: Option<String>,
    /// Meta author: `language`
    pub lang: Option<String>,
    /// Meta author: `og:image`
    pub img: Option<String>,
    /// Other meta tags
    pub other_meta: Option<HashMap<String, String>>,
    /// Add JS packages
    pub js_pkgs: Option<Vec<String>>,
}

macro_rules! generate_extension_function {
    ($name:ident) => {
        pub fn $name(&self) -> proc_macro2::TokenStream {
            if let Some(value) = &self.$name {
                quote::quote! {
                    meta_tags.insert(stringify!($name), std::borrow::Cow::Borrowed(#value));
                }
            } else {
                quote::quote!()
            }
        }
    };
}

macro_rules! generate_combined_extensions_function {
    ($($field:ident),*) => {
        pub fn generate_all_extensions(&self, is_parts: bool) -> proc_macro2::TokenStream {
            // Generate all individual TokenStreams
            $(
                let $field = self.$field();
            )*

            // Combine them in a single quote
            if is_parts {
                quote::quote! {
                    {
                        let meta_tags: &mut crate::shared::wini::layer::Tags = resp_parts.extensions.get_or_insert_default();

                        $(#$field)*
                    }
                }
            } else {
                quote::quote! {
                    {
                        let meta_tags: &mut crate::shared::wini::layer::Tags = resp.extensions_mut().get_or_insert_default();

                        $(#$field)*
                    }
                }
            }
        }
    };
}


impl ProcMacroParameters {
    generate_combined_extensions_function!(
        title,
        description,
        robots,
        author,
        site_name,
        lang,
        img,
        keywords,
        other_meta
    );

    generate_extension_function!(title);

    generate_extension_function!(description);

    generate_extension_function!(robots);

    generate_extension_function!(author);

    generate_extension_function!(site_name);

    generate_extension_function!(lang);

    generate_extension_function!(img);

    pub fn keywords(&self) -> proc_macro2::TokenStream {
        if let Some(value) = &self.keywords {
            let keyword_joined = value.join(", ");
            quote::quote! {
                meta_tags.insert(
                    "keywords",
                    std::borrow::Cow::Borrowed(#keyword_joined),
                );
            }
        } else {
            quote::quote!()
        }
    }

    pub fn other_meta(&self) -> proc_macro2::TokenStream {
        if let Some(metas) = &self.other_meta {
            let quotes = metas
                .iter()
                .map(|(meta_name, meta_value)| {
                    quote::quote! {
                        meta_tags.insert(
                            #meta_name,
                            std::borrow::Cow::Borrowed(#meta_value),
                        );
                    }
                })
                .collect::<Vec<_>>();

            quote::quote! {
                #(#quotes)*
            }
        } else {
            quote::quote!()
        }
    }

    /// Function that serve of parser for attributes in syn::meta::parser
    /// See: https://docs.rs/syn/latest/syn/meta/fn.parser.html for more info.
    pub fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if let Some(ident) = meta.path.get_ident() {
            match ident.to_string().as_str() {
                "other_meta" => {
                    let lit_fake_array: ExprArray = meta.value()?.parse()?;

                    for elem in lit_fake_array.elems {
                        // Should be ExprAssign `"str" = "str"`
                        let syn::Expr::Assign(expr) = elem else {
                            panic!(
                                r#"Expected an `ExprAssign` with the following format `"str" = "str"`"#
                            )
                        };

                        let syn::Expr::Lit(syn::ExprLit {
                            attrs: _,
                            lit: Lit::Str(left),
                        }) = *expr.left
                        else {
                            panic!("Expected left part (meta name) to be a `&'static str` ");
                        };

                        let syn::Expr::Lit(syn::ExprLit {
                            attrs: _,
                            lit: Lit::Str(right),
                        }) = *expr.right
                        else {
                            panic!("Expected right part (meta content) to be a `&'static str` ");
                        };

                        if let Some(other_meta) = &mut self.other_meta {
                            other_meta.insert(left.value(), right.value());
                        } else {
                            self.other_meta = Some(HashMap::from([(left.value(), right.value())]))
                        }
                    }

                    Ok(())
                },
                "keywords" => {
                    let lit_array: ExprArray = meta.value()?.parse()?;
                    let mut vec_elements = Vec::with_capacity(lit_array.elems.len());
                    for elem in lit_array.elems {
                        if let syn::Expr::Lit(lit) = elem &&
                            let syn::Lit::Str(lit_str) = lit.lit
                        {
                            vec_elements.push(lit_str.value());
                        }
                    }

                    // Assign it to the correct key
                    match ident.to_string().as_str() {
                        "keywords" => self.keywords = Some(vec_elements),
                        _ => unreachable!("Already matched."),
                    }

                    Ok(())
                },
                "js_pkgs" => {
                    let lit_array: ExprArray = meta.value()?.parse()?;
                    let mut vec_elements = Vec::with_capacity(lit_array.elems.len());
                    for elem in lit_array.elems {
                        if let syn::Expr::Lit(lit) = elem &&
                            let syn::Lit::Str(lit_str) = lit.lit
                        {
                            vec_elements.push(lit_str.value());
                        }
                    }

                    // Assign it to the correct key
                    match ident.to_string().as_str() {
                        "js_pkgs" => self.js_pkgs = Some(vec_elements),
                        _ => unreachable!("Already matched."),
                    }

                    Ok(())
                },
                "description" | "author" | "site_name" | "lang" | "img" | "title" | "robots" => {
                    let string_value = meta.value()?.parse::<LitStr>()?.value();
                    match ident.to_string().as_str() {
                        "description" => self.description = Some(string_value),
                        "author" => self.author = Some(string_value),
                        "site_name" => self.site_name = Some(string_value),
                        "title" => self.title = Some(string_value),
                        "lang" => self.lang = Some(string_value),
                        "img" => self.img = Some(string_value),
                        "robots" => self.robots = Some(string_value),
                        _ => unreachable!("Already matched."),
                    }
                    Ok(())
                },
                _ => Err(meta.error(format!("Unexpected attribute name: {ident}"))),
            }
        } else {
            Err(meta.error("Expected an ident."))
        }
    }
}
