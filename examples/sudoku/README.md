# bevy-sudoku

[from](https://github.com/Leafwing-Studios/bevy-sudoku)

A simple but polished open source Sudoku game written in the Bevy game engine in Rust.

To run this game, simply clone this repo, then use the `cargo run --release` command in the downloaded folder.

## Architecture

This game's core logic flows from input, with the `CommonLabel::Input` system label, into actions, with the `CommonLabel::Action` system label.
To explore these systems, begin at `src/main.rs` and start exploring the plugins.

Input systems are found in the `src/input/` folder, while action systems are found in either `src/logic/` or `src/graphics`, depending on whether they control the gameplay-relevant logic or the display.

The user interface of this game is split into two sections: the Sudoku board, and the supporting button, and this split is reflected in both the `src/input` and `src/graphics` folders.
