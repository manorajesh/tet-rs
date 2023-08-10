# tet-ris
![Build Status](https://github.com/manorajesh/tet-ris/actions/workflows/MacOS.yml/badge.svg)
![Build Status](https://github.com/manorajesh/tet-ris/actions/workflows/Linux.yml/badge.svg)
![Build Status](https://github.com/manorajesh/tet-ris/actions/workflows/Windows.yml/badge.svg)
![Downloads](https://img.shields.io/crates/d/tet-ris)
![Version](https://img.shields.io/crates/v/tet-ris)
![License](https://img.shields.io/crates/l/tet-ris)

[Civil-War-Reenactment](https://www.reddit.com/r/programming/comments/wu56a0/comment/ilauaz0/?utm_source=share&utm_medium=web3x) Style of Tetris 1984 (with some quality-of-life improvements) in Rust! 

<sub><sup>play upside-down as well!</sup></sub>

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
### `-h` Output
```shell
Play Tetris 1984ish in your terminal!

Usage: tet-ris [OPTIONS]

Options:
  -g, --ghost                 Disable ghost piece shown at the bottom of the board
  -c, --hold                  Disable hold piece with 'c' key
  -t, --tick <MILLISECONDS>   Gravity speed for the game [default: 10]
  -s, --save <FILE>           Path to save file [default: save.tetris]
      --chars <2 CHARACTERS>  Characters to use for tetrominoes [default: ██]
      --no-colors             Disable colors
  -o, --original              Return to 1984 Tetris
      --sirtet                Play Sirtet (Upside-down Tetris)
  -h, --help                  Print help (see more with '--help')
  -V, --version               Print version
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

### Gameplay Images

<img src="https://github.com/manorajesh/tet-ris/blob/master/images/color.png?raw=true" width=400>

<img src="https://github.com/manorajesh/tet-ris/blob/master/images/sirtet.png?raw=true" width=400>

<img src="https://github.com/manorajesh/tet-ris/blob/master/images/game.png?raw=true" width=400>

<img src="https://github.com/manorajesh/tet-ris/blob/master/images/game_over.png?raw=true" width=400>