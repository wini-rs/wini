#!/usr/bin/env bash

# This script is in charge of running the application in dev mode

source ./scripts/log.sh
source ./scripts/terminate.sh


on_interrupt() {
    pkill -f -9 watchexec
    terminate
    info 'Terminating...'
}

./scripts/clean-js-without-ts.sh

trap on_interrupt SIGINT

watchexec -i "node_modules/**" -e 'ts' -r just compile-ts &
watchexec -i "node_modules/**" -e 'scss' -r just compile-scss &
watchexec -i "target/**" -i "node_modules/**" -e 'rs,ts,md,scss,svg,json,sh' --stop-signal SIGTERM  -r "cargo run"
