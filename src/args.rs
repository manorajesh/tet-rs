use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Play Tetris 1984ish in your terminal!", long_about = "Civil-War-Reenactment Style of Tetris 1984 (with some quality-of-life improvements) in Rust!\n\nControls: Left and Right arrow keys to move, Up arrow key to rotate, Down arrow key to soft drop, Spacebar to hard drop, 'c' to hold piece, 'q' to quit, and 'p' to pause")]
pub struct Args {
    /// Disable ghost piece shown at the bottom of the board
    #[clap(short, long, action)]
    pub ghost: bool,

    /// Disable hold piece with 'c' key
    #[clap(short = 'c', long = "hold", action)]
    pub hold: bool,

    /// Gravity speed for the game
    #[clap(short = 't', long = "tick", default_value = "10", value_name = "MILLISECONDS")]
    pub gravity: u64,
}