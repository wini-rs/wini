# Scripts
This folder contains useful scripts and their related files, for managing the project.

- These scripts are made to work with [`just`](https://github.com/casey/just), that you can find in `./justfile`.
- They **MUST** be run from the root of the project (where there is the `./wini.toml` or `./justfile`). This is done by default when running `just`.
- All the scripts are made to work with `bash`. They haven't been tested to work with something else.
- You should use [`nix develop`](https://nixos.org/learn) to have all the dependencies installed. If not, see the required packages in `./flake.nix`.

If all these scripts are included here, it's because they are meant to be customizable, so feel free to modify them in your project!
Below is a quick overview of what each script is doing.

## Overview
- `ask-path-package.sh`: Ask the user the path of the files to include from the javascript package they installed
- `clean-js-without-ts.sh`: Clean javascript files that don't have an associated typescript file.
- `fmt.sh`: Format files of the project
- `launch.sh`: Terminate all the running servers and launch a new one
- `lint.sh`: Lint the project
- `log.sh`: Provide utilities function for other scripts. 
- `new.sh`: Creates a new *something*. This *something* can be, a page, a layout or a component.
- `run.sh`: Run the project in a dev environment.
- `scss.sh`: Compile all the scss files into css.
- `sync-packages.sh`: Sync packages from node_modules to `modules`
- `terminate.sh`: Terminates the current runnign wini server.
- `typescript.sh`: Compile all the ts files into js.
- `yesno.sh`: Provide utility function for `[y/n]` prompts in bash.
- `template/`: Template for creating *something* when running `./new.sh`
