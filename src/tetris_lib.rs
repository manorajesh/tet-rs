use core::panic;
use rand::seq::SliceRandom;

pub const EMPTY: char = '.';

pub fn render(display: &Vec<Vec<char>>, is_updated: bool) {
    if is_updated {
        return;
    }

    print!("{}", termion::cursor::Goto(4, 1)); // clear screen and move cursor to top left
    for (c, row) in display.iter().enumerate() {
        for ch in row {
            match ch {
                &EMPTY => print!(". "),
                'a' => print!("[]"),
                'l' => print!("[]"),
                _ => panic!("unknown character: {}", ch),
            }
        }
        print!("{}", termion::cursor::Goto(4, (c+2) as u16));
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
        print!(" !>"); // right wall
        print!("\r\n");
    }
    print!("<!{}!>\r\n", "=".repeat(display[0].len() * 2+1)); // bottom wall
    print!("  {}", "\\/".repeat(display[0].len())); // bottom spikes
    display
}

pub fn gravity(display: &mut Vec<Vec<char>>) -> bool {
    let prev_display = display.clone();
    for row in (0..display.len()).rev() {
        for col in 0..display[row].len() {
            if display[row][col] == 'a' {
                if row == display.len() - 1 || display[row + 1][col] != EMPTY {
                    *display = prev_display;
                    landed(display);
                    let game_over = new_piece(display);
                    return game_over;
                }

                display[row][col] = EMPTY;
                display[row + 1][col] = 'a';
            }
        }
    }
    false
}

pub fn handle_input(display: &mut Vec<Vec<char>>, key: char) {
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
        }

        's' => {
            // bring down piece until new piece is created
            while display[0][display[0].len() / 2] == EMPTY {
                gravity(display);
            }
        }

        'd' => {
            gravity(display);
        }

        _ => (),
    }
}

pub fn new_piece(display: &mut Vec<Vec<char>>) -> bool {
    let half_width = display[0].len() / 2;

    // game over
    if display[0][half_width] != EMPTY {
        return true;
    }

    let pieces = vec!['I', 'J', 'L', 'O', 'S', 'T', 'Z'];
    // let pieces = vec!['I'];

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