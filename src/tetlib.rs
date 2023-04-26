use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, KeyEventKind},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crate::gamescore::GameScore;
use crate::tetrominoe::Tetrominoe;

pub const EMP: char = '.';

pub fn render(
    display: &Vec<Vec<char>>,
    is_updated: bool,
    score: &mut GameScore,
    hold_piece: &Option<Tetrominoe>,
    next_piece: &Tetrominoe,
) {
    if !is_updated {
        return;
    }

    let mut stdout = stdout();
    let width: u16 = display[0].len() as u16;

    stdout.queue(MoveTo(width + 3, 1)).unwrap(); // move cursor to top left
    for (c, row) in display.iter().enumerate() {
        for ch in row {
            match ch {
                &EMP => {
                    stdout.queue(Print(" .")).unwrap();
                }
                'a' => {
                    stdout.queue(Print("[]")).unwrap();
                }
                'l' => {
                    stdout.queue(Print("[]")).unwrap();
                }
                'g' => {
                    stdout
                        .queue(SetForegroundColor(Color::Rgb {
                            r: 50,
                            g: 50,
                            b: 50,
                        }))
                        .unwrap()
                        .queue(Print("//"))
                        .unwrap()
                        .queue(ResetColor)
                        .unwrap();
                }

                _ => panic!("unknown character: {}", ch),
            }
        }
        stdout.queue(MoveTo(width + 3, (c + 2) as u16)).unwrap();
    }

    // hold piece
    stdout.queue(MoveTo(2, 1)).unwrap();
    stdout.queue(Print("Hold:")).unwrap();
    stdout.queue(MoveTo(2, 3)).unwrap();
    match hold_piece {
        Some(piece) => {
            let mut blank = Tetrominoe::new();
            let upright = blank.set(piece.ptype);
            for row in 0..upright.shape.len() {
                for col in 0..upright.shape[row].len() {
                    if upright.shape[row][col] == 'a' {
                        stdout.queue(Print("[]")).unwrap();
                    } else {
                        stdout.queue(Print("  ")).unwrap();
                    }
                }
                stdout.queue(MoveTo(2, (row + 4) as u16)).unwrap();
            }
        }

        None => (),
    }

    // print stats
    stdout.queue(MoveTo(width * 4, 1)).unwrap();
    stdout
        .queue(Print(format!("Score: {}", score.score)))
        .unwrap();
    stdout.queue(MoveTo(width * 4, 3)).unwrap();
    stdout
        .queue(Print(format!("Level: {}", score.level)))
        .unwrap();
    stdout.queue(MoveTo(width * 4, 5)).unwrap();
    score.update();
    let time = score.get_time();
    stdout
        .queue(Print(format!("Time: {}:{:02}", time / 60, time % 60)))
        .unwrap();

    // next piece
    stdout.queue(MoveTo(width * 4, 8)).unwrap();
    stdout.queue(Print("Next:")).unwrap();
    stdout.queue(MoveTo(width * 4, 10)).unwrap();
    for row in 0..next_piece.shape.len() {
        for col in 0..next_piece.shape[row].len() {
            if next_piece.shape[row][col] == 'a' {
                stdout.queue(Print("[]")).unwrap();
            } else {
                stdout.queue(Print("  ")).unwrap();
            }
        }
        stdout.queue(MoveTo(width * 4, (row + 11) as u16)).unwrap();
    }

    stdout.flush().unwrap();
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
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(MoveTo(11, 1)).unwrap(); // move cursor to top left while leaving space for hold
    for row in display.iter().enumerate() {
        stdout.queue(Print("<!")).unwrap(); // left wall
        for _ in row.1 {
            stdout.queue(Print("  ")).unwrap();
        }
        stdout.queue(Print("!>")).unwrap(); // right wall
        stdout.queue(MoveTo(11, (row.0 + 2) as u16)).unwrap();
    }
    stdout
        .queue(Print(format!(
            "<!{}!>\r\n",
            "=".repeat(display[0].len() * 2)
        )))
        .unwrap(); // bottom wall
    stdout
        .queue(Print(format!(
            "{}{}",
            " ".repeat(13),
            "\\/".repeat(display[0].len())
        )))
        .unwrap(); // bottom spikes
    stdout.queue(Hide).unwrap(); // Hide the cursor
    stdout.flush().unwrap();

    display
}

pub fn gravity(
    display: &mut Vec<Vec<char>>,
    active_piece: &mut Tetrominoe,
    next_piece: &mut Tetrominoe,
) -> bool {
    let prev_display = display.clone();
    for row in (0..display.len()).rev() {
        for col in 0..display[row].len() {
            if display[row][col] == 'a' {
                if row == display.len() - 1 || display[row + 1][col] == 'l' {
                    *display = prev_display;
                    landed(display);
                    let game_over = new_piece(display, active_piece, None, next_piece);
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

pub fn handle_input(
    display: &mut Vec<Vec<char>>,
    key: char,
    active_piece: &mut Tetrominoe,
    next_piece: &mut Tetrominoe,
) {
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
                gravity(display, active_piece, next_piece);
            }
        }

        'd' => {
            gravity(display, active_piece, next_piece);
        }

        'u' => {
            // let prev_display = display.clone();
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
            for row in 0..display.len() {
                for col in 0..display[row].len() {
                    if display[row][col] == 'a' {
                        display[row][col] = EMP;
                    }
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
                        display[row][col] =
                            active_piece.shape[row - active_piece.row][col - active_piece.col];
                    }
                }
            }
        }

        _ => (),
    }
}

pub fn new_piece(
    display: &mut Vec<Vec<char>>,
    active_piece: &mut Tetrominoe,
    desired_piece: Option<char>,
    next_piece: &mut Tetrominoe,
) -> bool {
    let half_width = display[0].len() / 2;

    // game over
    if display[0][half_width] != EMP {
        return true;
    }

    let piece = desired_piece.unwrap_or(get_next_piece(next_piece));
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
    active_piece.set_pos(0, (half_width - 1) as usize);
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
    let mut lines: usize = 0;
    'outer: for row in (0..display.len()).rev() {
        for ch in &display[row] {
            if *ch != 'l' {
                continue 'outer;
            }
        }
        display.remove(row);
        lines += 1;
    }

    for _ in 0..lines {
        display.insert(0, vec![EMP; display[0].len()]); // add new line at the top
    }

    match lines {
        1 => score.score += 40 * (score.level + 1),
        2 => score.score += 100 * (score.level + 1),
        3 => score.score += 300 * (score.level + 1),
        4 => score.score += 1200 * (score.level + 1),
        _ => (),
    }

    score.level = score.score / 1000;
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
    gravity(display, active_piece, &mut Tetrominoe::random());
    while display[0][display[0].len() / 2] == EMP {
        prev_display = display.clone();
        gravity(display, active_piece, &mut Tetrominoe::random());
    }
    *display = prev_display;
}

pub fn get_input() -> char {
    loop {
        if poll(Duration::from_millis(0)).unwrap() {
            let input = event::read().unwrap();
            match input {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'q', // quit
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 's', // hard drop
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'c', // hold
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'p', // pause
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'l', // move left
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'r', // move right
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'u', // rotate clockwise
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'd', // soft drop
                _ => (),
            }
        } else {
            return ' ';
        }
    }
}

pub fn hold(
    display: &mut Vec<Vec<char>>,
    active_piece: &mut Tetrominoe,
    hold_piece: &mut Option<Tetrominoe>,
    next_piece: &mut Tetrominoe,
) {
    // clear piece
    for row in display.iter_mut() {
        for col in row.iter_mut() {
            if *col == 'a' {
                *col = EMP;
            }
        }
    }

    // hold piece
    if let Some(hold) = hold_piece {
        let prev_piece = active_piece.clone();
        new_piece(display, active_piece, Some(hold.ptype), next_piece);
        *hold_piece = Some(prev_piece);
    } else {
        *hold_piece = Some(active_piece.clone());
        new_piece(display, active_piece, None, next_piece);
    }
}

fn get_next_piece(next_piece: &mut Tetrominoe) -> char {
    let temp = next_piece.ptype;
    *next_piece = Tetrominoe::random();
    temp
}

pub fn put_text(width: u16, height: u16, text: &str) {
    let mut stdout = stdout();
    let width = width*2-(text.len() as u16/4);
    stdout.queue(MoveTo(width, height/2)).unwrap();
    stdout
        .queue(SetForegroundColor(Color::Rgb {
            r: 255,
            g: 105,
            b: 97,
        }))
        .unwrap()
        .queue(Print(format!("{}", text)))
        .unwrap()
        .queue(ResetColor)
        .unwrap();
}
