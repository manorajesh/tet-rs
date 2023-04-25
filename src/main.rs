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
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const MAX_LEVEL: usize = 20;
    const GRAV_TICK: usize = 25;
    const LEVEL_MULT: f64 = 0.85;

    let mut stdout = stdout();

    enable_raw_mode().unwrap();

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
            while key != 'p' && key != 'q' {
                key = get_input();
                sleep(Duration::from_millis(10));
            }
        }

        // gravity
        if counter >= (GRAV_TICK as f64*LEVEL_MULT.powf(gamescore.level as f64)) as usize {
            if gravity(&mut display, &mut active_piece, &mut next_piece) {
                break;
            }
            counter = if gamescore.level < MAX_LEVEL { 0 } else { 100 };
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
        sleep(Duration::from_millis(10));
        stdout.flush().unwrap();
        counter += 1;
    }

    put_text(WIDTH as u16, HEIGHT as u16, "G A M E  O V E R");
    disable_raw_mode().unwrap();
    // execute!(stdout, LeaveAlternateScreen).unwrap();
    execute!(stdout, Show).unwrap();
    print!("{}", "\n".repeat(HEIGHT / 2 + 4))
}
