// Tetris

mod gamescore;
mod tetlib;
mod tetrominoe;
mod args;
mod gamestate;

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

use clap::Parser;

use tetlib::*;
use gamestate::GameState;

fn main() {
    let args = args::Args::parse();

    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const MAX_LEVEL: usize = 20;
    const GRAV_TICK: usize = 25;
    const LEVEL_MULT: f64 = 0.85;

    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    let mut gs = if let Some(path) = args.save {
        init(WIDTH, HEIGHT);
        GameState::deserial(Some(path), WIDTH, HEIGHT)
    } else {
        let mut gs = GameState::new(WIDTH, HEIGHT);
        new_piece(&mut gs.display, &mut gs.active_piece, None, &mut gs.next_piece);
        gs
    };

    // main loop
    loop {
        let prev_display = gs.display.clone();

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
        if gs.counter >= (GRAV_TICK as f64*LEVEL_MULT.powf(gs.gamescore.level as f64)) as usize {
            if gravity(&mut gs.display, &mut gs.active_piece, &mut gs.next_piece) {
                break;
            }
            gs.counter = if gs.gamescore.level < MAX_LEVEL { 0 } else { 100 };
        }

        // handle input
        handle_input(&mut gs.display, key, &mut gs.active_piece, &mut gs.next_piece);

        // hold piece
        if key == 'c' && !args.hold {
            hold(
                &mut gs.display,
                &mut gs.active_piece,
                &mut gs.hold_piece,
                &mut gs.next_piece,
            );
        }

        // full line
        full_line(&mut gs.display, &mut gs.gamescore);

        // ghost piece
        if !args.ghost{
            ghost_piece(&mut gs.display, &mut gs.active_piece);
        }

        // check if gs.display was changed
        let is_updated = gs.display != prev_display;

        // render
        render(&gs.display, is_updated, &gs.gamescore, &gs.hold_piece, &gs.next_piece);
        sleep(Duration::from_millis(args.gravity));
        stdout.flush().unwrap();
        gs.counter += 1;
    }

    gs.serial(None);
    put_text(WIDTH as u16, HEIGHT as u16, "G A M E  O V E R");
    disable_raw_mode().unwrap();
    execute!(stdout, Show).unwrap();
    print!("{}", "\n".repeat(HEIGHT / 2 + 4))
}

#[test]
fn test_main(){
    main();
}