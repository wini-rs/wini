use {clap::ArgMatches, std::ffi::OsString};

pub fn resolve_args_from_arg_match(arg_match: &ArgMatches) -> Option<Vec<OsString>> {
    arg_match.subcommand().map(|arg| {
        // ...
        let mut args = vec!["just".into(), arg.0.into()];

        if let Some(new_args) = resolve_args_from_arg_match(arg.1) {
            args.extend(new_args);
        }

        args.extend(arg.1.ids().flat_map(|id| {
            let mut res = Vec::new();

            if let Some(occurrences) = arg.1.get_occurrences::<String>(id.as_ref()) {
                for mut occurrence_values in occurrences {
                    for occurrence in occurrence_values.by_ref() {
                        res.push(occurrence.into());
                    }
                }
            }

            res
        }));

        args
    })
}

pub fn run_from_arg_match(arg_match: &ArgMatches) {
    let args = resolve_args_from_arg_match(arg_match).unwrap_or_default();
    let _ = pub_just::run(args.iter());
}
