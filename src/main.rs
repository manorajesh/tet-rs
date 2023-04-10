// Tetris
// render display vector function
// update display vector function

use rand::seq::SliceRandom;
use std::{thread::sleep, time::Duration, io::{stdout, Write}};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn render(display: &Vec<Vec<char>>) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1)); // clear screen and move cursor to top left
    for row in display {
        for ch in row {
            match ch {
                '.' => print!(". "),
                'a' => print!("[]"),
                _ => panic!("unknown character: {}", ch),
            }
        }
        println!();
    }
}

fn update(display: &mut Vec<Vec<char>>, state: i32, key: char) -> i32 {
    let half_width = display[0].len()/2;
    match state {
        0 => { // moving

            0
        },

        1 => { // landed
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
        },

        2 => { // game over
            2
        },
        
        _ => panic!("unknown state: {}", state),
    }
}

fn init(width: i32, height: i32) -> Vec<Vec<char>> {
    let mut display: Vec<Vec<char>> = Vec::new();

    for _ in 0..height {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..width {
            row.push('.');
        }
        display.push(row);
    }
    display
}

fn main() {
    let mut stdin = termion::async_stdin().keys();
    let stdout = stdout().into_raw_mode().unwrap();

    let width = 15;
    let height = 20;
    let mut state = 1; // 0: moving, 1: landed, 2: game over

    let mut display: Vec<Vec<char>> = init(width, height);
    
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

        if key == 'q' {
            break;
        }

        state = update(&mut display, state, key);
        if state == 2 {
            break;
        }
        render(&display);
        stdout.lock().flush().unwrap();
        sleep(Duration::from_millis(1000));
    }
}
