// Tetris

mod args;
mod gamescore;
mod gamestate;
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

use clap::Parser;

use gamestate::GameState;
use tetlib::*;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

fn main() {
    let args = args::Args::parse();

    const MAX_LEVEL: usize = 20;
    const GRAV_TICK: usize = 40;
    const LEVEL_MULT: f64 = 0.85;

    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    let mut gs = if let Some(path) = &args.save {
        if path_exists(path) {
            GameState::deserial(path.clone(), WIDTH, HEIGHT)
        } else {
            GameState::new(WIDTH, HEIGHT)
        }
    } else {
        GameState::new(WIDTH, HEIGHT)
    };

    if gs.is_game_over {
        gs.gamescore.stop_timer();
    }

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
        if gs.counter >= (GRAV_TICK as f64 * LEVEL_MULT.powf(gs.gamescore.level as f64)) as usize {
            if gravity(&mut gs) {
                gs.is_game_over = true;
                break;
            }
            gs.counter = if gs.gamescore.level < MAX_LEVEL {
                0
            } else {
                100
            };
        }

        // handle input
        handle_input(&mut gs, key);

        // hold piece
        if key == 'c' && !args.hold {
            hold(&mut gs);
        }

        // full line
        full_line(&mut gs);

        // ghost piece
        if !args.ghost {
            ghost_piece(&mut gs);
        }

        // check if gs.display was changed
        let is_updated = gs.display != prev_display || gs.is_game_over;

        // render
        render(&mut gs, is_updated);
        sleep(Duration::from_millis(args.gravity));
        stdout.flush().unwrap();
        gs.counter += 1;
    }

    // put_text(WIDTH as u16, HEIGHT as u16, "G A M E  O V E R");
    gs.serial();
    disable_raw_mode().unwrap();
    print!("{}", "\n".repeat(HEIGHT / 2 + 4));
    execute!(stdout, Show).unwrap();
}

fn path_exists(path: &String) -> bool {
    std::path::Path::new(path).exists()
}
