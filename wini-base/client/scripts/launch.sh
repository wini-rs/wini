#!/usr/bin/env bash

# This script is in charge of launching the server

source ./scripts/terminate.sh


terminate
cargo run
