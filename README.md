# tet-ris
![Build Status](https://github.com/manorajesh/tet-ris/actions/workflows/MacOS.yml/badge.svg)
![Build Status](https://github.com/manorajesh/tet-ris/actions/workflows/Linux.yml/badge.svg)
![Build Status](https://github.com/manorajesh/tet-ris/actions/workflows/Windows.yml/badge.svg)

Civil-War-Reenactment Style of Tetris 1984 (with some quality-of-life improvements) in Rust!

[![demo](https://asciinema.org/a/0PSmnvMDN4jZkJEKsC8J1bZTG.svg)](https://asciinema.org/a/0PSmnvMDN4jZkJEKsC8J1bZTG?t=5?autoplay=1)

## Installation
```shell
cargo install tet-ris
```
or
```shell
$ git clone https://github.com/manorajesh/tet-ris.git
$ cargo build --release
```

## Usage
```shell
tet-ris
```

### How to Play
The objective of Tetris is to move and rotate falling pieces called tetrominoes in order to create complete horizontal lines. When a line is complete, it will be cleared, and the lines above it will drop down. The game becomes progressively faster as you clear more lines and level up.

### Controls
* `Left Arrow`: Move the active piece left.
* `Right Arrow`: Move the active piece right.
* `Down Arrow`: Move the active piece down (soft drop).
* `Up Arrow`: Rotate the active piece clockwise.
* `Space`: Hard drop the active piece.
* `C`: Hold the active piece.
* `P`: Pause the game.
* `Q`: Quit the game.
