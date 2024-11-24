use {
    crate::{
        concat_paths,
        shared::wini::{
            config::SERVER_CONFIG,
            dependencies::{normalize_relative_path, SCRIPTS_DEPENDENCIES},
            err::ServerResult,
            packages_files::{VecOrString, PACKAGES_FILES},
            PUBLIC_ENDPOINTS,
        },
        utils::wini::buffer::buffer_to_string,
    },
    axum::{
        body::Body,
        extract::Request,
        middleware::Next,
        response::{IntoResponse, Response},
    },
    itertools::Itertools,
    meta::add_meta_tags,
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
            .unwrap()
            .into_response());
    }

    // Compute the request
    let rep = next.run(req).await;
    let (mut res_parts, res_body) = rep.into_parts();

    let resp_str = buffer_to_string(res_body).await.unwrap();

    // Extract and remove the meta tags from the response headers
    let meta_tags = add_meta_tags(&mut res_parts);

    // The css that is linked to a javascript package, and that therefore, should also be included
    let mut css_included_from_dependencies: Vec<String> = vec![];


    let scripts = match res_parts.headers.remove("js") {
        Some(scripts) => {
            let scripts = scripts.to_str().unwrap();
            let mut packages = Vec::<String>::new();

            // Convert the string separated by ; into a vec
            let mut scripts = scripts[..scripts.len() - 1]
                .split(';')
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(format!("/{s}"))
                    }
                })
                .collect::<Vec<String>>();

            // Get all dependencies
            let dependencies = scripts
                .iter()
                .filter_map(|script| (*SCRIPTS_DEPENDENCIES).get(script))
                .filter_map(|e| e.clone())
                .flatten()
                .map(|dep| {
                    let public_path =
                        normalize_relative_path(&concat_paths!("str", &SERVER_CONFIG.path.public))
                            .display()
                            .to_string();

                    if dep.starts_with(&public_path) {
                        dep[SERVER_CONFIG.path.public.len() - 3..].to_string()
                    } else {
                        if !dep.ends_with(".js") {
                            packages.push(dep.to_owned());
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
                    scripts.push(dep.to_owned())
                }
            }

            for pkg in packages {
                match (*PACKAGES_FILES).get(&pkg) {
                    Some(VecOrString::String(file)) => {
                        if file.ends_with(".css") {
                            css_included_from_dependencies.push(file.to_owned());
                        } else {
                            scripts.push(file.to_owned());
                        }
                    },
                    Some(VecOrString::Vec(files)) => {
                        for file in files {
                            if file.ends_with(".css") {
                                css_included_from_dependencies.push(file.to_owned());
                            } else {
                                scripts.push(file.to_owned());
                            }
                        }
                    },
                    None => {
                        log::warn!("The package {pkg:#?} doesn't have any associated minified file. Therefore, nothing will be send for this package.");
                        continue;
                    },
                }
            }

            scripts.reverse();

            scripts
        },
        None => Vec::new(),
    };

    // Compute the HTML to send
    let html = html::html(
        &resp_str,
        scripts,
        match res_parts.headers.remove("styles") {
            Some(styles) => {
                let styles = styles.to_str().unwrap();
                let mut styles = styles[..styles.len() - 1]
                    .split(';')
                    .unique()
                    .map(|e| format!("/{e}"))
                    .collect::<Vec<String>>();
                styles.extend(css_included_from_dependencies);
                styles
            },
            None => Vec::new(),
        },
        meta_tags,
    );

    // Recalculate the length
    *res_parts
        .headers
        .entry("content-length")
        .or_insert(0.into()) = (html.len() as i32).into();

    res_parts.headers.remove("transfer-encoding");

    let res = Response::from_parts(res_parts, Body::from(html));


    Ok(res)
}
