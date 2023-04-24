// Tetris

mod tetrominoe;
mod tetlib;
mod gamescore;

use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{execute, terminal::{enable_raw_mode, disable_raw_mode}, cursor::Show};

use tetlib::*;
use tetrominoe::Tetrominoe;
use gamescore::GameScore;

fn main() {
    let mut stdout = stdout();

    // Enter alternate screen and hide cursor
    // execute!(stdout, EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();

    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;

    let mut display: Vec<Vec<char>> = init(WIDTH, HEIGHT);
    let mut active_piece = Tetrominoe::new();
    let mut gamescore = GameScore::new();
    let mut hold_piece: Option<Tetrominoe> = None;
    let mut next_piece = Tetrominoe::random();
    new_piece(&mut display, &mut active_piece, None, &mut next_piece);

    let mut counter: usize = 0;

    // main loop
    loop {
        let prev_display = display.clone();

        // handle input
        let key = get_input();

        // quit
        if key == 'q' {
            break;
        }

        // gravity
        if counter == 10 - gamescore.level {
            if gravity(&mut display, &mut active_piece, &mut next_piece) {
                break;
            }
            counter = 0;
        }

        // handle input
        handle_input(&mut display, key, &mut active_piece, &mut next_piece);

        // hold piece
        if key == 'c' {
            hold(&mut display, &mut active_piece, &mut hold_piece, &mut next_piece);
        }

        // full line
        full_line(&mut display, &mut gamescore);

        // ghost piece
        ghost_piece(&mut display, &mut active_piece);

        // check if display was changed
        let is_updated = display != prev_display;

        // render
        render(&display, is_updated, &gamescore, &hold_piece, &next_piece);
        sleep(Duration::from_millis(50));
        stdout.flush().unwrap();
        counter += 1;
    }

    // Leave alternate screen and show cursor
    disable_raw_mode().unwrap();
    // execute!(stdout, LeaveAlternateScreen).unwrap();
    execute!(stdout, Show).unwrap();
    print!("{}", "\n".repeat(HEIGHT/2+1))
}
