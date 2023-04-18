// Tetris

mod tetrominoe;
mod tetlib;
mod gamescore;
mod musicplayer;

use std::{
    io::{stdout, Write},
    thread::{sleep, spawn},
    time::Duration,
};

use termion::raw::IntoRawMode;
use termion::input::TermRead;

use tetlib::*;
use tetrominoe::Tetrominoe;
use gamescore::GameScore;
use musicplayer::MusicPlayer;

fn main() {
    // music playback
    let mut musicplayer = MusicPlayer::new();
    musicplayer.start("music/korobeiniki.mp3");

    let mut stdin = termion::async_stdin().keys();
    // let mut stdin = std::io::stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;

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
        let key = get_input(&mut stdin);

        // quit
        if key == 'q' {
            break;
        }

        // gravity
        if counter == 10 - gamescore.level {
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

    // stop music
    musicplayer.stop();
    
    // Print prompt below game
    print!("{}{}\r\n", termion::cursor::Show, termion::cursor::Goto(1, (HEIGHT+3) as u16));
}
