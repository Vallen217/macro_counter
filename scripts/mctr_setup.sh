#!/bin/bash

# get the directory of macro_counter.
MCTR_DIR="$(dirname "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)")"

cd "$MCTR_DIR" || exit
cargo build --release

cd "$HOME" || exit
cargo install --path "$MCTR_DIR"
