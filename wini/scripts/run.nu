# This script is in charge of running the application in dev mode

source ./utils.nu

nu ./scripts/clean-js-without-ts.nu

just compile-ts
just compile-scss

# IFFEAT ssr
watchexec -i "target/**" -i "node_modules/**" -e 'ts,scss,rs' -d 100 --stop-signal SIGTERM  -r "just compile-ts; just compile-scss; cargo run"
# ENDIF
# IFFEAT ssg
watchexec -i "target/**" -i "node_modules/**" -e 'ts,scss,rs' -d 100 --stop-signal SIGTERM  -r "just compile-ts; just compile-scss; cargo run --features --no-default-features --features run-with-ssr"
# ENDIF
