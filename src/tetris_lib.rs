use core::panic;
use std::fs::File;
use std::io::Read;

use crate::tetrominoe::Tetrominoe;

pub const EMPTY: char = '.';

pub fn render(display: &Vec<Vec<char>>, is_updated: bool) {
    if is_updated {
        return;
    }

    print!("{}", termion::cursor::Goto(3, 1)); // clear screen and move cursor to top left
    for (c, row) in display.iter().enumerate() {
        for ch in row {
            match ch {
                &EMPTY => print!(" ."),
                'a' => print!("[]"),
                'l' => print!("[]"),
                _ => panic!("unknown character: {}", ch),
            }
        }
        print!("{}", termion::cursor::Goto(3, (c + 2) as u16));
    }
}

pub fn init(width: i32, height: i32) -> Vec<Vec<char>> {
    let mut display: Vec<Vec<char>> = Vec::new();

    // generation
    for _ in 0..height {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..width {
            row.push(EMPTY);
        }
        display.push(row);
    }

    // walls
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1)); // clear screen and move cursor to top left
    for row in &display {
        print!("<!"); // left wall
        for _ in row {
            print!("  ");
        }
        print!("!>"); // right wall
        print!("\r\n");
    }
    print!("<!{}!>\r\n", "=".repeat(display[0].len() * 2)); // bottom wall
    print!("  {}", "\\/".repeat(display[0].len())); // bottom spikes
    display
}

pub fn gravity(display: &mut Vec<Vec<char>>, active_piece: &mut Tetrominoe) -> bool {
    let prev_display = display.clone();
    for row in (0..display.len()).rev() {
        for col in 0..display[row].len() {
            if display[row][col] == 'a' {
                if row == display.len() - 1 || display[row + 1][col] != EMPTY {
                    *display = prev_display;
                    landed(display);
                    let game_over = new_piece(display, active_piece);
                    return game_over;
                }

                display[row][col] = EMPTY;
                display[row + 1][col] = 'a';
            }
        }
    }
    active_piece.row += 1;
    false
}

pub fn handle_input(display: &mut Vec<Vec<char>>, key: char, active_piece: &mut Tetrominoe) {
    let prev_display = display.clone();
    match key {
        'l' => {
            for row in (0..display.len()).rev() {
                for col in 0..display[row].len() {
                    if display[row][col] == 'a' {
                        if col == 0 || display[row][col - 1] != EMPTY {
                            *display = prev_display;
                            return;
                        }
                        display[row][col] = EMPTY;
                        display[row][col - 1] = 'a';
                    }
                }
            }

            if active_piece.col > 0 {
                active_piece.col -= 1;
            }
        }

        'r' => {
            for row in (0..display.len()).rev() {
                for col in (0..display[row].len()).rev() {
                    if display[row][col] == 'a' {
                        if col == display[row].len() - 1 || display[row][col + 1] != EMPTY {
                            *display = prev_display;
                            return;
                        }
                        display[row][col] = EMPTY;
                        display[row][col + 1] = 'a';
                    }
                }
            }
            active_piece.col += 1;
        }

        's' => {
            // bring down piece until new piece is created
            while display[0][display[0].len() / 2] == EMPTY {
                gravity(display, active_piece);
            }
        }

        'd' => {
            gravity(display, active_piece);
        }

        'u' => {
            // rotate piece
            active_piece.rotate();
            if active_piece.row + 4 > display.len() {
                active_piece.row = display.len() - 4;
            }
            
            if active_piece.col + 4 > display[0].len() {
                active_piece.col = display[0].len() - 4;
            }

            // clear piece and replace with new rotated piece
            for row in active_piece.row..active_piece.row + 4 {
                for col in active_piece.col..active_piece.col + 4 {
                    display[row][col] = EMPTY;
                }
            }

            for row in active_piece.row..active_piece.row + 4 {
                for col in active_piece.col..active_piece.col + 4 {
                    display[row][col] = active_piece.shape[row - active_piece.row][col - active_piece.col];
                }
            }
            
        }

        _ => (),
    }
}

pub fn new_piece(display: &mut Vec<Vec<char>>, active_piece: &mut Tetrominoe) -> bool {
    let half_width = display[0].len() / 2;

    // game over
    if display[0][half_width] != EMPTY {
        return true;
    }

    let pieces = vec!['I', 'J', 'L', 'O', 'S', 'T', 'Z'];
    // let pieces = vec!['S'];

    let piece = pieces[getrandom() % pieces.len()];
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
            display[2][half_width - 1] = 'a';
        }
        'L' => {
            // L
            // L
            // LL
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[2][half_width] = 'a';
            display[2][half_width + 1] = 'a';
        }
        'O' => {
            // OO
            // OO
            display[0][half_width] = 'a';
            display[0][half_width + 1] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width + 1] = 'a';
        }
        'S' => {
            // SS
            //  SS
            display[0][half_width] = 'a';
            display[0][half_width + 1] = 'a';
            display[1][half_width - 1] = 'a';
            display[1][half_width] = 'a';
        }
        'T' => {
            // T
            // TT
            // T
            display[0][half_width] = 'a';
            display[1][half_width - 1] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width + 1] = 'a';
        }
        'Z' => {
            //  ZZ
            // ZZ
            display[0][half_width - 1] = 'a';
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width + 1] = 'a';
        }
        _ => panic!("unknown picece: {}", piece),
    }
    active_piece.set(piece);
    active_piece.set_pos(0, (half_width-1) as usize);
    false
}

pub fn landed(display: &mut Vec<Vec<char>>) {
    for row in display {
        for ch in row {
            if *ch == 'a' {
                *ch = 'l';
            }
        }
    }
}

pub fn full_line(display: &mut Vec<Vec<char>>) {
    'outer: for row in (0..display.len()).rev() {
        for ch in &display[row] {
            if *ch != 'l' {
                continue 'outer;
            }
        }
        display.remove(row);
        display.insert(0, vec![EMPTY; display[0].len()]); // add new line at the top
    }
}

fn getrandom() -> usize {
    let mut file = File::open("/dev/urandom").expect("failed to open /dev/urandom");
    let mut bytes = [0; 8];
    file.read_exact(&mut bytes).unwrap();
    usize::from_le_bytes(bytes)
}
