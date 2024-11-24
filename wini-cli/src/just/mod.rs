use {
    clap::{Arg, Command},
    pub_just::ParameterKind,
};

pub mod args_from_file;
pub mod run;
pub mod search_justfile;

#[derive(Debug)]
pub struct MinimalJustfile {
    pub recipes: Vec<MinimalRecipe>,
}

#[derive(Debug)]
pub struct MinimalRecipe {
    name: String,
    doc: Option<String>,
    aliases: Option<Vec<String>>,
    params: Vec<MinimalParam>,
}

#[derive(Debug)]
pub struct MinimalParam {
    name: String,
    kind: ParameterKind,
}

impl From<MinimalRecipe> for Command {
    fn from(val: MinimalRecipe) -> Self {
        let mut command = Command::new(val.name);

        if let Some(doc) = val.doc {
            command = command.about(doc);
        }

        if let Some(aliases) = val.aliases {
            command = command.aliases(aliases)
        }

        command = command.args(
            val.params
                .into_iter()
                .map(|p| {
                    match p.kind {
                        ParameterKind::Plus | ParameterKind::Star => {
                            Arg::new(p.name)
                                .value_delimiter(',')
                                .help("Separate values by ','")
                        },
                        ParameterKind::Singular => Arg::new(p.name),
                    }
                })
                .collect::<Vec<Arg>>(),
        );

        command.clone()
    }
}
