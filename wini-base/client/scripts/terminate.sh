#!/usr/bin/env bash

get_port() {
    rg '^PORT=' .env | cut -d'=' -f2
}

terminate() {
    pid_to_kill="$(ss -tulnp | rg "$(get_port)" | sed -E 's/.*pid=([0-9]+).*/\1/g')"
    [ -n "$pid_to_kill" ] && kill -9 "$pid_to_kill"
}
