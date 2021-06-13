# bevy games

This is a playground for learning Bevy and to play more with rust.

## Current - main.rs

- Testing out go board idea
*UI has been a problem, looking for other options*

### Examples

Below are a collection of simple games from different sources I have been going though and getting working to learn bevy better and learn better way of doing things.  I learn alot updating these and learn what works and doesn't, and having them in one place lets me compair and reference them easier.

```bash
 cargo run --example ~name~
```

- snake
  - Based on [mbuffet tutorial](https://mbuffett.com/posts/bevy-snake-tutorial/) and [repo](https://github.com/marcusbuffett/bevy_snake/)
- flappy-bird
  - based on [drupalio repo](https://github.com/drupalio/flappy_bevy), updated from an earlier of bevy, was great experence learning how bevy has changed and figuring out newer ways of handling things

- sudoku
  - from [Alice Cecile](https://github.com/alice-i-cecile) [repo](https://github.com/Leafwing-Studios/bevy-sudoku)
  - This was for a [user experience report](https://github.com/bevyengine/bevy/discussions/2235)
  - Only required a little update to get running on main branch
  - Really cool button state system [puzzle_button](https://github.com/Leafwing-Studios/bevy-sudoku/blob/ui-game-grid/src/input/buttons.rs#L17) "which stores marker components on the system and then pipes them around as events"
  - arranged apps by logic / graphics / input, looks promising

- asteroid
  - Was called Kataster, I keep forgetting the name
  - from [repo](https://github.com/scnsh/Kataster) by https://github.com/scnsh/, this was an updated version of [repo](https://github.com/Bobox214/Kataster) by [Bobox214](https://github.com/Bobox214/) updated for bevy 0.5
  - Makes use of [Rapier](https://github.com/dimforge/bevy_rapier), a simd 2D and 3D physics engines for the Bevy

## Resources

[bevy docs](https://docs.rs/bevy/0.5.0/bevy/index.html) |  [bevy cheatbook](https://bevy-cheatbook.github.io/) | [bevy examples](https://github.com/bevyengine/bevy/tree/main/examples)

### Useful Commands

```bash
cargo watch -x "run" -i "assets"
```

**note**: I wrote a plugin that restores the window to the last location, that information gets saved to the assets directory, so if you use watch to restart the app you will want to exclude that directory