use std::fs::File;
use std::io::Read;

use termion::event::Key;
use termion::input::Keys;

use crate::tetrominoe::Tetrominoe;
use crate::gamescore::GameScore;

pub const EMP: char = '.';

pub fn render(display: &Vec<Vec<char>>, is_updated: bool, score: &GameScore) {
    if !is_updated {
        return;
    }

    print!("{}", termion::cursor::Goto(3, 1)); // clear screen and move cursor to top left
    for (c, row) in display.iter().enumerate() {
        for ch in row {
            match ch {
                &EMP => print!(" ."),
                'a' => print!("[]"),
                'l' => print!("[]"),
                'g' => print!("{}//{}", termion::color::Fg(termion::color::LightBlack), termion::color::Fg(termion::color::Reset)),
                _ => panic!("unknown character: {}", ch),
            }
        }
        print!("{}", termion::cursor::Goto(3, (c + 2) as u16));
    }

    print!("{}", termion::cursor::Goto((display.len() * 2-10) as u16, 1));
    print!("Score: {}", score.score);
    print!("{}", termion::cursor::Goto((display.len() * 2-10) as u16, 3));
    print!("Level: {}", score.level);
    print!("{}", termion::cursor::Goto((display.len() * 2-10) as u16, 5));
    let time = score.get_time();
    print!("Time: {}:{:02}", time / 60, time % 60);
}

pub fn init(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut display: Vec<Vec<char>> = Vec::new();

    // generation
    for _ in 0..height {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..width {
            row.push(EMP);
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
                if row == display.len() - 1 || display[row + 1][col] == 'l' {
                    *display = prev_display;
                    landed(display);
                    let game_over = new_piece(display, active_piece);
                    return game_over;
                }

                display[row][col] = EMP;
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
                        if col == 0 || display[row][col - 1] == 'l' {
                            *display = prev_display;
                            return;
                        }
                        display[row][col] = EMP;
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
                        if col == display[row].len() - 1 || display[row][col + 1] == 'l' {
                            *display = prev_display;
                            return;
                        }
                        display[row][col] = EMP;
                        display[row][col + 1] = 'a';
                    }
                }
            }
            active_piece.col += 1;
        }

        's' => {
            // bring down piece until new piece is created
            while display[0][display[0].len() / 2] == EMP {
                gravity(display, active_piece);
            }
        }

        'd' => {
            gravity(display, active_piece);
        }

        'u' => {
            let prev_display = display.clone();
            let prev_piece = active_piece.clone();

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
                    if display[row][col] == 'l' {
                        continue;
                    }
                    display[row][col] = EMP;
                }
            }

            for row in active_piece.row..active_piece.row + 4 {
                for col in active_piece.col..active_piece.col + 4 {
                    if display[row][col] == 'l' {
                        *display = prev_display;
                        *active_piece = prev_piece;
                        return;
                    }

                    if active_piece.shape[row - active_piece.row][col - active_piece.col] == 'a' {
                        display[row][col] = active_piece.shape[row - active_piece.row][col - active_piece.col];
                    }
                }
            }
        }

        _ => (),
    }
}

pub fn new_piece(display: &mut Vec<Vec<char>>, active_piece: &mut Tetrominoe) -> bool {
    let half_width = display[0].len() / 2;

    // game over
    if display[0][half_width] != EMP {
        return true;
    }

    let pieces = vec!['I', 'J', 'L', 'O', 'S', 'T', 'Z'];
    // let pieces = vec!['Z', 'S'];

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

pub fn full_line(display: &mut Vec<Vec<char>>, score: &mut GameScore) {
    'outer: for row in (0..display.len()).rev() {
        for ch in &display[row] {
            if *ch != 'l' {
                continue 'outer;
            }
        }
        display.remove(row);
        display.insert(0, vec![EMP; display[0].len()]); // add new line at the top
        score.score += 100;
        score.level += score.score / 1000;
    }
}

fn getrandom() -> usize {
    let mut file = File::open("/dev/urandom").expect("failed to open /dev/urandom");
    let mut bytes = [0; 8];
    file.read_exact(&mut bytes).unwrap();
    usize::from_le_bytes(bytes)
}

pub fn ghost_piece(display: &mut Vec<Vec<char>>, active_piece: &mut Tetrominoe) {
    for row in 0..display.len() {
        for col in 0..display[row].len() {
            if display[row][col] == 'g' {
                display[row][col] = EMP;
            }
        }
    }
    
    let mut ghost = display.clone();
    let mut active_piece = active_piece.clone();

    gravity_until_new_piece(&mut ghost, &mut active_piece);

    for row in 0..ghost.len() {
        for col in 0..ghost[row].len() {
            if ghost[row][col] == 'a' && display[row][col] == EMP {
                display[row][col] = 'g';
            }
        }
    }
}

fn gravity_until_new_piece(display: &mut Vec<Vec<char>>, active_piece: &mut Tetrominoe) {
    let mut prev_display = display.clone();
    gravity(display, active_piece);
    while display[0][display[0].len() / 2] == EMP {
        prev_display = display.clone();
        gravity(display, active_piece);
    }
    *display = prev_display;
}

pub fn get_input<T: Read>(stdin: &mut Keys<T>) -> char {
    let key = if let Some(Ok(key)) = stdin.next() {
        match key {
            Key::Char('q') => 'q', // quit
            Key::Left => 'l',      // left
            Key::Right => 'r',     // right
            Key::Char(' ') => 's', // down with spacebar
            Key::Down => 'd',      // down
            Key::Up => 'u',        // rotate
            _ => ' ',
        }
    } else {
        ' '
    };

    key
}