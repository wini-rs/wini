use {
    crate::{
        concat_paths,
        shared::wini::{
            PUBLIC_ENDPOINTS,
            config::SERVER_CONFIG,
            dependencies::{SCRIPTS_DEPENDENCIES, normalize_relative_path},
            err::{ServerError, ServerResult},
            packages_files::{PACKAGES_FILES, VecOrString},
        },
        utils::wini::buffer::buffer_to_string,
    },
    axum::{
        body::Body,
        extract::Request,
        middleware::Next,
        response::{IntoResponse, Response},
    },
    meta::add_meta_tags,
    std::collections::HashSet,
    tower_http::services::ServeFile,
};

mod html;
mod meta;



/// Use the basic template of HTML
pub async fn template(req: Request, next: Next) -> ServerResult<Response> {
    let path = &req.uri().path().to_string();


    if (*PUBLIC_ENDPOINTS).contains(path) {
        return Ok(ServeFile::new(format!("./public{path}"))
            .try_call(req)
            .await
            .map_err(|_| ServerError::PublicRessourceNotFound(path.to_owned()))
            .into_response());
    }

    // Compute the request
    let rep = next.run(req).await;
    let (mut res_parts, res_body) = rep.into_parts();

    let resp_str = buffer_to_string(res_body).await?;

    // Extract and remove the meta tags from the response headers
    let meta_tags = add_meta_tags(&mut res_parts);



    let (scripts, styles) = match res_parts.headers.remove("files") {
        Some(files) => {
            let files = files.to_str()?;

            // Convert the string separated by ; into a vec
            let mut scripts = vec![];
            let mut styles = vec![];

            for file in files[..files.len() - 1].split(';') {
                if !file.is_empty() {
                    let formatted_file = format!("/{file}");
                    if file.ends_with("css") {
                        styles.push(formatted_file);
                    } else if file.ends_with("js") {
                        scripts.push(formatted_file);
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
    let html = html::html(&resp_str, scripts, styles, &meta_tags);

    // Recalculate the length
    *res_parts
        .headers
        .entry("content-length")
        .or_insert(0.into()) = html.len().into();

    res_parts.headers.remove("transfer-encoding");

    let res = Response::from_parts(res_parts, Body::from(html));


    Ok(res)
}


fn order_scripts_by_dependent(scripts: &mut Vec<String>) -> HashSet<String> {
    // The css that is linked to a javascript package, and that therefore, should also be included
    let mut css_included_from_dependencies: HashSet<String> = HashSet::new();
    let mut packages = Vec::<String>::new();

    // Get all dependencies
    let dependencies = scripts
        .iter()
        .filter_map(|script| (*SCRIPTS_DEPENDENCIES).get(script))
        .filter_map(std::clone::Clone::clone)
        .flatten()
        .map(|dep| {
            let public_path =
                normalize_relative_path(concat_paths!("str", &SERVER_CONFIG.path.public))
                    .display()
                    .to_string();

            if dep.starts_with(&public_path) {
                dep[SERVER_CONFIG.path.public.len() - 3..].to_string()
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
        if scripts.contains(&dep) {
            scripts.retain(|script| *script != dep);
        }
        if !packages.contains(&dep) {
            scripts.push(dep.clone());
        }
    }

    for pkg in packages {
        match (*PACKAGES_FILES).get(&pkg) {
            Some(VecOrString::String(file)) => {
                if file.ends_with(".css") {
                    css_included_from_dependencies.insert(file.to_owned());
                } else {
                    scripts.push(file.to_owned());
                }
            },
            Some(VecOrString::Vec(files)) => {
                for file in files {
                    if file.ends_with(".css") {
                        css_included_from_dependencies.insert(file.to_owned());
                    } else {
                        scripts.push(file.to_owned());
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
