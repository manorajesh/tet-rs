// Tetris

mod args;
mod bag;
mod gamescore;
mod gamestate;
mod tetlib;
mod tetrominoe;
mod ai;

use std::{ io::{ stdout, Write }, thread::sleep, time::Duration };

use crossterm::{
    cursor::Show,
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};

use clap::Parser;

use gamestate::GameState;
use tetlib::*;
use crate::ai::AI;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

fn main() {
    let mut args = args::Args::parse();

    if args.og {
        args.no_colors = true;
        args.chars = "[]".to_string();
    }

    const MAX_LEVEL: usize = 20;
    const GRAV_TICK: usize = 40;
    const LEVEL_MULT: f64 = 0.85;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
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

    if args.sirtet {
        sirtet_borders(WIDTH, HEIGHT);
    }

    if args.ai {
        gs.enable_ai();
    }

    // loop for new game
    loop {
        // game loop
        loop {
            let prev_display = gs.display.clone();

            // handle input
            let mut key = get_input();

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

            if true {
                let mv = gs.find_best_move();
                if mv.x < gs.active_piece.col {
                    key = 'l';
                } else if mv.x > gs.active_piece.col {
                    key = 'r';
                } else if mv.rotation_state != gs.active_piece.rotation_state {
                    key = 'u';
                }

                // debug
                put_text(0, 50, format!("move: {:?}", mv).as_str());

                // hard drop if in correct position
                if key == ' ' {
                    key = 'd';
                }
            }

            // gravity
            if
                gs.counter >=
                (((GRAV_TICK as f64) * LEVEL_MULT.powf(gs.gamescore.level as f64)) as usize)
            {
                if gravity(&mut gs) {
                    gs.is_game_over = true;
                    break;
                }
                gs.counter = if gs.gamescore.level < MAX_LEVEL { 0 } else { 100 };
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
            render(&mut gs, is_updated, &args.chars, &args.no_colors, &args.sirtet);
            sleep(Duration::from_millis(args.gravity));
            stdout.flush().unwrap();
            gs.counter += 1;
        }

        // put_text(WIDTH as u16, HEIGHT as u16, "G A M E  O V E R");
        if !gs.serial() {
            break;
        }
        gs = GameState::new(WIDTH, HEIGHT);
    }
    disable_raw_mode().unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap();
    execute!(stdout, Show).unwrap();
}

fn path_exists(path: &String) -> bool {
    std::path::Path::new(path).exists()
}
