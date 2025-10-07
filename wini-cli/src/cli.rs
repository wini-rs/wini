use {
    crate::{
        init::err::InitError,
        just::{args_from_file::arguments_from_justfile_path, search_justfile::search},
    },
    anstyle::AnsiColor,
    clap::{Command, crate_version},
};

pub fn build() -> Result<clap::ArgMatches, InitError> {
    let mut command = base_command();

    if let Some(justfile_path) = search() {
        let justfile = arguments_from_justfile_path(&justfile_path)?;
        command = command.subcommands(justfile.recipes);
    } else {
        command = command.after_help("No justfile found.");
    }

    Ok(command.get_matches())
}

fn base_command() -> Command {
    Command::new("wini")
        .version(crate_version!())
        .styles(get_styles())
        .about("Handle your wini project!")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init").about("Initialize a new wini project"),
            // .arg(arg!(-u --url "Remote repo URL").action(ArgAction::SetTrue))
            // .arg(arg!(-b --branch "Remote repo URL on branch").action(ArgAction::SetTrue)),
        )
}

pub const fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .placeholder(AnsiColor::White.on_default().italic().underline())
        .usage(AnsiColor::Blue.on_default())
        .header(AnsiColor::Blue.on_default().bold())
        .literal(AnsiColor::Cyan.on_default())
        .invalid(AnsiColor::Red.on_default().bold())
        .valid(AnsiColor::Green.on_default().bold())
}
