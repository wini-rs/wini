use wini_cli::{self, cli, init::ask::ask};


fn main() {
    let matches = cli::build();

    if let Some(_matches) = matches.subcommand_matches("init") {
        if let Err(some_err) = ask() {
            eprintln!("{some_err}");
        }
    } else {
        wini_cli::just::run::run_from_arg_match(&matches);
    }
}
