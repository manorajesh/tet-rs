// Tetris
// render display vector function
// update display vector function

// TODO:
// - add score
// - add level
// - s and z pieces are not rotating correctly

mod tetrominoe;
mod tetris_lib;
mod gamescore;

use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use tetris_lib::{full_line, gravity, handle_input, init, new_piece, render, ghost_piece};
use tetrominoe::Tetrominoe;
use gamescore::GameScore;

fn main() {
    let mut stdin = termion::async_stdin().keys();
    // let mut stdin = std::io::stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    const WIDTH: i32 = 10;
    const HEIGHT: i32 = 20;

    let mut display: Vec<Vec<char>> = init(WIDTH, HEIGHT);
    let mut active_piece = Tetrominoe::new();
    let mut gamescore = GameScore::new();

    print!("{}", termion::cursor::Hide);
    new_piece(&mut display, &mut active_piece);

    let mut counter: usize = 0;

    // main loop
    loop {
        let prev_display = display.clone();
        // handle input
        let key = if let Some(Ok(key)) = stdin.next() {
            match key {
                Key::Char('q') => 'q', // quit
                Key::Left => 'l',      // left
                Key::Right => 'r',     // right
                Key::Char(' ') => 's', // down with spacebar
                Key::Down => 'd',      // down
                Key::Up => 'u',        // rotate
                _ => ' ',
            }
        } else {
            ' '
        };

        // quit
        if key == 'q' {
            break;
        }

        // gravity
        if counter == 5 {
            if gravity(&mut display, &mut active_piece) {
                break;
            }
            counter = 0;
        }

        // handle input
        handle_input(&mut display, key, &mut active_piece);

        // full line
        full_line(&mut display, &mut gamescore);

        // ghost piece
        ghost_piece(&mut display, &mut active_piece);

        // check if display was changed
        let is_updated = display != prev_display;

        // render
        render(&mut display, is_updated, &gamescore);
        sleep(Duration::from_millis(50));
        stdout.flush().unwrap();
        counter += 1;
    }

    print!("{}\r\n\r\n\r\n", termion::cursor::Show);
}
