#!/bin/bash

# get the directory of macro_counter.
MCTR_DIR="$(dirname "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)")"

cd "$MCTR_DIR" || exit
cargo build --release

cd "$HOME" || exit
cargo install --path "$MCTR_DIR"

# create the two directories which will store files generated by macro_counter.
mkdir -p "$HOME/Documents/Health/Macronutritional_Intake"
mkdir -p "$HOME/Documents/Health/Predefined_Meals"

# copy the launch script to bin
# so that specifying the script path is no longer necessary when executing.
chmod +x "$MCTR_DIR/scripts/mctr.sh"
sudo cp "$MCTR_DIR/scripts/mctr.sh" "/usr/local/bin/"
