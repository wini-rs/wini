use {
    crate::just::{MinimalJustfile, MinimalParam, MinimalRecipe},
    pub_just::{Compiler, Loader},
    std::{collections::HashMap, ffi::OsString, path::Path},
};

pub fn arguments_from_justfile_path(path: &Path) -> Result<MinimalJustfile, ()> {
    let loader = Loader::new();

    match Compiler::compile(&loader, path) {
        Ok(compilation_res) => {
            let justfile = compilation_res.justfile;

            let mut aliases: HashMap<String, Vec<String>> = HashMap::new();

            for (alias, to) in justfile.aliases {
                aliases
                    .entry(to.target.name.to_string())
                    .or_default()
                    .push(alias.to_owned())
            }

            let recipes = justfile
                .recipes
                .into_iter()
                .map(|(name, recipe)| {
                    let name = name.to_string();
                    MinimalRecipe {
                        doc: recipe.doc.clone(),
                        aliases: aliases.remove(&name),
                        name,
                        params: recipe
                            .parameters
                            .iter()
                            .map(|p| {
                                MinimalParam {
                                    name: p.name.to_string(),
                                    kind: p.kind,
                                }
                            })
                            .collect(),
                    }
                })
                .collect();


            Ok(MinimalJustfile { recipes })
        },
        Err(_) => {
            eprintln!("You have an error in your justfile:");
            let _ = pub_just::run(vec![OsString::new()].into_iter());

            Err(())
        },
    }
}
