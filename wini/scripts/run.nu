# This script is in charge of running the application in dev mode

source ./utils.nu

nu ./scripts/clean-js-without-ts.nu

just compile-ts
just compile-scss

[
    { || watch src --glob="**/*.ts" { || just compile-ts } }
    { || watch src --glob="**/*.scss" { || just compile-scss } }
    { || watch . --glob="**/*.rs" { || try { cargo run }; info "Restarting..." } }
    { || watch . --glob="**/*.rs" { || terminate; } }
] | par-each {|c| do $c}

