# Github Game Jam 2018 entry

## Theme
[HYBRID.](https://itch.io/jam/game-off-2018)

## Concept
A mashup of Lemmings and an endless runner sorta thing. The camera tracks the lead creep and using an ability (lift, change direction, dash) kills them.

## Controls
  - ``1`` -> lift
  - ``2`` -> change direction
  - ``3`` -> dash
  - ``r`` -> restart level
  - ``n`` -> next level (if % NEEDED is met)
  - ``p`` -> previous level
  - ``+`` -> volume up
  - ``-`` -> volume down

## Implementation
  - Language: [Rust](https://www.rust-lang.org/)
  - Engine: [Amethyst](https://www.amethyst.rs/)
  - Physics: [nphysics](https://www.nphysics.org/)

## License
[MIT License](LICENSE-MIT)

## Preview
![Level 2](screenshots/level_2.gif "level 2")
![Loads of creeps](screenshots/loads_of_creeps.gif "Loads of creeps")

## Building
  - Install rust [(rustup makes it easy)](https://rustup.rs/)
  - Install [Amethyst dependencies](https://github.com/amethyst/amethyst#dependencies)
  - Clone this repo
  - Run run.sh for rust nightly or run_stable.sh for rust stable

## Binaries
You can download pre-built binaries from the [itch.io page](https://cs2dsb.itch.io/lemrunner)