# This script is in charge of running the application in dev mode

source ./utils.nu

nu ./scripts/clean-js-without-ts.nu

just compile-ts
just compile-scss

[
    { || watch src --glob="**/*.ts" { || just compile-ts } }
    { || watch src --glob="**/*.scss" { || just compile-scss } }
# IFFEAT ssr
    { || watch . --glob="**/*.rs" { || try { cargo run }; info "Restarting..." } }
# ENDIF
# IFFEAT ssg
    { || watch . --glob="**/*.rs" { || try { cargo run --no-default-features --features run-with-ssr }; info "Restarting..." } }
# ENDIF
    { || watch . --glob="**/*.rs" { || terminate; } }
] | par-each {|c| do $c}

