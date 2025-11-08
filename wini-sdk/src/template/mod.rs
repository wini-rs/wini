use {
    crate::{
        shared::wini::{
            config::SERVER_CONFIG,
            dependencies::SCRIPTS_DEPENDENCIES,
            err::ServerResult,
            layer::Files,
            packages_files::{PACKAGES_FILES, VecOrString},
        },
        utils::wini::buffer::buffer_to_string,
    },
    axum::{body::Body, extract::Request, middleware::Next, response::Response},
    hyper::header::{CONTENT_LENGTH, TRANSFER_ENCODING},
    meta::add_meta_tags,
    std::{
        borrow::{Borrow, Cow},
        collections::HashSet,
    },
};

mod html;
mod meta;



/// Use the basic template of HTML
pub async fn template<F>(req: Request, next: Next, compute_html: F) -> ServerResult<Response>
where
    F: Fn(&str, Vec<Cow<str>>, Vec<Cow<str>>, &maud::Markup) -> String,
{
    // Compute the request
    let rep = next.run(req).await;
    let (mut res_parts, res_body) = rep.into_parts();

    let resp_str = buffer_to_string(res_body).await?;

    // Extract the meta tags from the response headers
    let meta_tags = add_meta_tags(&mut res_parts);

    // Files is declared here and not in the match so it has a lifetime that goes into `html::html`
    let files = res_parts.extensions.get::<Files>();
    let (scripts, styles) = match files {
        Some(files) => {
            // Convert the string separated by ; into a vec
            let mut scripts = Vec::new();
            let mut styles = Vec::new();

            for file in files {
                if !file.is_empty() {
                    if file.ends_with("css") {
                        styles.push(Cow::Borrowed(file.borrow()));
                    } else if file.ends_with("js") {
                        scripts.push(Cow::Borrowed(file.borrow()));
                    }
                }
            }

            let css_included_from_dependencies = order_scripts_by_dependent(&mut scripts);

            styles.extend(css_included_from_dependencies);

            (scripts, styles)
        },
        None => (Vec::new(), Vec::new()),
    };

    // Compute the HTML to send
    let html = compute_html(&resp_str, scripts, styles, &meta_tags);

    // Recalculate the length
    *res_parts.headers.entry(CONTENT_LENGTH).or_insert(0.into()) = html.len().into();

    res_parts.headers.remove(TRANSFER_ENCODING);

    let res = Response::from_parts(res_parts, Body::from(html));


    Ok(res)
}


fn order_scripts_by_dependent<'a>(scripts: &mut Vec<Cow<str>>) -> HashSet<Cow<'a, str>> {
    // The css that is linked to a javascript package, and that therefore, should also be included
    let mut css_included_from_dependencies: HashSet<Cow<str>> = HashSet::new();
    let mut packages = Vec::<String>::new();

    // Get all dependencies
    let dependencies = scripts
        .iter()
        .filter_map(|script| SCRIPTS_DEPENDENCIES.get(script.as_ref()))
        .filter_map(std::clone::Clone::clone)
        .flatten()
        .map(|dep| {
            let public_path = SERVER_CONFIG.path().public_from_src();

            if dep.starts_with(&public_path) {
                dep[SERVER_CONFIG.path().public().len() - 3..].to_string()
            } else {
                if !dep.ends_with(".js") {
                    packages.push(dep.clone());
                }
                dep
            }
        })
        .collect::<Vec<String>>();

    // Pop the dependencies at the top
    for dep in dependencies {
        if scripts.contains(&Cow::Borrowed(&dep)) {
            scripts.retain(|script| *script != dep);
        }
        if !packages.contains(&dep) {
            scripts.push(Cow::Owned(dep.clone()));
        }
    }

    for pkg in packages {
        match PACKAGES_FILES.get(&pkg) {
            Some(VecOrString::String(file)) => {
                if file.ends_with(".css") {
                    css_included_from_dependencies.insert(Cow::Borrowed(file));
                } else {
                    scripts.push(Cow::Borrowed(file));
                }
            },
            Some(VecOrString::Vec(files)) => {
                for file in files {
                    if file.ends_with(".css") {
                        css_included_from_dependencies.insert(Cow::Borrowed(file));
                    } else {
                        scripts.push(Cow::Borrowed(file));
                    }
                }
            },
            None => {
                log::warn!(
                    "The package {pkg:#?} doesn't have any associated minified file. Therefore, nothing will be send for this package."
                );
            },
        }
    }

    scripts.reverse();

    css_included_from_dependencies
}
