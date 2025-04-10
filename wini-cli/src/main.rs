use wini::{self, cli, init::ask::ask};


fn main() {
    let matches = cli::build().unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    });

    if let Some(_matches) = matches.subcommand_matches("init") {
        if let Err(some_err) = ask() {
            eprintln!("{some_err}");
            std::process::exit(1);
        }
    } else {
        wini::just::run::run_from_arg_match(&matches);
    }
}
