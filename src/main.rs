// Tetris

mod gamescore;
mod tetlib;
mod tetrominoe;

use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::Show,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use gamescore::GameScore;
use tetlib::*;
use tetrominoe::Tetrominoe;

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

        if key == 'p' {
            let mut key = get_input();
            put_text(WIDTH as u16, HEIGHT as u16, "P A U S E D");
            stdout.flush().unwrap();
            while key != 'p' {
                key = get_input();
            }
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
            hold(
                &mut display,
                &mut active_piece,
                &mut hold_piece,
                &mut next_piece,
            );
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

    put_text(WIDTH as u16, HEIGHT as u16, "G A M E  O V E R");
    disable_raw_mode().unwrap();
    // execute!(stdout, LeaveAlternateScreen).unwrap();
    execute!(stdout, Show).unwrap();
    print!("{}", "\n".repeat(HEIGHT / 2 + 1))
}
