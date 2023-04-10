// Tetris
// render display vector function
// update display vector function

// TODO:
// - state variable not changing after second new piece
// - right movement not working

use rand::seq::SliceRandom;
use core::panic;
use std::{thread::sleep, time::Duration, io::{stdout, Write}};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const EMPTY: char = '.';

fn render(display: &Vec<Vec<char>>) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1)); // clear screen and move cursor to top left
    for row in display {
        for ch in row {
            match ch {
                &EMPTY => print!(". "),
                'a' => print!("[]"),
                _ => panic!("unknown character: {}", ch),
            }
        }
        print!("\r\n");
    }
}

fn init(width: i32, height: i32) -> Vec<Vec<char>> {
    let mut display: Vec<Vec<char>> = Vec::new();

    for _ in 0..height {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..width {
            row.push(EMPTY);
        }
        display.push(row);
    }
    display
}

fn gravity(display: &mut Vec<Vec<char>>) -> i32 {
    for row in (0..display.len()).rev() {
        for col in 0..display[row].len() {
            if display[row][col] == 'a' {
                if row == display.len()-1 || display[row+1][col] != EMPTY {
                    return 2; // new piece
                }

                display[row][col] = EMPTY;
                display[row+1][col] = 'a';
            }
        }
    }
    0
}

fn handle_input(display: &mut Vec<Vec<char>>, key: char) {
    for row in (0..display.len()).rev() {
        for col in 0..display[row].len() {
            if display[row][col] == 'a' {
                match key {
                    'l' => {
                        if col == 0 || display[row][col-1] != EMPTY {
                            return;
                        }
                        display[row][col] = EMPTY;
                        display[row][col-1] = 'a';
                    }
                    'r' => {
                        if col == display[row].len()-1 || display[row][col+1] != EMPTY {
                            return;
                        }
                        display[row][col] = EMPTY;
                        display[row][col+1] = 'a';
                    }
                    _ => (),
                }
            }
        }
    }
}

fn new_piece(display: &mut Vec<Vec<char>>) -> i32 {
    let half_width = display[0].len() / 2;
    let pieces = vec!['I', 'J', 'L', 'O', 'S', 'T', 'Z'];

    let piece = pieces.choose(&mut rand::thread_rng()).unwrap();
    match piece {
        'I' => {
            // I
            // I
            // I
            // I
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[2][half_width] = 'a';
            display[3][half_width] = 'a';
        }
        'J' => {
            //  J
            //  J
            // JJ
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[2][half_width] = 'a';
            display[2][half_width-1] = 'a';
        }
        'L' => {
            // L
            // L
            // LL
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[2][half_width] = 'a';
            display[2][half_width+1] = 'a';
        }
        'O' => {
            // OO
            // OO
            display[0][half_width] = 'a';
            display[0][half_width+1] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width+1] = 'a';
        }
        'S' => {
            // SS
            //  SS
            display[0][half_width] = 'a';
            display[0][half_width+1] = 'a';
            display[1][half_width-1] = 'a';
            display[1][half_width] = 'a';
        }
        'T' => {
            // T
            // TT
            // T
            display[0][half_width] = 'a';
            display[1][half_width-1] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width+1] = 'a';
        }
        'Z' => {
            //  ZZ
            // ZZ
            display[0][half_width-1] = 'a';
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width+1] = 'a';
        }
        _ => panic!("unknown picece: {}", piece),
    }

    0 // return to moving state
}

fn main() {
    let mut stdin = termion::async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    const WIDTH: i32 = 15;
    const HEIGHT: i32 = 20;
    let mut state = 2; // 0: moving, 2: new piece

    let mut display: Vec<Vec<char>> = init(WIDTH, HEIGHT);
    
    // main loop
    loop {
        // handle input
        let key = if let Some(Ok(key)) = stdin.next() {
            match key {
                Key::Char('q') => 'q',
                Key::Left => 'l',
                Key::Right => 'r',
                _ => ' ',
            }
        } else {
            ' '
        };

        // quit
        if key == 'q' {
            break;
        }

        // spawn new piece
        if state == 2 {
            state = new_piece(&mut display);
        }

        // gravity
        state = gravity(&mut display);

        // handle input
        handle_input(&mut display, key);

        // render
        render(&display);
        sleep(Duration::from_millis(300));
        stdout.flush().unwrap();
    }
}
