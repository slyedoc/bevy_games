# bevy games

This is a playground for learning Bevy and to play more with rust.

## Current

- Testing out go board rendering

### Examples

```bash
 cargo run --example ~name~
```

- snake
  - Based on [mbuffet tutorial](https://mbuffett.com/posts/bevy-snake-tutorial/) and [repo](https://github.com/marcusbuffett/bevy_snake/)
- flappy-bird
  - based on [drupalio repo](https://github.com/drupalio/flappy_bevy), updated from an earlier of bevy, was great experence learning how bevy has changed and figuring out newer ways of handling things

- pong ***UNFINISHED***

- sudoku
  - from [Alice Cecile](https://github.com/alice-i-cecile) [repo](https://github.com/Leafwing-Studios/bevy-sudoku)
  - This was for a [user experience report](https://github.com/bevyengine/bevy/discussions/2235)
  - Only required a little update to get running on main branch
  - Really cool button state system [puzzle_button](https://github.com/Leafwing-Studios/bevy-sudoku/blob/ui-game-grid/src/input/buttons.rs#L17) "which stores marker components on the system and then pipes them around as events"
  - arranged apps by logic / graphics / input, looks promising

## Resources

[bevy docs](https://docs.rs/bevy/0.5.0/bevy/index.html) |  [bevy cheatbook](https://bevy-cheatbook.github.io/) | [bevy examples](https://github.com/bevyengine/bevy/tree/main/examples)

### Useful Commands

```bash
cargo watch -x "run" -i "assets"
```

**note**: I wrote a plugin that restores the window to the last location, that information gets saved to the assets directory, so if you use watch to restart the app you will want to exclude that directory