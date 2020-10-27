# Todo
A simple CLI based todo list management tool.

## Setup
### Step 1: Clone this repo.
    git clone https://github.com/dak-x/todo

### Step 2: Build
Execute the build file from your console using your favourite `shell`

    ./build.sh

The binary is located at `target/release/todo`. Copy it to your `/bin` folder to access directly from your shell.
The default list file is stored at `~/.config/todo/todo.json`. You can set your custom file by changing the `CONST CONFIG_PATH` variable in `src/lib.rs` and then rebuilding using `cargo build --release`

# Usage
Run `todo --help` to read the documentation. For each of the 4 commands view additional help via `todo sbcmd --help`

Feel free to Contribute!
<!-- # Currently under development !!!  -->