# Repository:    https://github.com/casey/just
# Documentation: https://just.systems/man


# Enter the development environment
env:
    nix develop -c "$SHELL"

# Run in dev
run:
    @./scripts/run.sh

check: compile-ts compile-scss
    cargo check

# Create the binary for the production
build-prod: compile-ts compile-scss
    ENV_TYPE="PROD" cargo build --release

# Run in production
run-prod: js-i js-sync-packages compile-ts compile-scss
    ENV_TYPE="PROD" cargo run --release

# Create the binary for the staging
build-staging: compile-ts compile-scss
    ENV_TYPE="STAGING" cargo build --release

# Run in staging
run-staging: js-i js-sync-packages compile-ts compile-scss
    ENV_TYPE="STAGING" cargo run --release



# Create a new *something* based on a template
new kind:
    @./scripts/new.sh {{kind}}

# Format
fmt:
    @./scripts/fmt.sh

# Lint
lint:
    @./scripts/lint.sh

# Updates the template that you're using by pulling latest commits
update-template:
    @./scripts/update-template.sh

# Adds a javascript package
js-add pkg: && js-sync-packages
    bun a {{pkg}}
    @echo -e "\e[1m=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
    bun i
    @echo -e "\e[1m=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
    @echo -e "Adding {{pkg}} to ./packages-files.toml\e[0m"
    @./scripts/ask-path-packages.sh {{pkg}}

# Removes a javascript package
js-rm pkg: && js-sync-packages
    bun rm {{pkg}}
    @sed -i '/^{{pkg}}\s*=/d' ./packages-files.toml

# Update javascript packages to their new version
js-update: && js-sync-packages
    bun update

js-i:
    bun i

# Sync all the javascript packages in ./public/modules/
js-sync-packages:
    @./scripts/sync-packages.sh


# Compile and watch for SCSS files
compile-scss:
    @echo -e '\e[34m[\e[32m+\e[34m]\e[0m Reloading SCSS...'
    @./scripts/scss.sh

# Compile and watch for Typescript files
compile-ts:
    @echo -e '\e[34m[\e[32m+\e[34m]\e[0m Reloading TypeScript...'
    @./scripts/typescript.sh

# Terminate process running on port `PORT` and start a new server
clean-launch:
    @./scripts/launch.sh
