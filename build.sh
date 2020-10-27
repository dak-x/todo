#!/bin/bash

echo "const CONFIG_PATH: &str = \"$HOME/.config/todo/todo.json\";" >> "src/lib.rs"
mkdir ~/.config/todo 
cp todo_template.json ~/.config/todo/todo.json
cargo build --release
target/release/todo --author "$USERNAME"
